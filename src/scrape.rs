// This is the scraping code I used to gather the data I needed. Need to compile it as main.
// Deps: reqwest, scraper
use serde::Serialize;
use reqwest;
use scraper::{self, Html, Selector, ElementRef};

#[derive(Debug, Serialize)]
struct Ingredients {
    name: String,
    nb: f32,
}

#[derive(Debug, Serialize)]
struct Recipe {
    name: String,
    product: String,
    input: Vec<Ingredients>,
}

fn main() {
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
    let input = cols[2]
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

    Some(Recipe {
        name,
        product,
        input,
    })
}
