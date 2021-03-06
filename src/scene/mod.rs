use crate::ggez::{Context, GameResult};

use crate::SharedGameState;
use crate::ui::Components;

pub mod game_scene;
pub mod loading_scene;

pub trait Scene {
    fn init(&mut self, _state: &mut SharedGameState, _ctx: &mut Context) -> GameResult { Ok(()) }

    fn tick(&mut self, _state: &mut SharedGameState, _ctx: &mut Context) -> GameResult { Ok(()) }

    fn draw(&self, _state: &mut SharedGameState, _ctx: &mut Context) -> GameResult { Ok(()) }

    fn debug_overlay_draw(&mut self, _game_ui: &mut Components, _state: &mut SharedGameState, _ctx: &mut Context, _frame: &mut imgui::Ui) -> GameResult { Ok(()) }
}
