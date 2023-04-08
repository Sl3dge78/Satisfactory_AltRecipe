#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use macroquad::{prelude::{*, coroutines::*}, input};

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
    checkmark: Texture2D,
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
            checkmark: Texture2D::from_file_with_format(include_bytes!("../out/res/ficsit_check.png"), None),
        }
    }
}

// --------
// Loading

fn init_images() -> ItemTextureMap {
    let mut result: ItemTextureMap = HashMap::new();
    for i in items::IMAGE_MAP.iter() {
        result.insert(i.0, None);
    }
    result
}

// --------
// Ui helpers

fn draw_centered_text(text: &str, x: f32, y: f32, text_params: TextParams) -> (f32, f32, f32, f32) {
    let measure = measure_text(text, Some(text_params.font), text_params.font_size, 1.0);
    let x = x - measure.width / 2.0;
    let y = y + (measure.height / 2.0) - (measure.height - measure.offset_y);
    draw_text_ex(text, x, y, text_params);
    return (x, y - measure.height, measure.width, measure.height);
}

fn draw_aligned_text(text: &str, x: f32, y: f32, text_params: TextParams) -> (f32, f32) {
    let measure = measure_text(text, Some(text_params.font), text_params.font_size, 1.0);
    let y_ = y + text_params.font_size as f32 / 2.0;
    draw_text_ex(text, x, y_, text_params);
    (x + measure.width, y + measure.height)
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

enum Alignement {
    Left,
    Center,
}

fn draw_icon_text(text: &str, icon: Texture2D, x: f32, y: f32, alignement: Alignement, text_params: TextParams) {
    let icon_size: f32 = text_params.font_size as f32;
    let pad = 5.0;
    let mut size = measure_text(text, Some(text_params.font), text_params.font_size, 1.0);
    size.width += icon_size + pad; // padding

    let mut layout_x = match alignement {
        Alignement::Center => screen_width() / 2.0 - size.width / 2.0,
        Alignement::Left => x,
    };

    draw_texture_ex(icon, layout_x, y - size.height / 2.0, text_params.color, DrawTextureParams {dest_size: Some(Vec2::new(icon_size, icon_size)), ..Default::default()});
    layout_x += icon_size + pad;
    draw_aligned_text(text, layout_x, y, text_params);

}

// --------
// Ui elements

fn draw_ingredient(item: &Item, x: &mut f32, y: f32, size: f32) {
    if let Some(tex) = item.texture {
        draw_rounded_rectangle(*x, y, size, size, 5.0, LIGHT_GRAY);
        draw_centered_texture(tex, *x + size / 2.0, y + size / 2.0, size * 0.90, WHITE);
        *x += size + 5.0;
    }

}

fn recipe_button(recipe: &Recipe, offset_x: f32, selected: bool, font: Font, globe: Texture2D) -> bool {
    // Calc extent 
    let rect = Rect::new(offset_x, BORDER_SIZE + 50.0, screen_width() / 3.0, screen_height() - (BORDER_SIZE + 50.0) * 2.0);
    let font_size = 15;

    let mouse_in = rect.contains(input::mouse_position().into());
    let color = if selected { ORANGE } else {if mouse_in { GRAY } else { Color::from_rgba(0x00, 0x00, 0x00, 0x00) }};

    // Big bckg rectangle
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);

    // Prepare layout
    let mut layout_y = rect.y + rect.h / 2.0;

    // Image
    {
        let image_sz = rect.w / 2.0;
        let x = offset_x + rect.w / 2.0;
        let y = rect.y + rect.h / 2.0;
        draw_centered_texture(globe, x, y, image_sz, Color::from_rgba(0xff, 0xff, 0xff, 0x10));
        if let Some(tex) = recipe.product.texture {
            draw_centered_texture(tex, x, y, image_sz * 0.75, WHITE);
        }
        layout_y += image_sz / 2.0 + 20.0;
    }

    let ingredient_size = rect.w / 10.0;
    let mut layout_x = rect.x + ingredient_size;

    // Recipe name
    draw_aligned_text(&format!("Alternate Blueprint: {}", &recipe.name), layout_x, layout_y, TextParams { font_size, font, ..Default::default()});
    layout_y += 15.0;

    // Ingredients 
    {
        let mut layout_x = layout_x;
        // In
        for input in &recipe.input {
            draw_ingredient(&input.name, &mut layout_x, layout_y, ingredient_size);
        }

        // Arrow
        let pad = ingredient_size / 4.0;
        draw_triangle(Vec2::new(layout_x, layout_y + pad), Vec2::new(layout_x, layout_y + ingredient_size - pad), Vec2::new(layout_x + pad * 1.414, layout_y + ingredient_size / 2.0), LIGHT_GRAY);
        layout_x += pad * 1.414 + 5.0;

        // Out
        draw_ingredient(&recipe.product, &mut layout_x, layout_y, ingredient_size);
        layout_y += ingredient_size + 20.0;
    }

    (layout_x, _) = draw_aligned_text(&format!("Production Rate: "), layout_x, layout_y, TextParams { font_size, font, ..Default::default()});
    draw_aligned_text(&format!("{} per minute", recipe.rate), layout_x, layout_y, TextParams { font_size, font, color: ORANGE, ..Default::default()});

    if input::is_mouse_button_released(MouseButton::Left) {
        if mouse_in {
            return true;
        }
    }
    false 
}

