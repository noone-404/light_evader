// ui_manager.rs
use macroquad::prelude::*;
use macroquad::ui;

#[path = "read_score.rs"]
pub mod read_score;

pub struct UIManager {
    pub game_over: bool,
    pub score: u32,
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

    pub fn draw(&mut self) {
        if self.game_over {
            draw_text("Game Over", screen_width() / 2.1, 165.0, 20.0, WHITE);

            // For some reason it doesn't work
            if read_score::new_high_score(self.score as i64) == true {
                draw_text(
                    format!("New High Score: {}", self.score).as_str(),
                    screen_width() / 2.1 - 8.0,
                    190.0,
                    20.0,
                    WHITE,
                );
            } else if read_score::new_high_score(self.score as i64) == false {
                draw_text(
                    format!("Your score: {}", self.score).as_str(),
                    screen_width() / 2.1 - 8.0,
                    190.0,
                    20.0,
                    WHITE,
                );
                draw_text(
                    format!("High Score: {}", self.high_score).as_str(),
                    screen_width() / 2.1 - 1.0,
                    200.0,
                    14.0,
                    WHITE,
                );
            }

            if ui::root_ui().button(Vec2::new(screen_width() / 2.1 + 15.0, 250.0), "Retry") {
                // Retry logic here
                self.game_over = false;
                self.score = 0;
            }

            if ui::root_ui().button(Vec2::new(screen_width() / 2.1 + 20.0, 300.0), "Exit") {
                // Exit logic here
                std::process::exit(0);
            }
        }
    }

    pub fn update(&mut self, score: u32) {
        self.score = score;
        self.game_over = true;
        self.high_score = read_score::return_high_score();
    }
}
