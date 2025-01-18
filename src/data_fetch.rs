use crate::types::{Calendar, Planner, PlannerList};
use chrono::{Local, NaiveDate};
use ratatui::widgets::ListState;
use std::collections::BTreeMap;
use std::env;
use std::process::exit;
use url::Url;

const TOKEN_ENV_VAR: &str = "CANVAS_ACCESS_TOKEN";
const URL_ENV_VAR: &str = "CANVAS_URL";
const API_ENDPOINT: &str = "/api/v1/planner/items";

pub fn get_base_url() -> Url {
    match env::var(&URL_ENV_VAR) {
        Ok(url) => match Url::parse(&url) {
            Ok(url) => url,
            Err(_) => {
                eprintln!(
                    "Invalid URL: {} from environment variable {}",
                    &url, &URL_ENV_VAR
                );
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("Please set the {} environment variable", &URL_ENV_VAR);
            exit(1);
        }
    }
}

async fn get_planner() -> Result<Vec<Planner>, reqwest::Error> {
    let access_token = match env::var(&TOKEN_ENV_VAR) {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Please set the {} environment variable", &TOKEN_ENV_VAR);
            exit(1);
        }
    };

    let full_url = Url::parse_with_params(
        get_base_url()
            .join(API_ENDPOINT)
            .expect("Unable to join base url")
            .as_str(),
        &[
            ("access_token", access_token),
            ("start_date", Local::now().format("%Y-%m-%d").to_string()),
        ],
    )
    .expect("Unable to add \"access_token\" to url");

    let planners: Vec<Planner> = reqwest::get(full_url).await?.json().await?;

    Ok(planners)
}

pub async fn get_calendar() -> Result<Calendar, reqwest::Error> {
    let planners = get_planner().await?;
    let mut planner_map: BTreeMap<NaiveDate, PlannerList> = BTreeMap::new();

    for planner in planners {
        let date = planner.plannable_date.date_naive();
        planner_map
            .entry(date)
            .or_insert(PlannerList {
                list: Vec::new(),
                state: ListState::default(),
            })
            .list
            .push(planner);
    }

    Ok(Calendar { planners: planner_map })
}