fn confirm_button(text_params: TextParams, checkmark: Texture2D) -> bool {
    let w = 200.0;
    let x = screen_width() / 2.0 - w / 2.0;
    let rect = Rect {x, y: screen_height() - BORDER_SIZE, w, h: 50.0};

    let mouse_in = rect.contains(input::mouse_position().into());
    let color = if mouse_in { LIGHT_GRAY } else { GRAY };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);

    draw_icon_text("Confirm", checkmark, rect.x + rect.w / 2.0, rect.y + rect.h / 2.0, Alignement::Center, text_params);

    // Size elems
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

    let mut selected_recipe: Option<u8> = None;
    let mut displayed_recipes = select_recipes(&mut res.recipes, &mut res.item_textures).await;
    let mut next = start_coroutine(async move { (select_recipes(&mut res.recipes, &mut res.item_textures).await, res.recipes, res.item_textures) });

    loop {
        clear_background(BLACK);
        // Background image + blur
        draw_texture_ex(res.mam, 0.0, 0.0, WHITE, DrawTextureParams {dest_size: Some(Vec2::new(screen_width(), screen_height())), ..Default::default()});
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0x0d, 0x0d, 0x0d, 0xf0));
        
        // Top/Bottom Borders
        draw_rectangle(0.0, 0.0, screen_width(), BORDER_SIZE, DARK_GRAY);
        draw_rectangle(0.0, screen_height() - BORDER_SIZE, screen_width(), BORDER_SIZE, DARK_GRAY);

        // Top text
        draw_icon_text("Analysis Complete!", res.warning_icon, 10.0, BORDER_SIZE / 2.0, Alignement::Left, TextParams {font:res.font, ..Default::default()});

        for (i, recipe) in displayed_recipes.iter().enumerate() {
            let is_selected = if let Some(r) = selected_recipe { r == i as u8 } else { false };
            if recipe_button(recipe, i as f32 * screen_width() / 3.0, is_selected, res.font, res.globe) {
                selected_recipe = Some(i as u8);
            }
        }

        draw_centered_text("The analysis of Hard Drive is completed! Select your desired reward.", screen_width() / 2.0, BORDER_SIZE + 25.0, TextParams { font: res.font, color:WHITE, ..Default::default()});

        let text_color = if selected_recipe == None { LIGHT_GRAY } else { WHITE };
        if confirm_button(TextParams { font: res.font, font_size: 20, color:text_color, ..Default::default()}, res.checkmark) && selected_recipe != None {
            (displayed_recipes, res.recipes, res.item_textures) = next.retrieve().unwrap();
            next = start_coroutine(async move { (select_recipes(&mut res.recipes, &mut res.item_textures).await, res.recipes, res.item_textures) });
            selected_recipe = None;
        }
        next_frame().await;
    }
}


async fn select_recipes(recipes: &mut Vec<Recipe>, texs: &mut ItemTextureMap) -> Vec<Recipe> {
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
    for i in &ids {
        recipes[*i].load(texs).await;
    }
    ids.iter().map(|i| recipes[*i].clone()).collect()
}

