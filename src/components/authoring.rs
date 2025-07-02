use std::cmp::min;
use std::ops::Add;
use std::time::Instant;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::Paragraph,
};
use ratatui::layout::Direction;
use ratatui::style::{Color, Modifier};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders};
use tokio::sync::mpsc::UnboundedSender;
use super::Component;

use crate::action::Action;

#[derive(Debug, Clone, Default)]
pub struct AuthoringTools {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub has_focus: bool,
}

impl AuthoringTools {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for AuthoringTools {
    fn focus(&mut self) {
        self.has_focus = true;
    }
    fn unfocus(&mut self) {
        self.has_focus = false;
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up => {
            },
            KeyCode::Down => {
            },
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
            ])
            .split(area);
        frame.render_widget(
            Paragraph::new("Currently editing mod \'{}\'")
                .style(Style::default())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(
                            if self.has_focus {
                                Style::default().fg(Color::LightCyan)
                            } else {
                                Style::default().fg(Color::White)
                            }
                        )
                        .title("{}")
                ),
            chunks[0]
        );

        Ok(())
    }
}
