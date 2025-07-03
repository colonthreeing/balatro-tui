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
use crate::components::optionselector::{OptionSelector, OptionSelectorText};
use crate::mods::ModList;

#[derive(Clone)]
pub struct ModlistComponent {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub has_focus: bool,
    options: OptionSelector<Box<dyn FnMut(u16)>>,
}

impl ModlistComponent {
    pub fn new() -> Self {
        let mut installed_mod_selector = OptionSelector::new(vec![]);
        installed_mod_selector.title = "Installed mods".to_string();

        let mut mods = ModList::new().get_local_mods();

        mods.sort_by(|a, b| {
            match b.enabled.cmp(&a.enabled) {
                std::cmp::Ordering::Equal => a.name.cmp(&b.name),
                o => o,
            }
        });

        mods.iter_mut().for_each(|m| {
            installed_mod_selector.options.push(
                vec![
                    OptionSelectorText::new(m.name.clone(), Style::default()),
                    OptionSelectorText::new(format!(" {} ", m.version.clone()), Style::default().fg(Color::LightBlue)),
                    OptionSelectorText::new(format!("by {}", m.author.clone().join(", ")), Style::default().fg(Color::DarkGray)),
                ]
                //                Span::styled(format!("{} {} by {:?}", m.name, m.version, m.author), Style::default().fg(Color::Green)),
            );
            if !m.enabled.unwrap_or(true) {
                installed_mod_selector.options.last_mut().unwrap().push(OptionSelectorText::new(" (disabled)".to_string(), Style::default().fg(Color::Red)));
            }
        });
        
        Self {
            action_tx: None,
            has_focus: false,
            options: installed_mod_selector,
        }
    }
}

impl Component for ModlistComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            _ => {
                self.options.handle_key_event(key)?;
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.options.draw(frame, area).expect("Options failed to draw!");
        Ok(())
    }

    fn focus(&mut self) {
        self.has_focus = true;
        self.options.focus();
    }

    fn unfocus(&mut self) {
        self.has_focus = false;
        self.options.unfocus();
    }
}
