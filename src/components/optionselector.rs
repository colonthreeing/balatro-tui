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
use ratatui::style::{Color, Modifier};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders};
use tokio::sync::mpsc::UnboundedSender;
use super::Component;

use crate::action::Action;

#[derive(Debug, Clone)]
pub struct OptionSelectorText {
    pub text: String,
    pub style: Style,
}

impl OptionSelectorText {
    pub fn new(text: String, style: Style) -> Self {
        Self {
            text,
            style
        }
    }
}

#[derive(Default)]
pub struct OptionSelector<CB> {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub options: Vec<Vec<OptionSelectorText>>,
    pub selected: u16,
    pub title: String,
    pub has_focus: bool,
    
    pub callback: Option<CB>,
    offset: u16,
}

impl<CB> Clone for OptionSelector<CB>
where CB: FnMut(u16)
{
    fn clone(&self) -> Self {
        let mut s = Self::new(self.options.clone());
        s.selected = self.selected;
        s.title = self.title.clone();
        s.has_focus = self.has_focus;
        s.action_tx = self.action_tx.clone();
        s.options = self.options.clone();
        s.offset = self.offset;
        
        s
    }
}

impl<CB> OptionSelector<CB>
where CB: FnMut(u16)
{
    pub fn new(ops: Vec<Vec<OptionSelectorText>>) -> Self {
        Self {
            options: ops,
            selected: 0,
            action_tx: None,
            title: String::new(),
            has_focus: false,
            callback: None,
            offset: 0,
        }
    }
    
    pub fn set_callback(&mut self, cb: CB) {
        self.callback = Some(cb);
    }
    
    pub fn process_callback(&mut self) {
        if let Some(mut cb) = self.callback.take() {
            cb(self.selected);
        }
    }
}

impl<CB> Component for OptionSelector<CB>
where CB: FnMut(u16)
{
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up => {
                self.selected = self.selected.saturating_sub(1);
                if self.selected < self.options.len() as u16 {
                    self.offset = self.selected.saturating_sub(5);
                }
            },
            KeyCode::Down => {
                self.selected = min(self.selected.saturating_add(1), (self.options.len().saturating_sub(1)) as u16);
                if self.selected > 5 {
                    self.offset = self.selected.saturating_sub(5);
                }
            },
            KeyCode::Enter => {
                self.process_callback();
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {},
            Action::Render => {},
            _ => {}
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut op_i = 0;
        let ops: Vec<Line> = self.options.clone().into_iter()
            .map(|str| {
                op_i += 1;
                let mut lines = vec![];
                /*
                if op_i == self.selected + 1 {
                    lines.push(Span::styled(str[0].text, str[0].style.fg(Color::Green)));
                } else {
                    lines.push(Span::styled(str[0].text, str[0].style));
                }
                */
                
                for s in str {
                    lines.push(Span::styled(s.text, s.style));
                }
                
                if lines.len() >= 1 && op_i == self.selected + 1 {
                    lines[0] = lines[0].clone().style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
                }
                
                Line::from(lines)
            })
            .collect();
        let content = Paragraph::new(ops)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(Span::from(&self.title))
                    .border_style(if self.has_focus { Style::default().fg(Color::LightCyan) } else { Style::default().fg(Color::White) })
            )
            .scroll((self.offset, 0));

        frame.render_widget(content, area);

        Ok(())
    }

    fn focus(&mut self) {
        self.has_focus = true;
    }

    fn unfocus(&mut self) {
        self.has_focus = false;
    }
}
