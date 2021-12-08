use jenkins_api::JenkinsBuilder;
use serde::Deserialize;

#[derive(Deserialize)]
struct JobHealth {}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LastBuild {
    number: u32,
    duration: u32,
    result: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LastBuildOfJob {
    display_name: String,
    last_build: LastBuild,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JenkinsJob {
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct JenkinsJobs {
    jobs: Vec<JenkinsJob>,
}

fn job_branches(jenkins: &jenkins_api::Jenkins, job: &str) -> Result<Vec<JenkinsJob>> {
    let j: JenkinsJobs = jenkins.get_object_as(
        jenkins_api::client::Path::Job {
            name: job,
            configuration: None,
        },
        jenkins_api::client::TreeBuilder::new()
            .with_field(
                jenkins_api::client::TreeBuilder::object("jobs")
                    .with_subfield("name")
                    .with_subfield("url"),
            )
            .build(),
    )?;
    Ok(j.jobs)
}

//
#[allow(dead_code)]
fn job_info(jenkins: &jenkins_api::Jenkins, job: &str) -> Result<LastBuildOfJob> {
    jenkins.get_object_as(
        jenkins_api::client::Path::Job {
            name: job,
            configuration: None,
        },
        jenkins_api::client::TreeBuilder::new()
            .with_field("displayName")
            .with_field(
                jenkins_api::client::TreeBuilder::object("lastBuild")
                    .with_subfield("number")
                    .with_subfield("duration")
                    .with_subfield("result"),
            )
            .build(),
    )
}

fn main() -> Result<()> {
    let jenkins = JenkinsBuilder::new(&std::env::var("JENKINS_URL")?).build()?;

    let jobs: JenkinsJobs = jenkins.get_object_as(
        jenkins_api::client::Path::Home,
        jenkins_api::client::TreeBuilder::object("jobs")
            .with_subfield("name")
            .with_subfield("url")
            .build(),
    )?;

    for job in jobs.jobs {
        println!("{}:", job.name);
        let branches = job_branches(&jenkins, &job.name)?;
        println!(
            "  Branches:\n    {}",
            branches
                .iter()
                .map(|b| b.name.clone())
                .collect::<Vec<String>>()
                .join("\n    ")
        );
    }

    Ok(())
}
