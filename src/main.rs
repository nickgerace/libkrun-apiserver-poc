use anyhow::{anyhow, Result};
use chrono::offset::Utc;
use log::{debug, error, info};
use serde_derive::{Deserialize, Serialize};
use std::env;
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Input {
    args: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Output {
    args: Vec<String>,
    timestamp: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::builder().format_module_path(false).init();

    // Logic from: https://github.com/containers/krunvm/blob/8027270987eb5ae3f90c920dd71bb352754d3b70/src/main.rs#L144-L154
    if users::get_current_uid() != 0 && env::var("BUILDAH_ISOLATION").is_err() {
        return Err(anyhow!("Must be launched inside a buildah unshare session"));
    }

    match env::var("VALIDATE") {
        Ok(o) if o.to_lowercase() == "true" => libkrun_apiserver::validate()?,
        _ => debug!("Skipping extra validation step"),
    }

    // Set a content length limit of 16 KB.
    let arguments_route = warp::post()
        .and(warp::path("arguments"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|config: Input| {
            match libkrun_apiserver::krunvm(&config.args) {
                Ok(o) => match o {
                    Some(s) => info!("Exited with status code: {}", s),
                    None => info!("Status code unknown"),
                },
                Err(e) => error!("{}", e),
            }
            warp::reply::json(&Output {
                args: config.args,
                timestamp: format!("{}", Utc::now()),
            })
        });

    loop {
        warp::serve(arguments_route)
            .run(([127, 0, 0, 1], 3030))
            .await;
        info!("Restarting server...")
    }
}
