use macroquad::prelude::*;

mod save_score;

// Struct For the player so it will all be in one place
struct Player {
    x: f32,
    y: f32,
    texture: Texture2D, // The image
    color: macroquad::prelude::Color, // The color is not important but we have it anyway
    speed: f32
}

#[macroquad::main("Light Evader")] // Tell macroquad that this is the main function that we are going to use
async fn main() { // Main function

    // Create the player through the struct we initialized above
    let mut player = Player { x: screen_width() / 2.0, y: screen_height() / 2.0, texture: load_texture("src/assets/shade.png").await.unwrap(), color: macroquad::color::WHITE , speed: 250.0 };
    
    // Main loop
    loop {

        let score = get_time() as i64;

        clear_background(BLACK); // Set the backround color!!!

        let delta = get_frame_time(); // Get the frames so we will work with frames instead

        if is_key_pressed(KeyCode::Space) {
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

        draw_text(&format!("Score: {}", score), 10.0, 20.0, 30.0, WHITE);

        next_frame().await // Call next frame
    }
}
