use chrono::{Duration, TimeZone, Utc};

use jenkins_api::{
    build::{BuildStatus, CommonBuild},
    client::{Path, TreeBuilder},
    Jenkins, JenkinsBuilder,
};
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
struct BuildInfo {
    #[allow(dead_code)] // We currently don't use the job_name
    branch_name: String,
    last_build: Option<CommonBuild>,
}

#[derive(Deserialize, Clone)]
struct JenkinsJob {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct JenkinsJobs {
    jobs: Vec<JenkinsJob>,
}

fn build_info(jenkins: &Jenkins, branch: &JenkinsJob) -> Result<BuildInfo> {
    let job = jenkins.get_job(&branch.name)?;
    let last_build = match job.last_build {
        Some(b) => Some(b.get_full_build(jenkins)?),
        None => None,
    };
    Ok(BuildInfo {
        branch_name: job.name,
        last_build,
    })
}

fn get_jobs(jenkins: &Jenkins) -> Result<Vec<JenkinsJob>> {
    let jobs: JenkinsJobs = jenkins.get_object_as(
        Path::Home,
        TreeBuilder::object("jobs")
            .with_subfield("name")
            .with_subfield("url")
            .build(),
    )?;
    Ok(jobs.jobs)
}

fn format_timestamp(milliseconds: u64) -> String {
    Utc.timestamp_millis(milliseconds as i64).to_string()
}

fn report_on_job(
    job: JenkinsJob,
    branches: Vec<JenkinsJob>,
    failed: Vec<CommonBuild>,
    _unbuilt: Vec<BuildInfo>,
    last_success: Option<CommonBuild>,
) {
    println!("{}", job.name);

    if branches.len() >= 10 {
        println!("  Warning: {} branches", branches.len());
    }

    if !failed.is_empty() {
        println!("  Warning: {} failed branches", failed.len())
    }

    match last_success {
        None => println!("  Warning: Last main branch build failed."),

        // Warn if last successful build is older than ~6 months
        Some(i)
            if (i.timestamp as i64)
                < Utc::now()
                    .checked_sub_signed(Duration::days(30 * 6))
                    .unwrap()
                    .timestamp_millis() =>
        {
            println!(
                "  Warning: Old last successful build {}",
                format_timestamp(i.timestamp),
            )
        }
        _ => {}
    }
}

fn main() -> Result<()> {
    let jenkins = JenkinsBuilder::new(&std::env::var("JENKINS_URL")?).build()?;

    let jobs = get_jobs(&jenkins)?;

    for job in jobs {
        // We want the branch at $JENKINS_URL/job/<job_name>/job/<branch_name>,
        // but get_object_as does not support getting sub-objects (using name: <job_name>/job/<branch_name> sanitises the / to %2F which does not work).
        // So we need to build a new root for getting branches
        let job_jenkins = JenkinsBuilder::new(&job.url).build()?;

        let branches = get_jobs(&job_jenkins)?;

        let mut unbuilt = vec![];
        let mut failed = vec![];
        let mut last_success = None;

        for branch in &branches {
            let build_info = build_info(&job_jenkins, branch)?;

            match build_info.last_build {
                None => {
                    unbuilt.push(build_info);
                }
                Some(ref info) => match info.result {
                    Some(BuildStatus::Failure) | Some(BuildStatus::Aborted) => {
                        failed.push(info.clone());
                    }
                    Some(BuildStatus::NotBuilt) => {
                        unbuilt.push(build_info.clone());
                    }
                    Some(BuildStatus::Success) => {
                        if branch.name == "main" || branch.name == "master" {
                            last_success = match last_success {
                                Some(last) => {
                                    Some(std::cmp::max_by_key(last, info.clone(), |t| t.timestamp))
                                }
                                None => Some(info.clone()),
                            };
                        }
                    }
                    None | Some(BuildStatus::Unstable) => {}
                },
            };
        }
        report_on_job(job, branches, failed, unbuilt, last_success);
    }

    Ok(())
}
