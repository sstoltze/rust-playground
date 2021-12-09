use jenkins_api::{
    build::CommonBuild,
    client::{Path, TreeBuilder},
    Jenkins, JenkinsBuilder,
};
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct BuildInfo {
    #[allow(dead_code)] // We currently don't use the job_name
    job_name: String,
    last_build: Option<CommonBuild>,
}

#[derive(Deserialize)]
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
        job_name: job.name,
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
    use chrono::{TimeZone, Utc};
    Utc.timestamp_millis(milliseconds as i64).to_string()
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
        println!("{} - {} branches.", job.name, branches.len());

        for branch in branches {
            println!("  {}", branch.name);

            let info = build_info(&job_jenkins, &branch)?;
            let build_info = match info.last_build {
                None => "No build info".to_string(),
                Some(info) => format!(
                    "Last build: {}, {}",
                    info.result
                        .map(|b| format!("{:?}", b))
                        .unwrap_or_else(|| "In progress".to_string()),
                    format_timestamp(info.timestamp)
                ),
            };
            println!("    {}", build_info);
        }
    }

    Ok(())
}
