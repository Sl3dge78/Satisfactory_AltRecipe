use macroquad::{prelude::*, ui::{*, widgets::Group}};
use futures::executor::block_on;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Ingredients {
    name: String,
    nb: f32,
}

#[derive(Debug, Deserialize)]
struct Recipe {
    name: String,
    product: String,
    input: Vec<Ingredients>,
}

struct Resources {
    warning_icon: Texture2D, 
    recipes: Vec<Recipe>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            warning_icon: Texture2D::from_file_with_format(include_bytes!("../res/warning.png"), None),
            recipes: serde_json::from_str(include_str!("../res/recipes.json")).unwrap(),
        }
    }
}

#[macroquad::main("Satisfactory Alt Recipe")]
async fn main() {
    
    let res = Resources::new();

    let selected = select_recipes(&res.recipes);

    loop {
        clear_background(DARKGRAY);
        draw_static_elems(&res);
        for recipe in &selected {
            Group::new(hash!(), Vec2::new(screen_width()/3.0, screen_height())).ui(&mut root_ui(), |ui| {
                if ui.button(None, recipe.name.clone()) {
                    info!("Yahoo");
                }
            });
        }
        next_frame().await
    }
}

fn select_recipes(recipes: &Vec<Recipe>) -> Vec<&Recipe> {
    let mut ids = Vec::new();
    loop {
        let nb = rand::rand() as usize % recipes.len();
        if ids.contains(&nb) {
            continue;
        }
        ids.push(nb);
        if ids.len() == 3 {
            break;
        }
    }
    println!("{:?}", ids);
    let mut result = Vec::new();
    for i in ids {
        result.push(&recipes[i]);
    }
    result
}

fn draw_static_elems(res: &Resources) {
    let analysis_str = "The analysis of Hard Drive is completed! Select your desired reward."; 
    let center = get_text_center(analysis_str, None, 30, 1.0, 0.0);
    draw_rectangle(0.0, 0.0, screen_width(), 50.0, Color::from_rgba(0x3F, 0x3F, 0x3F, 0xFF));
    draw_rectangle(0.0, screen_height() - 50.0, screen_width(), 50.0, Color::from_rgba(0x3F, 0x3F, 0x3F, 0xFF));

    draw_texture(res.warning_icon, 10.0, 10.0, WHITE);
    draw_text("Analysis Complete!", 50.0, 35.0, 30.0, WHITE);
    draw_text(analysis_str, center.x, 80.0, 30.0, WHITE);

}
