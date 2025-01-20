use crate::types::{Calendar, CalendarItem, DateItems};
use chrono::{Local, NaiveDate};
use reqwest::StatusCode;
use std::collections::BTreeMap;
use std::env;
use std::process::exit;
use url::Url;

const TOKEN_ENV_VAR: &str = "CANVAS_ACCESS_TOKEN";
const URL_ENV_VAR: &str = "CANVAS_URL";
const API_ENDPOINT: &str = "/api/v1/planner/items";

pub fn get_base_url() -> Url {
    let url = env::var(&URL_ENV_VAR).unwrap_or_else(|_| {
        eprintln!("Please set the {} environment variable", &URL_ENV_VAR);
        exit(1);
    });
    let url = Url::parse(&url).unwrap_or_else(|_| {
        eprintln!(
            "Unable to parse URL: \"{}\" from environment variable {}",
            &url, &URL_ENV_VAR
        );
        exit(1);
    });
    url
}

pub async fn get_calendar() -> Result<Calendar, reqwest::Error> {
    let access_token = env::var(&TOKEN_ENV_VAR).unwrap_or_else(|_| {
        eprintln!("Please set the {} environment variable", &TOKEN_ENV_VAR);
        exit(1);
    });

    let full_url = Url::parse_with_params(
        get_base_url()
            .join(API_ENDPOINT)
            .expect("Unable to join parsed environment url with API endpoint")
            .as_str(),
        &[
            ("access_token", access_token),
            ("start_date", Local::now().format("%Y-%m-%d").to_string()),
        ],
    )
    .expect("Unable to join parsed environment url with query params");

    let response = reqwest::get(full_url).await.unwrap_or_else(|err| {
        if err.is_request() {
            eprintln!(
                "Error when sending request to url: \"{}\" Please check that {} is correct",
                get_base_url(),
                &URL_ENV_VAR
            );
        } else {
            eprintln!("Unknown GET request error: {}", &err);
        }
        exit(1);
    });

    match response.status() {
        StatusCode::OK => {}
        StatusCode::UNAUTHORIZED => {
            eprintln!("Invalid canvas access token. Please check that the token you provided in {} is still valid", &TOKEN_ENV_VAR);
            exit(1);
        }
        StatusCode::NOT_FOUND => {
            eprintln!(
                "Invalid url: \"{}\" Please check that {} is correct",
                get_base_url(),
                &URL_ENV_VAR
            );
            exit(1);
        }
        _ => {
            panic!("Unexpected status code after sending GET request!")
        }
    }

    let calendar_items: Vec<CalendarItem> = response
        .json()
        .await
        .expect("Canvas API response in unexpected format");

    let mut date_map: BTreeMap<NaiveDate, DateItems> = BTreeMap::new();

    for item in calendar_items {
        let date = item.datetime.date_naive();
        date_map.entry(date).or_default().items.push(item);
    }

    Ok(Calendar { date_map })
}
