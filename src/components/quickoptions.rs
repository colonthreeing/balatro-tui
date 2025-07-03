use std::io::{BufReader, Error};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Rect, Size};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use std::process::{Command, Stdio};
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;
use balatro_tui::{get_balatro_appdata_dir, get_balatro_dir, launch_balatro, open};
use crate::action::Action;
use crate::components::Component;
use crate::components::optionselector::{Actions, OptionSelector, OptionSelectorText};
use crate::config::{get_config_dir, get_data_dir, Config};
use crate::tui::Event;
use std::rc::Rc;
use std::cell::RefCell;
use ratatui::prelude::Color;
use ratatui::text::Line;
use tokio::process::Child;
use tokio::sync::mpsc;
use crate::action;

pub struct QuickOptions {
    pub options: OptionSelector,
    pub has_focus: bool,
    action_tx: Option<UnboundedSender<Action>>,
    pub launching_balatro: bool,
    local_action_tx: mpsc::UnboundedSender<Actions>,
    local_action_rx: mpsc::UnboundedReceiver<Actions>,
}

impl QuickOptions {
    pub fn new() -> Self {
        // let mut this = Self {
        //     options: OptionSelector::default(),
        //     has_focus: false,
        //     action_tx: None,
        //     launching_balatro: false,
        // };

        let mut options = OptionSelector::new(vec![
            vec![OptionSelectorText::new("Launch Balatro".to_string(), Style::default())],
            vec![OptionSelectorText::new("Open Balatro data folder".to_string(), Style::default())],
            vec![OptionSelectorText::new("Open Balatro mods folder".to_string(), Style::default())],
        ]);

        options.title = "Quick Options".to_string();

        let (local_tx, local_rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
            options,
            has_focus: false,
            action_tx: None,
            launching_balatro: false,
            local_action_tx: local_tx,
            local_action_rx: local_rx,
        }
    }
}

impl Component for QuickOptions {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.action_tx = Some(tx);
        self.options.register_local_action_handler(self.local_action_tx.clone())?;
        Ok(())
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            _ => {
                if self.launching_balatro {
                    self.launching_balatro = false;
                } else {
                    self.options.handle_key_event(key)?;
                }
            }
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Tick => {
                let act = self.local_action_rx.try_recv();
                if act.is_ok() {
                    let a = act?;
                    match a {
                        Actions::Selected(c) => {
                            match c {
                                0 => {
                                    launch_balatro(true).expect("Balatro failed to launch!");
                                    self.launching_balatro = true;
                                }
                                1 => {
                                    open(get_balatro_dir().to_str().unwrap())
                                }
                                2 => {
                                    open(get_balatro_appdata_dir().to_str().unwrap())
                                }
                                _ => {}
                            }
                        },
                    }
                }
            },
            _ => {}
        }
        Ok(None)
    }
    
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        if !self.launching_balatro {
            self.options.draw(frame, area)
        } else {
            frame.render_widget(
                Paragraph::new(
                    vec![
                        Line::from("Launching Balatro, please wait...").style(Style::default()).centered(),
                        Line::from("   (Press any key to continue)").style(Style::default().fg(Color::Gray)).centered(),
                    ]
                )
                .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(if self.has_focus { Style::default().fg(Color::LightCyan) } else { Style::default().fg(Color::White) })
                ),
                area
            );

            Ok(())
        }
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