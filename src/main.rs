use reqwest;
use std::fs::File;
use std::io;
use select::document::Document;
use select::predicate::{Name};
use std::collections::HashMap;

fn get_next_graph_data(x: &str) {
    // Extract just the json data from the javascript
    let i = x.find("chart: {\n").unwrap();
    let j = x.rfind(");").unwrap();
    let slice = format!("{}{}", "{\n", x[i..j].to_string());
    // println!("{}", slice);

    let mut map = HashMap::new();
    map.insert("type", "image/png");
    map.insert("options", &slice);

    let client = reqwest::Client::new();
    let res = client.post("http://export.highcharts.com/").json(&map).send();
    // let content = res.unwrap().bytes();
    // println!("{:?}", content);
    let mut content = res.unwrap();

    let mut file = File::create("chart.png").expect("Failed creating file");
    io::copy(&mut content, &mut file).expect("Unable to copy data");

}

fn main() {

    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://temple-covid.herokuapp.com/").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());

    // println!("{:?}", document);

    // Find the script tag in the html
    let mut iterable = document.find(Name("script"));
    // Skip over script tags that just "import" other js files
    iterable.next();
    iterable.next();
    iterable.next();
    iterable.next();
    let mut script = iterable.next();
    let mut text = script.unwrap().text();
    get_next_graph_data(&text);

    script = iterable.next();
    text = script.unwrap().text();
    // println!("{:?}", text);
    get_next_graph_data(&text);

    script = iterable.next();
    text = script.unwrap().text();
    get_next_graph_data(&text);

    script = iterable.next();
    text = script.unwrap().text();
    get_next_graph_data(&text);
}
