use std::{collections::BTreeMap, fmt::Display};

use chrono::{DateTime, Local, NaiveDate};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarItem {
    pub course_id: usize,
    #[serde(rename = "context_name")]
    pub course_name: String,
    #[serde(rename = "plannable_date")]
    pub datetime: DateTime<Local>,
    #[serde(rename = "plannable")]
    pub info: CalendarItemInfo,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarItemInfo {
    pub id: usize,
    pub title: String,
}

#[derive(Debug)]
pub struct Calendar {
    pub date_map: BTreeMap<NaiveDate, DateItems>,
}

#[derive(Debug, Default)]
pub struct DateItems {
    pub items: Vec<CalendarItem>,
    pub state: ListState,
}

impl CalendarItem {
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

impl Display for &CalendarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.get_course_code(), self.info.title)
    }
}

#[test]
fn test_get_course_code() {
    let item = CalendarItem {
        course_name: "CSCI 440 - 01 Operating Systems Spring 2025".to_string(),
        datetime: DateTime::default(),
        course_id: 1,
        info: CalendarItemInfo {
            id: 1,
            title: "dummy".to_string(),
        },
        html_url: "https://www.canvas.net".to_string(),
    };

    assert_eq!(item.get_course_code(), "CSCI 440");
}
