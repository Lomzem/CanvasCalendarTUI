use std::io;

use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, HighlightSpacing, List, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};

use crate::{
    data_fetch::get_base_url,
    types::{Calendar, DateItems},
};

// just invert the bg and fg
const SELECTED_STYLE: Style = Style::new().fg(Color::Black).bg(Color::White);

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
            .date_map
            .range(today..) // Start with dates greater than or equal to today
            .next()
        {
            closest_date
        } else if let Some((&closest_date, _)) = calendar
            .date_map
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

        let [current_date_space, list_space, instruction_space] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(6),
        ])
        .areas(block.inner(area));

        Paragraph::new(format!("Current Date: {}", self.current_date))
            .render(current_date_space, buf);

        block.render(area, buf);
        self.render_list(list_space, buf);
        self.render_instructions(instruction_space, buf)
    }
}

impl Menu {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<_> = self
            .calendar
            .date_map
            .get(&self.current_date)
            .unwrap_or(&DateItems {
                ..Default::default()
            })
            .items
            .iter()
            // .map(|planner_item| Line::from(planner_item.to_string()))
            .map(|planner_item| {
                if planner_item.submissions.is_none()
                    || planner_item
                        .submissions
                        .as_ref()
                        .is_some_and(|s| !s.submitted)
                {
                    Line::from(planner_item.to_string())
                } else {
                    Line::from(vec![
                        Span::from(planner_item.to_string()),
                        Span::from(" ✔").green(),
                    ])
                }
            })
            .collect();

        if !items.is_empty() {
            let state = &mut self
                .calendar
                .date_map
                .get_mut(&self.current_date)
                .expect("Struct must have state")
                .state;
            if state.selected().is_none() {
                state.select(Some(0));
            }
        }

        let list = List::new(items)
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(
            list,
            area,
            buf,
            &mut self
                .calendar
                .date_map
                .get_mut(&self.current_date)
                .unwrap()
                .state,
        );
    }

    fn render_instructions(&mut self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("<h>", Style::new().blue()),
                Span::raw(" Back"),
            ]),
            Line::from(vec![
                Span::styled("<l>", Style::new().blue()),
                Span::raw(" Forward"),
            ]),
            Line::from(vec![
                Span::styled("<j>", Style::new().blue()),
                Span::raw(" Down"),
            ]),
            Line::from(vec![
                Span::styled("<k>", Style::new().blue()),
                Span::raw(" Up"),
            ]),
            Line::from(vec![
                Span::styled("<o>", Style::new().blue()),
                Span::raw(" Open URL"),
            ]),
            Line::from(vec![
                Span::styled("<q>", Style::new().blue()),
                Span::raw(" Quit"),
            ]),
        ]);
        paragraph.render(area, buf);
    }
}

impl Menu {
    fn next_date(&mut self) {
        // go to next date with planner item
        let next = self
            .calendar
            .date_map
            .range(self.current_date.succ_opt().expect("Cant get next date")..)
            .next();
        if let Some((date, _)) = next {
            self.current_date = *date;
        }
    }

    fn prev_date(&mut self) {
        let prev = self
            .calendar
            .date_map
            .range(..self.current_date.pred_opt().expect("Cant get prev date"))
            .rev()
            .next();
        if let Some((date, _)) = prev {
            self.current_date = *date;
        }
    }

    fn down(&mut self) {
        if let Some(planner_list) = self.calendar.date_map.get_mut(&self.current_date) {
            planner_list.state.select_next();
        }
    }

    fn up(&mut self) {
        if let Some(planner_list) = self.calendar.date_map.get_mut(&self.current_date) {
            planner_list.state.select_previous();
        }
    }

    fn open_url(&self) {
        if let Some(planner_list) = self.calendar.date_map.get(&self.current_date) {
            if let Some(planner) = planner_list
                .items
                .get(planner_list.state.selected().unwrap())
            {
                let url = get_base_url()
                    .join(&planner.html_url)
                    .expect("Unable to join url");
                webbrowser::open(&url.as_str()).expect("Unable to open url");
            }
        }
    }
}
