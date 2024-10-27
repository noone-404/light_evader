// ui_manager.rs
use macroquad::prelude::*;

use crate::{Light, Player};
//use macroquad::ui::{hash, widgets, root_ui};

#[path = "read_score.rs"]
pub mod read_score;

pub struct UIManager {
    pub game_over: bool,
    pub score: i64,
    pub high_score: i64,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            game_over: false,
            score: 0,
            high_score: 0,
        }
    }

    pub async fn draw( &mut self, player: &mut Player, score: &mut i64, time_bettween_spawns: f64, spawn_timer: f64, lights: &mut Vec<Light>,) {    
    if self.game_over {
            let window_size = vec2(screen_width(), screen_height());
            let center_x = window_size.x / 2.0;
        
            // Optional: Draw a semi-transparent overlay
            let overlay_color = Color::from_rgba(0, 0, 0, 128); // Semi-transparent black
            draw_rectangle(0.0, 0.0, window_size.x, window_size.y, overlay_color);
        
            // Draw the UI elements manually
            let label_spacing = 40.0; // Space between labels and buttons
            let base_y = 200.0;
        
            // Centering elements
            let score_label = format!("Score: {}", self.score);
            let high_score_label = format!("High Score: {}", self.high_score);
        
            let score_size = measure_text(&score_label, None, 20, 1.0);
            let high_score_size = measure_text(&high_score_label, None, 20, 1.0);
            let button_width = 100.0;
            let button_height = 30.0;
        
            // Draw score and high score labels
            draw_text(
                &score_label,
                center_x - score_size.width / 2.0,
                base_y,
                20.0,
                WHITE,
            );
            draw_text(
                &high_score_label,
                center_x - high_score_size.width / 2.0,
                base_y + label_spacing,
                20.0,
                WHITE,
            );
        
            // Draw buttons
            let quit_button_y = base_y + label_spacing * 2.0;
            let play_again_button_y = base_y + label_spacing * 3.0;
        
            // Draw Quit button
            draw_rectangle(center_x - button_width / 2.0, quit_button_y, button_width, button_height, WHITE);
            let quit_text = "Quit";
            let quit_text_size = measure_text(quit_text, None, 20, 1.0);
            let quit_text_y = quit_button_y + (button_height / 2.0) - (quit_text_size.height / 2.0) + 10.0; // Adjusted for fine alignment
            draw_text(quit_text, center_x - quit_text_size.width / 2.0, quit_text_y, 20.0, BLACK);
        
            // Draw Play Again button
            draw_rectangle(center_x - button_width / 2.0, play_again_button_y, button_width, button_height, WHITE);
            let play_again_text = "Play Again";
            let play_again_text_size = measure_text(play_again_text, None, 20, 1.0);
            let play_again_text_y = play_again_button_y + (button_height / 2.0) - (play_again_text_size.height / 2.0) + 10.0; // Adjusted for fine alignment
            draw_text(play_again_text, center_x - play_again_text_size.width / 2.0, play_again_text_y, 20.0, BLACK);
        
            // Save the score if it's a high score
            read_score::new_high_score(*score as i64);

            // Check for mouse clicks on buttons
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
        
                if mouse_pos.0 >= center_x - button_width / 2.0 && mouse_pos.0 <= center_x + button_width / 2.0 {
                    if mouse_pos.1 >= quit_button_y && mouse_pos.1 < quit_button_y + button_height {
                        std::process::exit(0);
                    }
                    if mouse_pos.1 >= play_again_button_y && mouse_pos.1 < play_again_button_y + button_height {
                        self.score = 0;
                        self.reset_game(player, score, time_bettween_spawns, spawn_timer, lights).await;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, score: i64) {
        self.score = score;
        self.game_over = true;
        self.high_score = read_score::return_high_score();
    }

    async fn reset_game( &mut self, player: &mut Player, score: &mut i64, mut _time_bettween_spawns: f64, mut _spawn_timer: f64, lights: &mut Vec<Light>,) {
        self.game_over = false;
        *score = 0;
        _time_bettween_spawns = 1.0;
        _spawn_timer = 0.0;
    
        player.x = screen_width() / 2.0;
        player.y = screen_height() / 2.0;
        player.texture = load_texture("src/assets/shade.png").await.unwrap();
        player.player_is_alive = true;
    
        lights.clear();
    }
}
