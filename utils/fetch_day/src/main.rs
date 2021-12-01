use regex::Regex;
use reqwest::Error;
use std::{
    env::args,
    fs::{read_to_string, remove_file, write},
    iter::Peekable,
    path::Path,
    str::Chars,
};

fn main() {
    let token = env!("AOC_SESSION_TOKEN");

    let day = args().last().unwrap().parse::<usize>().unwrap();

    let day_url = format!("https://adventofcode.com/2021/day/{}", day);

    let instruction_file = Path::new("instructions.html");
    let instructions_html = if !instruction_file.exists() {
        let instructions = retrieve_instructions(token, &day_url).unwrap();
        write(instruction_file, &instructions).unwrap();
        instructions
    } else {
        read_to_string(instruction_file).unwrap()
    };

    let parts = Regex::new(r"(?s)<article.*?>(.+?)</article>")
        .unwrap()
        .captures_iter(&instructions_html)
        .map(|caps| caps.get(1).unwrap().as_str())
        .collect::<Vec<&str>>();

    let output = parts
        .iter()
        .map(|part| {
            let mut output = String::new();
            let mut iter = part.chars().peekable();
            while iter.peek().is_some() {
                let out = recursive_parse(&day_url, &mut iter);
                output.push_str(&out);
            }
            output
        })
        .collect::<Vec<String>>()
        .join("\n");

    for lang in ["go", "ts", "rs"] {
        let lang_instruction_file = Path::new(lang)
            .join(format!("day_{}", &day))
            .join("README.md");
        if !lang_instruction_file.parent().unwrap().exists() {
            std::fs::create_dir_all(&lang_instruction_file.parent().unwrap()).unwrap();
        }
        write(lang_instruction_file, &output).unwrap();
    }

    let data_file = Path::new("data").join(format!("day_{}.txt", &day));
    if !data_file.exists() {
        let data = download_input(token, &day_url).unwrap();
        write(data_file, data).unwrap();
    }

    remove_file(instruction_file).unwrap();
}

fn retrieve_instructions(token: &str, day_url: &str) -> Result<String, Error> {
    let c = reqwest::blocking::Client::new();
    let req = c
        .get(day_url)
        .header("Cookie", format!("session={}", token))
        .build()
        .unwrap();
    c.execute(req)?.text()
}

fn download_input(token: &str, day_url: &str) -> Result<String, Error> {
    let c = reqwest::blocking::Client::new();
    let req = c
        .get(format!("{}/input", day_url))
        .header("Cookie", format!("session={}", token))
        .build()
        .unwrap();
    c.execute(req)?.text()
}

fn recursive_parse<'a>(day_url: &str, input: &mut Peekable<Chars<'a>>) -> String {
    let raw_tag = input
        .take_while(|c| c.ne(&'>'))
        .collect::<Vec<char>>()
        .iter()
        .take_while(|&c| c.ne(&' '))
        .collect::<String>();

    let mut tag = raw_tag.trim();
    if let Some(t) = tag.strip_prefix("<") {
        tag = t;
    }

    let mut output = String::new();

    match tag {
        "h2" => output.push_str("\n## "),
        "em" => output.push_str("**"),
        "code" => output.push_str("`"),
        "pre" => output.push_str("<pre>"),
        "p" => output.push_str("\n\n"),
        _ => {}
    }

    loop {
        match match input.next() {
            Some(v) => v,
            None => return output,
        } {
            '<' => {
                if let Some('/') = input.peek() {
                    let _ = input.take_while(|c| c.ne(&'>')).collect::<String>();
                    break;
                }
                output.push_str(&recursive_parse(day_url, input));
            }
            '>' => break,
            v => {
                output.push(v);
            }
        }
    }

    match tag {
        "h2" if output.contains(" --- Day") => {
            output = output.replace("\n## ", "# [");
            output.push_str(format!("]({})", day_url).as_str());
        }
        "em" => output.push_str("**"),
        "code" => output.push_str("`"),
        "pre" => {
            output = output
                .replace("<pre>`", "<pre><code>")
                .replace("`", "</code>");
            output = output
                .split("\n")
                .map(|line| line.trim())
                .collect::<Vec<&str>>()
                .join("\n");
            output.push_str("</pre>")
        }
        _ => {}
    }

    output
}
