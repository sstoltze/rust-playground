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

fn report_on_job(job: JenkinsJob, last_success: Option<CommonBuild>) {
    println!("{}", job.name);

    match last_success {
        None => println!("  Warning: Last main branch build failed."),

        Some(i)
            if (i.timestamp as i64)
                > Utc::now()
                    .checked_sub_signed(Duration::days(366 + 18))
                    .unwrap()
                    .timestamp_millis() =>
        {
            println!("  Built in 2021: {}", format_timestamp(i.timestamp),)
        }
        _ => println!("  Not built in 2021."),
    }
}

const BRANCHES: [&str; 2] = ["main", "master"];

fn main() -> Result<()> {
    let jenkins = JenkinsBuilder::new(&std::env::var("JENKINS_URL")?).build()?;

    let jobs = get_jobs(&jenkins)?;

    for job in jobs {
        // We want the branch at $JENKINS_URL/job/<job_name>/job/<branch_name>,
        // but get_object_as does not support getting sub-objects (using name: <job_name>/job/<branch_name> sanitises the / to %2F which does not work).
        // So we need to build a new root for getting branches
        let job_jenkins = JenkinsBuilder::new(&job.url).build()?;

        let mut last_success = None;

        for branch_name in &BRANCHES {
            let branch = JenkinsJob {
                name: branch_name.to_string(),
                url: "".to_string(),
            };
            if let Ok(build_info) = build_info(&job_jenkins, &branch) {
                match build_info.last_build {
                    None => {}
                    Some(ref info) => match info.result {
                        Some(BuildStatus::Success) => {
                            last_success = match last_success {
                                Some(last) => {
                                    Some(std::cmp::max_by_key(last, info.clone(), |t| t.timestamp))
                                }
                                None => Some(info.clone()),
                            };
                        }
                        _ => {}
                    },
                };
            }
        }
        report_on_job(job, last_success);
    }

    Ok(())
}
