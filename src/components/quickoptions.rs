use std::io::BufReader;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::layout::{Rect, Size};
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use std::process::{Command, Stdio};
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;
use balatro_tui::launch_balatro;
use crate::action::Action;
use crate::components::Component;
use crate::components::optionselector::{OptionSelector, OptionSelectorText};
use crate::config::Config;
use crate::tui::Event;

pub struct QuickOptions {
    pub options: OptionSelector<Box<dyn FnMut(u16)>>,
    pub has_focus: bool,
    action_tx: Option<UnboundedSender<Action>>,
    pub launching_balatro: bool,
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
        ]);

        options.title = "Quick Options".to_string();
        
        Self {
            options,
            has_focus: false,
            action_tx: None,
            launching_balatro: false,
        }
    }
    
    pub fn setup_callback(&mut self) {
        let launch_fn = Box::new(move |_| {
            launch_balatro(true).expect("Balatro failed to launch :(");
        });
        self.options.set_callback(launch_fn);
    }
    
    pub fn launch_game(&mut self) {
        self.launching_balatro = true;
        launch_balatro(true).expect("Balatro failed to launch :(");
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
                self.options.handle_key_event(key)?;
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        if !self.launching_balatro {
            self.options.draw(frame, area)
        } else {
            frame.render_widget(
                Paragraph::new(
                    "Launching Balatro, please wait..."
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