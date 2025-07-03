use std::cell::RefCell;
use std::fmt::Pointer;
use std::rc::Rc;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use log::{info, warn};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};
use super::Component;
use crate::{action::Action, config::Config};
use crate::app::App;
use crate::components::authoring::AuthoringTools;
use crate::components::modlist::ModlistComponent;
use crate::components::optionselector::{OptionSelector, OptionSelectorText};
use crate::components::quickoptions::QuickOptions;
use crate::mods::{Mod, ModList};
use crate::tui::Event;

#[derive(Default)]
enum Focused {
    #[default]
    Modes,
    InstalledMods,
    Authoring,
    Quicks,
}

pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    quick_ops: QuickOptions,
    installed_mod_selector: ModlistComponent,
    mode_selector: OptionSelector<Box<dyn Fn(u16)>>,
    focused: Focused,
    authoring: AuthoringTools,
    has_focus: bool
}

impl Home {
    pub fn new() -> Self {
        let installed_mod_selector = ModlistComponent::new();

        let mut mode_selector = OptionSelector::new(vec![
            vec![OptionSelectorText::new("Quick Options".to_string(), Style::default())],
            vec![OptionSelectorText::new("Installed Mods".to_string(), Style::default())],
            vec![OptionSelectorText::new("Find New Mods".to_string(), Style::default())],
            vec![OptionSelectorText::new("Mod Authoring Tools".to_string(), Style::default())],
        ]);

        mode_selector.has_focus = true;
        mode_selector.title = "Modes".to_string();
        
        let authoring = AuthoringTools::new();
        
        let mut quick_ops = QuickOptions::new();
        quick_ops.setup_callback();
        
        Self {
            installed_mod_selector,
            mode_selector,
            authoring,
            quick_ops,
            command_tx: None,
            config: Config::default(),
            focused: Focused::Modes,
            has_focus: false,
        }
    }
}

impl Component for Home {
    fn focus(&mut self) {
        self.has_focus = true;
    }
    fn unfocus(&mut self) {
        self.has_focus = false;
    }
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
            KeyCode::Char('e') => {
                info!("Hello!");
            }
            _ => {
                match self.focused {
                    Focused::Modes => {
                        match key.code {
                            KeyCode::Right => {
                                match self.mode_selector.selected {
                                    0 => {
                                        self.focused = Focused::Quicks;
                                        self.quick_ops.focus();
                                    }
                                    1 => { // installed mods
                                        self.focused = Focused::InstalledMods;
                                        self.installed_mod_selector.focus();
                                    }
                                    2 => {}
                                    3 => {
                                        self.focused = Focused::Authoring;
                                        self.authoring.focus();
                                    }
                                    _ => {}
                                }
                                self.mode_selector.has_focus = false;
                            }
                            _ => {
                                let _ = self.mode_selector.handle_key_event(key);
                            }
                        }
                    }
                    Focused::Quicks => {
                        match key.code {
                            KeyCode::Left => {
                                self.focused = Focused::Modes;
                                self.quick_ops.unfocus();
                                self.mode_selector.focus();
                            }
                            _ => {
                                let _ = self.quick_ops.handle_key_event(key);
                            }
                        }
                    }
                    Focused::InstalledMods => {
                        match key.code {
                            KeyCode::Left => {
                                self.focused = Focused::Modes;
                                self.installed_mod_selector.unfocus();
                                self.mode_selector.focus();
                            }
                            _ => {
                                let _ = self.installed_mod_selector.handle_key_event(key);
                            }
                        }
                    }
                    Focused::Authoring => {
                        match key.code {
                            KeyCode::Left => {
                                self.focused = Focused::Modes;
                                self.authoring.unfocus();
                                self.mode_selector.focus();
                            }
                            _ => {
                                let _ = self.authoring.handle_key_event(key);
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
                Constraint::Length(3),
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
        self.mode_selector.draw(frame, horizontal_chunks[0])?;
        
        match self.mode_selector.selected {
            0 => { // quick options
                self.quick_ops.draw(frame, horizontal_chunks[1])?;
            }
            1 => { // installed mods
                self.installed_mod_selector.draw(frame, horizontal_chunks[1])?;
            }
            2 => { // find mods
                
            }
            3 => { // mod tools
                self.authoring.draw(frame, horizontal_chunks[1])?;
            }
            _ => {}
        }
        
        frame.render_widget(
            TuiLoggerWidget::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Logs")
                )
                .output_level(None)
                .style_info(Style::default().fg(Color::LightGreen))
                .style_warn(Style::default().fg(Color::Yellow))
                .style_error(Style::default().fg(Color::Red))
                .style_debug(Style::default().fg(Color::Blue))
                .output_file(false)
                .output_target(false)
                .output_timestamp(None)
                .output_line(false),
            vertical_chunks[2]
        );

        Ok(())
    }
}