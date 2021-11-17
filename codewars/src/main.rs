#![allow(dead_code)] // Cargo level attribute
fn main() {
    println!("{}", play_pass("BORN IN 2015!", 1));
    let s = "abcd\nefgh\nijkl\nmnop".to_string();
    println!(
        "original:\n{}\nvert:\n{}\nhor:\n{}",
        s,
        vert_mirror(s.clone()),
        hor_mirror(s.clone())
    );
    println!("{}", part_list(vec!["I", "wish", "I", "hadn't", "come"]));
}

fn part_list(arr: Vec<&str>) -> String {
    let mut result = String::new();
    for i in 1..arr.len() {
        let (left, right) = arr.split_at(i);
        result.push_str(&format!("({}, {})", left.join(" "), right.join(" ")));
    }
    result
}

fn hor_mirror(s: String) -> String {
    s.lines().rev().collect::<Vec<_>>().join("\n")
}
fn vert_mirror(s: String) -> String {
    s.lines()
        .map(|l| l.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn oper(op: fn(String) -> String, s: String) -> String {
    op(s)
}

fn play_pass(s: &str, n: u32) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let translated_string: String = s
        .chars()
        .map(|c| {
            if let Some(i) = alphabet.find(c.to_ascii_lowercase()) {
                alphabet
                    .chars()
                    .nth((i + n as usize) % alphabet.len())
                    .unwrap()
            } else {
                match c.to_digit(10) {
                    Some(d) => std::char::from_digit(9 - d, 10).unwrap(),
                    None => c,
                }
            }
        })
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    translated_string.chars().rev().collect()
}

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.iter()
        .map(|t| match (t.0 >= 55, t.1 > 7) {
            (true, true) => String::from("Senior"),
            _ => String::from("Open"),
        })
        .collect()
}

fn nb_year_rounding_errors(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let percent = percent / 100.0;
    let aug = aug as f64;
    let p = p as f64;
    let p0 = p0 as f64;
    let temp = aug / percent + p0;
    ((1.0 + (p - p0) / temp).ln() / (1.0 + percent).ln()).ceil() as i32
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut pop = p0;
    let mut years = 0;
    let percent = 1.0 + percent / 100.0;
    while pop < p {
        pop = (pop as f64 * percent) as i32 + aug;
        years += 1;
    }
    years
}

fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|c| "aeiou".contains(|v| &v == c))
        .count()
}

fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut result = "".to_string(); // your code
    if !list_art.is_empty() {
        for (index, category) in list_cat.iter().enumerate() {
            if index >= 1 {
                result = format!("{} - ", result);
            }
            result = format!("{}{}", result, format_category(category, &list_art));
            println!("{}", category);
        }
    }
    result
}

fn parse_stock(stock: &str) -> (&str, u32) {
    let t: Vec<&str> = stock.split_whitespace().collect();
    (t[0], t[1].parse::<u32>().unwrap())
}

fn count_category(cat: &str, list_art: &Vec<&str>) -> u32 {
    let mut count = 0;
    for stock in list_art.iter().map(|s| parse_stock(s)) {
        if stock.0.starts_with(cat) {
            count += stock.1;
        }
    }
    count
}

fn format_category(cat: &str, list_art: &Vec<&str>) -> String {
    format!("({} : {})", cat, count_category(cat, list_art))
}
