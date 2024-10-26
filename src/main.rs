use ::rand::{thread_rng, Rng};
use macroquad::prelude::*; // Import everything from the library we want to use // Import random number generator

mod save_score; // Tell that we want to use the save_score.rs file
pub mod ui_manager;

// Struct For the player so it will all be in one place
struct Player {
    x: f32,
    y: f32,
    texture: Texture2D,    // The image
    player_is_alive: bool, // Determine if player is alive or not
    color: Color,          // The color is not important but we have it anyway
    speed: f32,
}

// Get all of the light stuff in one place with a sctruct
struct Light {
    x: f32,
    y: f32,
    radious: f32,
    max_radious: f32, // The maximum radious of the light which will be useful
    color: Color,
    timer: f32,
}

#[macroquad::main("Light Evader")] // Tell macroquad that this is the main function that we are going to use
// Main function
async fn main() {
    Conf {
        window_title: "Light Evader".to_owned(),
        window_width: 800.to_owned(),
        window_height: 600.to_owned(),
        fullscreen: false.to_owned(),
        window_resizable: false.to_owned(),
        ..Default::default()
    };

    let mut lights: Vec<Light> = Vec::new(); // Initialize the lights as a vector so it could contain muliple lights lights at once

    // Create the player through the struct we initialized above
    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        texture: load_texture("src/assets/shade.png").await.unwrap(),
        player_is_alive: true,
        color: macroquad::color::WHITE,
        speed: 250.0,
    };

    let mut score = get_time() as i64;

    let mut spawn_timer = 0.0;

    let mut time_bettween_spawns = 1.0;

    // Main loop
    loop {
        clear_background(BLACK); // Set the backround color!!!

        let delta = get_frame_time(); // Get the frames so we will work with frames instead

        if player.player_is_alive {
            score = get_time() as i64;

            if is_key_pressed(KeyCode::Space) {
                // For now we will have the space bar to save the score
                save_score::save_score(score).unwrap();
            }

            // Break it down if hit escape
            if is_key_down(KeyCode::Escape) {
                println!("Closing Game\n");
                break;
            }

            // Movement of player
            // Move Left
            if is_key_down(KeyCode::A) {
                player.x -= player.speed * delta;
            }

            // Move Right
            if is_key_down(KeyCode::D) {
                player.x += player.speed * delta;
            }

            // Move Up
            if is_key_down(KeyCode::W) {
                player.y -= player.speed * delta;
            }

            // Move Down
            if is_key_down(KeyCode::S) {
                player.y += player.speed * delta;
            }

            // Spawn a new light every second
            spawn_timer += get_frame_time();
            if spawn_timer >= time_bettween_spawns {
                let mut rng = thread_rng();
                lights.push(Light {
                    x: rng.gen_range(0.0..screen_width()),
                    y: rng.gen_range(0.0..screen_height()),
                    radious: 0.0,
                    max_radious: rng.gen_range(50.0..100.0),
                    color: macroquad::color::YELLOW,
                    timer: 5.0,
                });
                spawn_timer = 0.0;
            }
            if score > 14 {
                time_bettween_spawns = 0.8;
            } else if score > 22 {
                time_bettween_spawns = 0.6;
            }

            // Update the lights
            for light in &mut lights {
                light.radious += 0.1;
                if light.radious > light.max_radious {
                    light.radious = light.max_radious;
                }
                light.timer -= delta;
            }

            lights.retain(|l| l.timer > 0.0);

            // Draw the lights
            for light in &lights {
                draw_circle(light.x, light.y, light.radious, light.color);
            }
        }

        // Check for collision with the player
        for light in &lights {
            // Over here we check if the distance is less than the radious of the light
            let distance = ((player.x + player.texture.width() / 2.0 - light.x).powi(2)
                + (player.y + player.texture.height() / 2.0 - light.y).powi(2))
            .sqrt();
            if distance < (player.texture.width() / 2.0 + light.radious) {
                player.texture = load_texture("src/assets/shade_light_up.png").await.unwrap();
                player.player_is_alive = false;

                let mut ui = ui_manager::UIManager::new();

                ui.update(score as u32);

                ui.draw();
            }
        }

        // Make sure the player doesn't go out of the screen by saying to minimize the x and y with the screen width and height minus the texture so we won't have accurences were the texture goes off screen
        player.x = clamp(player.x, 0.0, screen_width() - player.texture.width());
        player.y = clamp(player.y, 0.0, screen_height() - player.texture.height());

        draw_texture(&player.texture, player.x, player.y, player.color); // Draw the player

        draw_text(&format!("Score: {}", score), 10.0, 20.0, 30.0, WHITE); // Draw the score on the screen on a comfortable place

        draw_text(
            &format!("FPS: {}", get_fps().to_string()),
            10.0,
            40.0,
            20.0,
            WHITE,
        ); // Show the fps on screen

        // We make the light increase it's radious overtime so he player will have time to see it and also here is where the max_radious variable is useful
        for light in &mut lights {
            light.radious += 0.5;
            if light.radious > light.max_radious {
                light.radious = light.max_radious;
            }
        }

        next_frame().await // Call next frame
    }
}
