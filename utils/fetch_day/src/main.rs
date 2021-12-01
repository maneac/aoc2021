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

    add_ts_template(&output, day);
    add_go_template(&output, day);
    add_rs_template(&output, day);

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

fn add_ts_template(readme: &str, day: usize) {
    let lang_instruction_dir = Path::new("ts").join(format!("day_{}", &day));

    if !lang_instruction_dir.exists() {
        std::fs::create_dir_all(&lang_instruction_dir).unwrap();
    }
    write(lang_instruction_dir.join("README.md"), &readme).unwrap();

    write(
        lang_instruction_dir.join("main.ts"),
        r#"export function readData() {
    throw new Error("unimplemented");
}

export function part1(_data: any) {
    throw new Error("unimplemented");
}

export function part2(_data: any) {
    throw new Error("unimplemented");
}

export function main() {
    const data = readData();
  
    console.log("Part 1: ", part1(data));
    console.log("Part 2: ", part2(data));
}"#,
    )
    .unwrap();

    write(
        lang_instruction_dir.join("main.test.ts"),
        r#"import { assertEquals } from "https://deno.land/std@0.116.0/testing/asserts.ts";
import { part1, part2, readData } from "./main.ts";

Deno.test("part 1 real", () => {
    const input = readData();

    assertEquals(part1(input), 0);
});

Deno.test("part 2 real", () => {
    const input = readData();

    assertEquals(part2(input), 0);
});"#,
    )
    .unwrap();
}

fn add_go_template(readme: &str, day: usize) {
    let lang_instruction_dir = Path::new("go").join(format!("day_{}", &day));

    if !lang_instruction_dir.exists() {
        std::fs::create_dir_all(&lang_instruction_dir).unwrap();
    }
    write(lang_instruction_dir.join("README.md"), &readme).unwrap();

    write(
        lang_instruction_dir.join("main.go"),
        r#"package main

import "log"

func main() {
    data := readData()

    log.Println("Part 1: ", part1(data))
    log.Println("Part 2: ", part2(data))
}
        
func readData() []string {
    return nil
}

func part1(input []string) string {
    panic("unimplemented")
}

func part2(input []string) string {
    panic("unimplemented")
}"#,
    )
    .unwrap();

    write(
        lang_instruction_dir.join("main_test.go"),
        r#"package main

import "testing"

func TestPart1Real(t *testing.T) {
    data := readData()

    expected := ""
    actual := part1(data)

    if actual != expected {
        t.Fatalf("Expected: %v\nActual: %v", expected, actual)
    }
}

func TestPart2Real(t *testing.T) {
    data := readData()

    expected := ""
    actual := part2(data)

    if actual != expected {
        t.Fatalf("Expected: %v\nActual: %v", expected, actual)
    }
}
"#,
    )
    .unwrap();
}

fn add_rs_template(readme: &str, day: usize) {
    let lang_instruction_dir = Path::new("rs").join(format!("day_{}", &day));

    if !lang_instruction_dir.exists() {
        std::fs::create_dir_all(&lang_instruction_dir.join("src")).unwrap();
    }
    write(lang_instruction_dir.join("README.md"), &readme).unwrap();

    write(
        lang_instruction_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "day_{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
            day
        ),
    )
    .unwrap();

    write(
        lang_instruction_dir.join("src").join("main.rs"),
        r#"fn main() {
    let data = read_data();

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data() -> Vec<String> {
    todo!()
}

fn part_1(_input: &Vec<String>) -> String {
    todo!()
}

fn part_2(_input: &Vec<String>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_real() {
        let data = read_data();

        assert_eq!("", part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data();

        assert_eq!("", part_2(&data));
    }
}
"#,
    )
    .unwrap();
}
