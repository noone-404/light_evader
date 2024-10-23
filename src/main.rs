use macroquad::prelude::*; // Import everything from the library we want to use
use ::rand::{Rng, thread_rng}; // Import random number generator

mod save_score; // Tell that we want to use the save_score.rs file

// Struct For the player so it will all be in one place
struct Player {
    x: f32,
    y: f32,
    texture: Texture2D, // The image
    color: macroquad::prelude::Color, // The color is not important but we have it anyway
    speed: f32
}

// Get all of the light stuff in one place with a sctruct
struct Light {
    x: f32,
    y: f32,
    radious: f32,
    color: macroquad::prelude::Color
}

#[macroquad::main("Light Evader")] // Tell macroquad that this is the main function that we are going to use
async fn main() { // Main function

    let mut rng = thread_rng(); // Initialize the random number generator by calling the thread_rng function directly

    // Create the player through the struct we initialized above
    let mut player = Player { x: screen_width() / 2.0, y: screen_height() / 2.0, texture: load_texture("src/assets/shade.png").await.unwrap(), color: macroquad::color::WHITE , speed: 250.0 };
    
    // We use the random number generator to randomly initialize the x and y of the light and then go with the struct normally
    let light = Light { x: rng.gen_range(0.0..screen_width()), y: rng.gen_range(0.0..screen_height()), radious: 100.0, color: macroquad::color::YELLOW };

    // Main loop
    loop {

        let score = get_time() as i64; // We initialize a variable for the score that is called in the main loop because we want it constantly to update and that's because it increases according to the time to the time that passed

        clear_background(BLACK); // Set the backround color!!!

        let delta = get_frame_time(); // Get the frames so we will work with frames instead

        if is_key_pressed(KeyCode::Space) { // For now we will have the space bar to save the score
            save_score::save_score(score).unwrap();
            break;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        } // Break it down if hit escape

        // Movement of player
        if is_key_down(KeyCode::A) {
            player.x -= player.speed * delta;
        }

        if is_key_down(KeyCode::D) {
            player.x += player.speed * delta;
        }

        if is_key_down(KeyCode::W) {
            player.y -= player.speed * delta;
        }

        if is_key_down(KeyCode::S) {
            player.y += player.speed * delta;
        }

        // Make sure the player doesn't go out of the screen but it will anyway
        player.x = clamp(player.x, 0.0, screen_width() - player.texture.width());
        player.y = clamp(player.y, 0.0, screen_height() - player.texture.height());
        
        draw_texture(&player.texture, player.x, player.y, player.color); // Draw the player

        //draw_text(&format!("FPS: {}", get_fps().to_string()), 10.0, 10.0, 20.0, WHITE);

        draw_circle(light.x, light.y, light.radious, light.color); // Draw the light on the screen

        draw_text(&format!("Score: {}", score), 10.0, 20.0, 30.0, WHITE); // Draw the score on the screen on a comfortable place

        next_frame().await // Call next frame
    }
}
