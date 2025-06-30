use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};
use crate::app::App;
use crate::components::optionselector::{OptionSelector, OptionSelectorText};
use crate::mods::{Mod, ModList};
use crate::tui::Event;

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    selector: OptionSelector,
    mods: Vec<Mod>,
}

impl Home {
    pub fn new() -> Self {
        let mut selector = OptionSelector::default();
        selector.title = "Installed mods".to_string();

        let mut mods = ModList::new().get_local_mods();
        
        mods.sort_by(|a, b| {
            match b.enabled.cmp(&a.enabled) {
                std::cmp::Ordering::Equal => a.name.cmp(&b.name),
                o => o,
            }
        });
        
        mods.iter_mut().for_each(|m| {
            selector.options.push(
                vec![
                    OptionSelectorText::new(m.name.clone(), Style::default()),
                    OptionSelectorText::new(format!(" {} ", m.version.clone()), Style::default().fg(Color::LightBlue)),
                    OptionSelectorText::new(format!("by {}", m.author.clone().join(", ")), Style::default().fg(Color::DarkGray)),
                ]
//                Span::styled(format!("{} {} by {:?}", m.name, m.version, m.author), Style::default().fg(Color::Green)),
            );
            if !m.enabled.unwrap_or(true) {
                selector.options.last_mut().unwrap().push(OptionSelectorText::new(" (disabled)".to_string(), Style::default().fg(Color::Red)));
            }
        });
        
        Self {
            selector,
            mods,
            ..Default::default()
        }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            // KeyCode::Char('k') => {
            //     let mut ml = ModList::new();
            //     ml.clone_online_mod_list();
            // }
            _ => {
                let _ = self.selector.handle_key_event(key);
            }
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
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
            Paragraph::new("Balatro TUI")
                .style(Style::default())
                .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                ),
            chunks[0]
        );

        self.selector.draw(frame, chunks[1])?;

        // frame.render_widget(
        //     Paragraph::new("Bottom")
        //         .style(Style::default())
        //         .block(
        //             Block::default()
        //                 .borders(Borders::ALL)
        //                 .border_type(BorderType::Rounded)
        //         ),
        //     chunks[2]
        // );

        Ok(())
    }
}
