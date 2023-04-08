#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use macroquad::{prelude::*, input};

// mod scrape;
// #[allow(unused_imports)]
// use scrape::*;

mod items;
use items::*;

const DARK_GRAY: Color = color_u8!(0x3F, 0x3F, 0x3F, 0xFF);
const GRAY: Color = color_u8!(0x65, 0x65, 0x65, 0xFF);
const LIGHT_GRAY: Color = color_u8!(0x90, 0x90, 0x90, 0xFF);
const BLACK: Color = color_u8!(0x0d,0x0d,0x0d,0xff); 
const WHITE: Color = color_u8!(0xff,0xff,0xff,0xff); 
const ORANGE: Color = color_u8!(0xe4,0x93,0x43,0xff); 
const BORDER_SIZE: f32 = 75.0;

struct Resources {
    warning_icon: Texture2D, 
    recipes: Vec<Recipe>,
    font: Font,
    item_textures: ItemTextureMap,
    globe: Texture2D,
    mam: Texture2D,
}

type ItemTextureMap = HashMap<&'static str, Option<Texture2D>>;

impl Resources {
    pub async fn new() -> Resources {
        Resources {
            warning_icon: Texture2D::from_file_with_format(include_bytes!("../out/res/warning.png"), None),
            recipes: serde_json::from_str(include_str!("../out/res/recipes.json")).unwrap(),
            font : load_ttf_font("res/DejaVuSans.ttf").await.unwrap(),
            item_textures: init_images(),
            globe: Texture2D::from_file_with_format(include_bytes!("../out/res/globe.png"), None),
            mam: Texture2D::from_file_with_format(include_bytes!("../out/res/mam.png"), None),
        }
    }
}

fn init_images() -> ItemTextureMap {
    let mut result: ItemTextureMap = HashMap::new();
    for i in items::IMAGE_MAP.iter() {
        result.insert(i.0, None);
    }
    result
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

fn draw_centered_texture(texture: Texture2D, x: f32, y: f32, size: f32, color: Color) {
    let x = x - size / 2.0;
    let y = y - size / 2.0;
    draw_texture_ex(texture, x, y, color, DrawTextureParams { dest_size: Some(Vec2::new(size, size)), ..Default::default()});

}

fn draw_rounded_rectangle(x: f32, y: f32, w: f32, h: f32, border: f32, color: Color) {
    // Draw 5 rectangles and 4 cirles for the corner
    draw_rectangle(x+border, y, w - border * 2.0, border, color); // Top
    draw_rectangle(x+border, y + h - border, w - border * 2.0, border, color); // Bottom

    draw_rectangle(x, y+border, border, h - border * 2.0, color); // Left
    draw_rectangle(x + w - border, y+border, border, h - border * 2.0, color); // Right

    draw_rectangle(x + border, y + border, w - border * 2.0, h - border * 2.0, color); // Center

    draw_circle(x + border, y + border, border, color); // Upper Right
    draw_circle(x + w - border, y + border, border, color); // Upper Left

    draw_circle(x + border, y + h - border, border, color); // Lower Right
    draw_circle(x + w - border, y + h - border, border, color); // Lower Left

}

fn recipe_button(recipe: &Recipe, offset_x: f32, selected: bool, res: &Resources) -> bool {
    // Calc extent 
    let y = BORDER_SIZE + 50.0;
    let h = screen_height() - (BORDER_SIZE + 50.0) * 2.0; 
    let w = screen_width()/3.0;
    let rect = Rect::new(offset_x, y, w, h);

    let mouse_in = rect.contains(input::mouse_position().into());
    let color = if selected { ORANGE } else {if mouse_in { GRAY } else { Color::from_rgba(0x00, 0x00, 0x00, 0x00) }};

    // Big bckg rectangle
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);

    // Prepare layout
    let image_sz = w / 2.0;
    let ingredient_size = w / 10.0;
    let mut layout_x = offset_x + ingredient_size;
    let mut layout_y = y + h / 2.0;

    // Image
    draw_centered_texture(res.globe, offset_x + w / 2.0, y + h / 2.0, image_sz, Color::from_rgba(0xff, 0xff, 0xff, 0x10));
    if let Some(tex) = res.item_textures.get(&recipe.product as &str).unwrap() {
        draw_centered_texture(*tex, offset_x + w / 2.0, y + h / 2.0, image_sz * 0.75, WHITE);
    }
    layout_y += image_sz / 2.0 + 20.0;

    // Recipe name
    draw_aligned_text(&format!("Alternate Blueprint: {}", &recipe.name), layout_x, layout_y, TextParams { font_size: 15, font: res.font, ..Default::default()}, TextAlignement::Left);
    layout_y += 15.0;

    // Ingredients 
    {
        let mut layout_x = layout_x;
        // In
        for input in &recipe.input {
            if let Some(tex) = res.item_textures.get(&input.name as &str).unwrap() {
                draw_rounded_rectangle(layout_x, layout_y, ingredient_size, ingredient_size, 5.0, LIGHT_GRAY);
                draw_texture_ex(*tex, layout_x, layout_y, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(ingredient_size, ingredient_size)), ..Default::default()});
                layout_x += ingredient_size + 5.0;
            }
        }

        // Arrow
        let pad = ingredient_size / 4.0;
        draw_triangle(Vec2::new(layout_x, layout_y + pad), Vec2::new(layout_x, layout_y + ingredient_size - pad), Vec2::new(layout_x + pad * 1.414, layout_y + ingredient_size / 2.0), LIGHT_GRAY);
        layout_x += pad * 1.414 + 5.0;

        // Out
        if let Some(tex) = res.item_textures.get(&recipe.product as &str).unwrap() {
            draw_rounded_rectangle(layout_x, layout_y, ingredient_size, ingredient_size, 5.0, LIGHT_GRAY);
            draw_texture_ex(*tex, layout_x, layout_y, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(ingredient_size, ingredient_size)), ..Default::default()});
        }
    }
    layout_y += ingredient_size + 20.0;

    (layout_x, _) = draw_aligned_text(&format!("Production Rate: "), layout_x, layout_y, TextParams { font_size: 15, font: res.font, ..Default::default()}, TextAlignement::Left);
    draw_aligned_text(&format!("{} per minute", recipe.rate), layout_x, layout_y, TextParams { font_size: 15, font: res.font, color: ORANGE, ..Default::default()}, TextAlignement::Left);

    if input::is_mouse_button_released(MouseButton::Left) {
        if mouse_in {
            return true;
        }
    }
    false 
}

