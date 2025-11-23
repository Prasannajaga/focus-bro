use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
pub struct MetaData {
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
}

#[tauri::command]
pub fn fetch_meta(url: String) -> MetaData {
    let body = reqwest::blocking::get(&url)
        .and_then(|res| res.text())
        .unwrap_or_default();

    let doc = Html::parse_document(&body);

    let get_meta = |name: &str| {
        let selector = Selector::parse(&format!("meta[name=\"{}\"]", name)).unwrap();
        doc.select(&selector)
            .next()
            .and_then(|e| e.value().attr("content"))
            .map(|s| s.to_string())
    };

    let get_og = |property: &str| {
        let selector =
            Selector::parse(&format!("meta[property=\"{}\"]", property)).unwrap();
        doc.select(&selector)
            .next()
            .and_then(|e| e.value().attr("content"))
            .map(|s| s.to_string())
    };

    MetaData {
        title: get_meta("title")
            .or_else(|| get_og("og:title"))
            .or_else(|| {
                let selector = Selector::parse("title").unwrap();
                doc.select(&selector)
                    .next()
                    .map(|e| e.text().collect::<String>())
            }),

        description: get_meta("description").or_else(|| get_og("og:description")),
        image: get_og("og:image"),
    }
}
