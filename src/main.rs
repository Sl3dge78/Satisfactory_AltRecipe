use macroquad::{prelude::*, input};
use serde::Deserialize;

const DARK_GRAY : Color = color_u8!(0x3F, 0x3F, 0x3F, 0xFF);
const GRAY : Color = color_u8!(0x65, 0x65, 0x65, 0xFF);
const LIGHT_GRAY : Color = color_u8!(0x90, 0x90, 0x90, 0xFF);
const BLACK : Color = color_u8!(0x0d,0x0d,0x0d,0xff); 
const WHITE: Color = color_u8!(0xff,0xff,0xff,0xff); 
const ORANGE: Color = color_u8!(0xe4,0x93,0x43,0xff); 
const BORDER_SIZE : f32 = 75.0;

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
    font: Font,
}

impl Resources {
    pub async fn new() -> Resources {
        Resources {
            warning_icon: Texture2D::from_file_with_format(include_bytes!("../res/warning.png"), None),
            recipes: serde_json::from_str(include_str!("../res/recipes.json")).unwrap(),
            font : load_ttf_font("res/DejaVuSans.ttf").await.unwrap(),
        }
    }
}

struct State {
    recipe_selected: Option<u8>,
}

impl State {
    fn new() -> Self {
        Self {
            recipe_selected: None,
        }
    }
}

fn confirm_button(rect: Rect, text: &str, text_params: TextParams) -> bool {
    let mouse_in = rect.contains(input::mouse_position().into());
    let color = if mouse_in { LIGHT_GRAY } else { GRAY };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_centered_text(text, rect.x + rect.w / 2.0, rect.y + rect.h / 2.0, text_params);
    if input::is_mouse_button_released(MouseButton::Left) {
        if mouse_in {
            return true;
        }
    }
    false 
}

fn recipe_button(recipe: &Recipe, offset_x: f32, selected: bool, font: Font) -> bool {
    let y = BORDER_SIZE + 50.0;
    let h = screen_height();
    let w = screen_width()/3.0;
    let rect = Rect::new(offset_x, y, w, h);
    let mouse_in = rect.contains(input::mouse_position().into());
    let color = if selected { ORANGE } else {if mouse_in { GRAY } else { BLACK }};
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);

    draw_centered_text(&recipe.name, offset_x + w / 2.0, y + h / 2.0, TextParams { font_size: 30, font, ..Default::default()} );

    if input::is_mouse_button_released(MouseButton::Left) {
        if mouse_in {
            return true;
        }
    }
    false 
}

#[macroquad::main("Satisfactory Alt Recipe")]
async fn main() {
    
    let res = Resources::new().await;
    let mut state = State::new();

    let mut selected = select_recipes(&res.recipes);

    loop {
        clear_background(BLACK);
        for (i, recipe) in selected.iter().enumerate() {
            if recipe_button(recipe, i as f32 * screen_width() / 3.0, if let Some(r) = state.recipe_selected { r == i as u8 } else { false }, res.font) {
                state.recipe_selected = Some(i as u8);
            }
        }
        draw_static_elems(&res);

        let w = 200.0;
        let x = screen_width() / 2.0 - w / 2.0;
        let text_color = if state.recipe_selected == None { LIGHT_GRAY } else { WHITE };
        if confirm_button(Rect {x, y: screen_height() - BORDER_SIZE, w, h: 50.0}, "Confirm",  TextParams { font: res.font, font_size: 20, color:text_color, ..Default::default()}) {
            selected = select_recipes(&res.recipes);
            state.recipe_selected = None;
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

fn draw_centered_text(text: &str, x: f32, y: f32, text_params: TextParams) {
    let measure = measure_text(text, Some(text_params.font), text_params.font_size, 1.0);
    draw_text_ex(text, x - measure.width / 2.0, y + (measure.height / 2.0) - (measure.height - measure.offset_y), text_params);
}

enum TextAlignement {
    Left,
    Center,
}

fn draw_aligned_text(text: &str, x: f32, y: f32, text_params: TextParams, align: TextAlignement) {
    let measure = measure_text(text, Some(text_params.font), text_params.font_size, 1.0);
    let x = match align {
        TextAlignement::Left => x,
        TextAlignement::Center => x - measure.width / 2.0,
    };
    draw_text_ex(text, x, y + (measure.height / 2.0) - (measure.height - measure.offset_y), text_params);
}

fn draw_static_elems(res: &Resources) {
    draw_rectangle(0.0, 0.0, screen_width(), BORDER_SIZE, DARK_GRAY);
    draw_rectangle(0.0, screen_height() - BORDER_SIZE, screen_width(), BORDER_SIZE, DARK_GRAY);

    draw_texture(res.warning_icon, 10.0, BORDER_SIZE / 2.0 - res.warning_icon.height() / 2.0, WHITE);
    draw_aligned_text("Analysis Complete!", 50.0, BORDER_SIZE / 2.0, TextParams { font: res.font, color:WHITE, ..Default::default()}, TextAlignement::Left);
    draw_centered_text("The analysis of Hard Drive is completed! Select your desired reward.", screen_width() / 2.0, BORDER_SIZE + 25.0, TextParams { font: res.font, color:WHITE, ..Default::default()});
}