#[macroquad::main("Satisfactory Alt Recipe")]
async fn main() {

    // do_the_scrape();
    rand::srand(miniquad::date::now() as u64); 
    
    let mut res = Resources::new().await;
    let mut state = State::new();

    let mut selected = select_recipes(&res.recipes, &mut res.item_textures).await;

    loop {
        clear_background(BLACK);
        draw_texture_ex(res.mam, 0.0, 0.0, WHITE, DrawTextureParams {dest_size: Some(Vec2::new(screen_width(), screen_height())), ..Default::default()});
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0x0d, 0x0d, 0x0d, 0xf0));

        for (i, recipe) in selected.iter().enumerate() {
            if recipe_button(recipe, i as f32 * screen_width() / 3.0, if let Some(r) = state.recipe_selected { r == i as u8 } else { false }, &res) {
                state.recipe_selected = Some(i as u8);
            }
        }
        draw_static_elems(&res);

        let w = 200.0;
        let x = screen_width() / 2.0 - w / 2.0;
        let text_color = if state.recipe_selected == None { LIGHT_GRAY } else { WHITE };
        if confirm_button(Rect {x, y: screen_height() - BORDER_SIZE, w, h: 50.0}, "Confirm",  TextParams { font: res.font, font_size: 20, color:text_color, ..Default::default()}) {
            selected = select_recipes(&res.recipes, &mut res.item_textures).await;
            state.recipe_selected = None;
        }
        next_frame().await
    }
}

async fn load_image_texture(name: &str, texs: &mut ItemTextureMap) {
    if let Some(v) = texs.get_mut(name) {
        if *v == None {
            let path = format!("res/images/{}", IMAGE_MAP.get(name).unwrap());
            *v = match load_texture(&path).await {
                Ok(tex) => Some(tex),
                Err(e) => {
                    error!("Unable to load {}: {}", path, e);
                    None
                },
            };
        }
    }
}

async fn recipe_load_textures(r: &Recipe, texs: &mut ItemTextureMap) {
    load_image_texture(&r.product, texs).await;
    for ing in &r.input {
        load_image_texture(&ing.name, texs).await;
    }
}

async fn select_recipes<'a>(recipes: &'a Vec<Recipe>, texs: &mut ItemTextureMap) -> Vec<&'a Recipe> {
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
    let mut result = Vec::new();
    for i in ids {
        info!("{:?}", recipes[i]);
        recipe_load_textures(&recipes[i], texs).await;
        result.push(&recipes[i]);
    }
    result
}

fn draw_centered_text(text: &str, x: f32, y: f32, text_params: TextParams) {
    let measure = measure_text(text, Some(text_params.font), text_params.font_size, 1.0);
    draw_text_ex(text, x - measure.width / 2.0, y + (measure.height / 2.0) - (measure.height - measure.offset_y), text_params);
}

#[allow(dead_code)]
enum TextAlignement {
    Left,
    Center,
}

fn draw_aligned_text(text: &str, x: f32, y: f32, text_params: TextParams, align: TextAlignement) -> (f32, f32) {
    let measure = measure_text(text, Some(text_params.font), text_params.font_size, 1.0);
    let x = match align {
        TextAlignement::Left => x,
        TextAlignement::Center => x - measure.width / 2.0,
    };
    draw_text_ex(text, x, y + (measure.height / 2.0) - (measure.height - measure.offset_y), text_params);
    (x + measure.width, y + measure.height)
}

fn draw_static_elems(res: &Resources) {
    draw_rectangle(0.0, 0.0, screen_width(), BORDER_SIZE, DARK_GRAY);
    draw_rectangle(0.0, screen_height() - BORDER_SIZE, screen_width(), BORDER_SIZE, DARK_GRAY);



    draw_texture(res.warning_icon, 10.0, BORDER_SIZE / 2.0 - res.warning_icon.height() / 2.0, WHITE);
    draw_aligned_text("Analysis Complete!", 50.0, BORDER_SIZE / 2.0, TextParams { font: res.font, color:WHITE, ..Default::default()}, TextAlignement::Left);
    draw_centered_text("The analysis of Hard Drive is completed! Select your desired reward.", screen_width() / 2.0, BORDER_SIZE + 25.0, TextParams { font: res.font, color:WHITE, ..Default::default()});
}
