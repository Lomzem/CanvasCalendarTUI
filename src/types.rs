use std::{collections::BTreeMap, fmt::Display};

use chrono::{DateTime, Local, NaiveDate};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Planner {
    pub course_id: usize,
    #[serde(rename = "context_name")]
    pub course_name: String,
    pub plannable_date: DateTime<Local>,
    pub plannable: Plannable,
    pub plannable_type: String,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plannable {
    pub id: usize,
    pub title: String,
}

#[derive(Debug)]
pub struct PlannerList {
    pub list: Vec<Planner>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct Calendar {
    pub planners: BTreeMap<NaiveDate, PlannerList>,
}

impl Planner {
    pub fn get_course_code(&self) -> String {
        // get first two splits of course name
        let course_name = self.course_name.split(' ').collect::<Vec<&str>>();
        if course_name.len() > 1 {
            return course_name[0..2].join(" ").to_string();
        } else {
            return "".to_string();
        }
    }
}

impl Display for &Planner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.get_course_code(), self.plannable.title)
    }
}
