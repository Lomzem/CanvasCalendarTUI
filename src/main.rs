mod data_fetch;
mod menu;
mod types;

use menu::Menu;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let calendar = data_fetch::get_calendar().await?;
    // serde_json::to_writer_pretty(std::fs::File::create("planner.json")?, &calendar)?;
    // let calendar = serde_json::from_reader(std::fs::File::open("planner.json")?)?;

    // let planner = vec![
    //     types::Planner {
    //         course_id: 1,
    //         course_name: "Course 1".to_string(),
    //         plannable_date: chrono::Local::now(),
    //         plannable: types::Plannable {
    //             id: 1,
    //             title: "Assignment 1".to_string(),
    //         },
    //         plannable_type: "Assignment".to_string(),
    //         html_url: "https://canvas.example.com/courses/1/assignments/1".to_string(),
    //     },
    //     types::Planner {
    //         course_id: 1,
    //         course_name: "Course 1".to_string(),
    //         plannable_date: chrono::Local::now(),
    //             id: 2,
    //         plannable: types::Plannable {
    //             title: "Assignment 2".to_string(),
    //         },
    //         plannable_type: "Assignment".to_string(),
    //         html_url: "https://canvas.example.com/courses/1/assignments/2".to_string(),
    //     },
    // ];
    // let planners: std::collections::BTreeMap<chrono::NaiveDate,types::PlannerList>  = planner
    //     .into_iter()
    //     .fold(std::collections::BTreeMap::new(), |mut acc, planner| {
    //         let date = planner.plannable_date.date_naive();
    //         acc.entry(date)
    //             .or_insert_with(|| types::PlannerList {
    //                 list: Vec::new(),
    //                 state: ratatui::widgets::ListState::default(),
    //             })
    //             .list
    //             .push(planner);
    //         acc
    //     });
    // let calendar = crate::types::Calendar {
    //     planners,
    // };

    let mut terminal = ratatui::init();
    let menu_res = Menu::new(calendar).run(&mut terminal);
    ratatui::restore();
    menu_res?;

    Ok(())
}
