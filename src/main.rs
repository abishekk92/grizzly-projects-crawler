use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    data: Vec<Project>,
    total_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Project {
    slug: String,
    name: String,
    repo_url: Option<String>,
    presentation_url: Option<String>,
    project_image_id: Option<String>,
    banned: bool,
    reviewed: bool,
    seen: i8,
    hackathon_name: String,
    prize_tracks: Vec<Track>,
    sponsored_prizes: Vec<String>,
    image_url: Option<String>,
    project_image_content_type: Option<String>,
    description: Option<String>,
    additional_information: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Track {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hackathon_name = "grizzlython";

    let mut wtr = csv::Writer::from_path("projects.csv")?;

    wtr.write_record(&[
        "slug",
        "name",
        "repo_url",
        "presentation_url",
        "project_image_id",
        "banned",
        "reviewed",
        "seen",
        "hackathon_name",
        "prize_tracks",
        "sponsored_prizes",
        "image_url",
        "project_image_content_type",
        "description",
        "additional_information",
    ])?;

    let total_pages = 35;

    for page in 0..=total_pages {
        let api_url = format!(
            "https://solana.com/api/hackathon/projects?page={}&hackathonName={}",
            page, hackathon_name
        );
        println!("Fetching page {} of {}", page, total_pages);
        let response: Response = reqwest::get(&api_url).await?.json().await?;

        let projects = response.data;

        for project in projects {
            // Convert the Vec<Track> to a comma-separated list of track names
            let prize_tracks = project
                .prize_tracks
                .iter()
                .map(|track| track.name.clone())
                .collect::<Vec<String>>()
                .join(",");

            // Convert the Vec<String> to a comma-separated list of strings
            let sponsored_prizes = project.sponsored_prizes.join(",");

            let record = [
                project.slug,
                project.name,
                project.repo_url.unwrap_or_default(),
                project.presentation_url.unwrap_or_default(),
                project.project_image_id.unwrap_or_default(),
                project.banned.to_string(),
                project.reviewed.to_string(),
                project.seen.to_string(),
                project.hackathon_name,
                prize_tracks,
                sponsored_prizes,
                project.image_url.unwrap_or_default(),
                project.project_image_content_type.unwrap_or_default(),
                project.description.unwrap_or_default(),
                project.additional_information.unwrap_or_default(),
            ];

            wtr.serialize(&record)?;
        }
        println!("Sleeping for 1 second to avoid overloading the API");
        sleep(Duration::from_secs(1)).await; // Sleep for 1 second to avoid overwhelming the server
    }

    wtr.flush()?;

    println!("Projects written to projects.csv");

    Ok(())
}
