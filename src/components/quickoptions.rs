use std::io::{BufReader, Error};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Rect, Size};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use std::process::{Command, Stdio};
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;
use balatro_tui::{get_balatro_appdata_dir, get_balatro_dir, launch_balatro, xdg_open};
use crate::action::Action;
use crate::components::Component;
use crate::components::optionselector::{OptionSelector, OptionSelectorText};
use crate::config::{get_config_dir, get_data_dir, Config};
use crate::tui::Event;
use std::rc::Rc;
use std::cell::RefCell;
use ratatui::prelude::Color;
use ratatui::text::Line;
use tokio::process::Child;

pub struct QuickOptions {
    pub options: OptionSelector<Box<dyn FnMut(u16)>>,
    pub has_focus: bool,
    action_tx: Option<UnboundedSender<Action>>,
    pub launching_balatro: Rc<RefCell<bool>>,
}

impl Clone for QuickOptions {
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
            has_focus: self.has_focus,
            action_tx: self.action_tx.clone(),
            launching_balatro: self.launching_balatro.clone(),
        }
    }
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
            vec![OptionSelectorText::new("Open config folder".to_string(), Style::default())],
        ]);

        options.title = "Quick Options".to_string();

        Self {
            options,
            has_focus: false,
            action_tx: None,
            launching_balatro: Rc::new(RefCell::new(false)),
        }
    }

    pub fn setup_callback(&mut self) {
        let launching_balatro = Rc::clone(&self.launching_balatro);
        let on_select = Box::new(move |selection: u16| {
            match selection {
                0 => { // Launch balatro
                    *launching_balatro.borrow_mut() = true;
                    match launch_balatro(true) {
                        Ok(_) => {}
                        Err(error) => {
                            error!("Error launching balatro: {}", error);
                        }
                    }
                }
                1 => {
                    let _ = xdg_open(get_balatro_dir().to_str().unwrap());
                }
                2 => {
                    let _ = xdg_open(get_balatro_appdata_dir().to_str().unwrap());
                }
                3 => {
                    let _ = xdg_open(get_config_dir().to_str().unwrap());
                }
                _ => {
                    error!("Unimplemented option selected");
                }
            }
        });
        self.options.set_callback(on_select);
    }
}

impl Component for QuickOptions {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            _ => {
                if *self.launching_balatro.borrow() {
                    *self.launching_balatro.borrow_mut() = false;
                } else {
                    self.options.handle_key_event(key)?;
                }
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        if !*self.launching_balatro.borrow() {
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