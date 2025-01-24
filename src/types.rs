use std::{collections::BTreeMap, fmt::Display};

use chrono::{DateTime, Local, NaiveDate};
use ratatui::widgets::ListState;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

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
    #[serde(deserialize_with = "deserialize_submission")]
    pub submissions: Option<Submission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub submitted: bool,
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

fn deserialize_submission<'de, D>(deserializer: D) -> Result<Option<Submission>, D::Error>
where
    D: Deserializer<'de>,
{
    struct SubmissionVisitor;

    impl<'de> Visitor<'de> for SubmissionVisitor {
        type Value = Option<Submission>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a bool or a object representing a submission")
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let submission: Submission =
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(submission))
        }
    }

    deserializer.deserialize_any(SubmissionVisitor)
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

// #[test]
// fn test_get_course_code() {
//     let item = CalendarItem {
//         course_name: "CSCI 440 - 01 Operating Systems Spring 2025".to_string(),
//         datetime: DateTime::default(),
//         course_id: 1,
//         info: CalendarItemInfo {
//             id: 1,
//             title: "dummy".to_string(),
//         },
//         html_url: "https://www.canvas.net".to_string(),
//     };
//
//     assert_eq!(item.get_course_code(), "CSCI 440");
// }
