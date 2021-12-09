use jenkins_api::{
    build::CommonBuild,
    client::{Path, TreeBuilder},
    Jenkins, JenkinsBuilder,
};
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildInfo {
    #[allow(dead_code)] // We currently don't use the job_name
    job_name: String,
    build: Option<CommonBuild>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JenkinsJob {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct JenkinsJobs {
    jobs: Vec<JenkinsJob>,
}

fn job_branches(jenkins: &Jenkins, job: &str) -> Result<Vec<JenkinsJob>> {
    let jobs: JenkinsJobs = jenkins.get_object_as(
        Path::Job {
            name: job,
            configuration: None,
        },
        TreeBuilder::new()
            .with_field(
                TreeBuilder::object("jobs")
                    .with_subfield("name")
                    .with_subfield("url"),
            )
            .build(),
    )?;
    Ok(jobs.jobs)
}

fn build_info(job: &JenkinsJob, branch: &str) -> Result<BuildInfo> {
    // We want the branch at $JENKINS_URL/job/<job_name>/job/<branch_name>,
    // but get_object_as does not support getting sub-objects (using name: <job_name>/job/<branch_name> sanitises the / to %2F which does not work).
    // So we need to build a new root
    let jenkins = JenkinsBuilder::new(&job.url).build()?;
    let job = jenkins.get_job(branch)?;
    let build_info = match job.last_build {
        Some(b) => {
            let info = b.get_full_build(&jenkins)?;
            Some(info)
        }
        None => None,
    };
    Ok(BuildInfo {
        job_name: job.name,
        build: build_info,
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
    use chrono::{TimeZone, Utc};
    Utc.timestamp_millis(milliseconds as i64).to_string()
}

fn main() -> Result<()> {
    let jenkins = JenkinsBuilder::new(&std::env::var("JENKINS_URL")?).build()?;

    let jobs = get_jobs(&jenkins)?;

    for job in jobs {
        println!("{}", job.name);
        let branches = job_branches(&jenkins, &job.name)?;
        println!("  Branches",);
        for branch in branches {
            println!("    {}", branch.name);
            let info = build_info(&job, &branch.name)?;
            match info.build {
                None => println!("      No build info"),
                Some(info) => println!(
                    "      Last build: {}, {}",
                    info.result
                        .map(|b| format!("{:?}", b))
                        .unwrap_or_else(|| "In progress".to_string()),
                    format_timestamp(info.timestamp)
                ),
            }
        }
    }

    Ok(())
}
