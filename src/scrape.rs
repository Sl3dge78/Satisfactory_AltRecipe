
use reqwest;
use scraper::{self, Html, Selector, ElementRef};

use crate::items::*;

#[allow(dead_code)]
pub fn do_the_scrape() {
    let response = reqwest::blocking::get("https://satisfactory.fandom.com/wiki/Hard_Drive").unwrap().text().unwrap();
    let document = Html::parse_document(&response);
    let sel = Selector::parse("#alternateRecipesTable").unwrap();
    let table = document.select(&sel).next().unwrap();
    let row_sel = Selector::parse("tr").unwrap();

    let recipes : Vec<Recipe> = table.select(&row_sel).filter_map(|x| parse_line(x)).collect();
    let serialized = serde_json::to_string(&recipes).unwrap();
    println!("{}", serialized);
}

fn parse_line(row: ElementRef) -> Option<Recipe> {
    let col_sel = Selector::parse("td").unwrap();
    let cols : Vec<ElementRef> = row.select(&col_sel).collect();
    if cols.len() < 1 {
        return None;
    }

    let name = cols[0].text().next().unwrap().to_string();
    let product = cols[1].text().next().unwrap().to_string();
    let input: Vec<Ingredients> = cols[2]
        .text()
        .filter(|e| e.len() > 1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|x| {
            Ingredients {
                name : x[0].to_string(),
                nb : x[1].parse::<f32>().unwrap_or(0.0)
            }
            })
        .collect();
    let rate: f32 = cols[3].text().next().unwrap().parse::<f32>().unwrap();

    /*
    for (i, image) in cols[1].select(&Selector::parse("img[src]").unwrap()).enumerate() {
        if let Some(link) = image.value().attr("data-src") {
            let png = link.find(".png").unwrap();
            let link = &link[0 .. png + 4];
            let file_name = &link[link.rfind("/").unwrap()+1 ..];
            println!("(\"{}\", \"{}\"),", product, file_name);

            let response = reqwest::blocking::get(link).unwrap().bytes().unwrap();
            std::fs::write(format!("res/images/{}", file_name), response).unwrap();
        }
    }
    */

    Some(Recipe {
        name,
        product,
        input,
        rate,
    })
}
