use std::io;

use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};

use crate::{data_fetch::get_base_url, types::{Calendar, PlannerList}};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug)]
pub struct Menu {
    should_quit: bool,
    current_date: NaiveDate,
    calendar: Calendar,
}

impl Menu {
    pub fn new(calendar: Calendar) -> Self {
        let today = chrono::Local::now().naive_local().date();

        let current_date = if let Some((&closest_date, _)) = calendar
            .planners
            .range(today..) // Start with dates greater than or equal to today
            .next()
        {
            closest_date
        } else if let Some((&closest_date, _)) = calendar
            .planners
            .range(..today) // If no dates >= today, look for the latest date before today
            .next_back()
        {
            closest_date
        } else {
            today
        };

        Self {
            should_quit: false,
            current_date,
            calendar,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
        while !self.should_quit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), io::Error> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('h') => {
                self.prev_date();
            }
            KeyCode::Char('l') => {
                self.next_date();
            }
            KeyCode::Char('j') => self.down(),
            KeyCode::Char('k') => self.up(),
            KeyCode::Char('o') => self.open_url(),
            _ => {}
        }
    }
}

impl Widget for &mut Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Canvas Calendar TUI ".bold());
        let block = Block::bordered().title(title.centered());

        let [current_date, list] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(block.inner(area));

        Paragraph::new(format!("Current Date: {}", self.current_date)).render(current_date, buf);

        block.render(area, buf);
        self.render_list(list, buf);
    }
}

impl Menu {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<_> = self
            .calendar
            .planners
            .get(&self.current_date)
            .unwrap_or(&PlannerList {
                list: Vec::new(),
                state: ListState::default(),
            })
            .list
            .iter()
            .map(|planner_item| Line::from(planner_item.to_string()))
            .collect();

        if !items.is_empty() {
            let state = &mut self
                .calendar
                .planners
                .get_mut(&self.current_date)
                .unwrap()
                .state;
            if state.selected().is_none() {
                state.select(Some(0));
            }
        }

        let list = List::new(items)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(
            list,
            area,
            buf,
            &mut self
                .calendar
                .planners
                .get_mut(&self.current_date)
                .unwrap()
                .state,
        );
    }
}

impl Menu {
    fn next_date(&mut self) {
        // go to next date with planner item
        let next = self
            .calendar
            .planners
            .range(self.current_date.succ_opt().expect("Cant get next date")..)
            .next();
        if let Some((date, _)) = next {
            self.current_date = *date;
        }
    }

    fn prev_date(&mut self) {
        let prev = self
            .calendar
            .planners
            .range(..self.current_date.pred_opt().expect("Cant get prev date"))
            .next_back();
        if let Some((date, _)) = prev {
            self.current_date = *date;
        }
    }

    fn down(&mut self) {
        if let Some(planner_list) = self.calendar.planners.get_mut(&self.current_date) {
            planner_list.state.select_next();
        }
    }

    fn up(&mut self) {
        if let Some(planner_list) = self.calendar.planners.get_mut(&self.current_date) {
            planner_list.state.select_previous();
        }
    }

    fn open_url(&self) {
        if let Some(planner_list) = self.calendar.planners.get(&self.current_date) {
            if let Some(planner) = planner_list.list.get(planner_list.state.selected().unwrap()) {
                let url = get_base_url().join(&planner.html_url).expect("Unable to join url");
                webbrowser::open(&url.as_str()).expect("Unable to open url");
            }
        }
    }
}
