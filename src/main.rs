mod data_fetch;
mod menu;
mod types;

use crate::menu::Menu;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let calendar = data_fetch::get_calendar().await?;

    // for item in calendar.date_map.iter() {
    //     dbg!(&item.1.items);
    // }

    let mut terminal = ratatui::init();
    let menu_res = Menu::new(calendar).run(&mut terminal);
    ratatui::restore();
    menu_res?;

    Ok(())
}
