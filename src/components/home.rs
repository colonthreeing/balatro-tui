use std::fmt::Pointer;
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
enum Focused {
    #[default]
    Modes,
    InstalledMods,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    installed_mod_selector: OptionSelector,
    mode_selector: OptionSelector,
    mods: Vec<Mod>,
    focused: Focused
}

impl Home {
    pub fn new() -> Self {
        let mut installed_mod_selector = OptionSelector::default();
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

        let mut mode_selector = OptionSelector::default();

        mode_selector.has_focus = true;
        mode_selector.title = "Modes".to_string();

        mode_selector.options = vec![
            vec![OptionSelectorText::new("Installed Mods".to_string(), Style::default())],
            vec![OptionSelectorText::new("Find New Mods".to_string(), Style::default())],
            vec![OptionSelectorText::new("Mod Authoring Tools".to_string(), Style::default())],
        ];

        Self {
            installed_mod_selector,
            mode_selector,
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
                match self.focused {
                    Focused::Modes => {
                        match key.code {
                            KeyCode::Right => {
                                self.focused = Focused::InstalledMods;
                                self.installed_mod_selector.has_focus = true;
                                self.mode_selector.has_focus = false;
                            }
                            _ => {
                                let _ = self.mode_selector.handle_key_event(key);
                            }
                        }
                    }
                    Focused::InstalledMods => {
                        match key.code {
                            KeyCode::Left => {
                                self.focused = Focused::Modes;
                                self.installed_mod_selector.has_focus = false;
                                self.mode_selector.has_focus = true;
                            }
                            _ => {
                                let _ = self.installed_mod_selector.handle_key_event(key);
                            }
                        }
                    }
                }
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
        let vertical_chunks = Layout::default()
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
            vertical_chunks[0]
        );


        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(40),
                Constraint::Min(50)
            ])
            .split(vertical_chunks[1]);

        // frame.render_widget(
        //     Paragraph::new("")
        //         .style(Style::default())
        //         .block(
        //                 Block::default()
        //                     .borders(Borders::ALL)
        //                     .border_type(BorderType::Rounded)
        //                     .title("Modes")
        //         ),
        //     horizontal_chunks[0]
        // );

        self.mode_selector.draw(frame, horizontal_chunks[0])?;
        self.installed_mod_selector.draw(frame, horizontal_chunks[1])?;

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
