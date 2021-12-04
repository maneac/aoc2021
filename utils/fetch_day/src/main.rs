#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use chrono::{Datelike, Utc};
use clap::Parser;
use regex::Regex;
use reqwest::Error;
use std::{
    fs::{create_dir_all, read_to_string, remove_file, write},
    iter::Peekable,
    path::Path,
    str::Chars,
};

#[derive(clap::Parser, Debug)]
struct Opts {
    #[clap(
        short,
        long = "day",
        value_name = "DAY",
        possible_values = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25"],
        about = "Day to download the instructions and input for (defaults to the min(current day, 25) in EST)"
    )]
    day_opt: Option<usize>,

    #[clap(default_value = "0")]
    day: usize,

    #[clap(
        short = 'f',
        long = "download",
        about = "Force the download of the instructions"
    )]
    force_download: bool,

    #[clap(long, about = "Skip the downloading of the input data")]
    no_data: bool,

    #[clap(
        short,
        long,
        use_delimiter = true,
        possible_values = ["go", "rs", "ts", ""],
        default_value = "go,rs,ts",
        about = "Languages to create instructions and templates for"
    )]
    langs: Option<Vec<String>>,

    #[clap(long, about = "Skip code template creation for each language")]
    skip_templates: bool,

    #[clap(long, about = "Keep the raw instruction HTML file")]
    keep_instructions: bool,

    #[clap(
        long,
        about = "Update the READMEs to contain part 2. Alias for '--download --no-data --skip-templates'"
    )]
    part_2: bool,
}

fn main() {
    let opts = {
        let mut opts = Opts::parse();
        opts.day = opts.day_opt.unwrap_or(
            Utc::now()
                .with_timezone(&chrono::offset::FixedOffset::west(5 * 3600)) // EST
                .day()
                .min(25) as usize,
        );

        if opts.part_2 {
            opts.force_download = true;
            opts.no_data = true;
            opts.skip_templates = true;
        }
        opts
    };
    let day = opts.day;

    let token = env!("AOC_SESSION_TOKEN");

    let day_url = format!("https://adventofcode.com/2021/day/{}", day);

    let instruction_file = Path::new("instructions.html");
    let instructions_html = if opts.force_download || !instruction_file.exists() {
        let instructions = retrieve_instructions(token, &day_url).unwrap();
        if opts.keep_instructions {
            write(instruction_file, &instructions).unwrap();
        }
        instructions
    } else {
        let instructions = read_to_string(instruction_file).unwrap();
        if !opts.keep_instructions {
            remove_file(instruction_file).unwrap();
        }
        instructions
    };

    let parts = Regex::new(r"(?s)<article.*?>(.+?)</article>")
        .unwrap()
        .captures_iter(&instructions_html)
        .map(|caps| caps.get(1).unwrap().as_str())
        .collect::<Vec<&str>>();

    let readme_contents = parts
        .iter()
        .map(|part| {
            let mut output = String::new();
            let mut iter = part.chars().peekable();
            while iter.peek().is_some() {
                let out = recursive_parse(&day_url, &mut iter, false);
                output.push_str(&out);
            }
            output
        })
        .collect::<Vec<String>>()
        .join("\n");

    if let Some(langs) = &opts.langs {
        langs.iter().for_each(|lang| match lang.as_str() {
            "ts" => add_ts_template(&opts, &readme_contents, day),
            "go" => add_go_template(&opts, &readme_contents, day),
            "rs" => add_rs_template(&opts, &readme_contents, day),
            _ => panic!("unknown language: {}", &lang),
        });
    }

    if !opts.no_data {
        let data_file = Path::new("data").join(format!("day_{}.txt", &day));
        if !data_file.exists() {
            let data = download_input(token, &day_url).unwrap();
            write(data_file, data).unwrap();
        }
    }
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

fn recursive_parse<'a>(
    day_url: &str,
    input: &mut Peekable<Chars<'a>>,
    is_preformatted: bool,
) -> String {
    let whole_tag = input.take_while(|c| c.ne(&'>')).collect::<String>();

    let raw_tag = whole_tag
        .chars()
        .take_while(|&c| c.ne(&' '))
        .collect::<String>();

    let mut tag = raw_tag.trim();
    if let Some(t) = tag.strip_prefix('<') {
        tag = t;
    }

    let mut pre = is_preformatted;
    let mut output = String::new();

    match tag {
        "h2" => output.push_str("\n## "),
        "em" if is_preformatted => output.push_str("<b>"),
        "code" if is_preformatted => output.push_str("<code>"),
        "em" => output.push_str("**"),
        "code" => output.push('`'),
        "pre" => {
            output.push_str("\n\n<pre>");
            pre = true
        }
        "p" => output.push_str("\n\n"),
        "ul" => output.push('\n'),
        "li" => output.push_str("  - "),
        "a" => output.push('['),
        "span" => {}
        "" if input.peek().is_none() => {}
        _ => {
            panic!("unknown tag: {}", tag)
        }
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
                output.push_str(&recursive_parse(day_url, input, pre));
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
        "em" if is_preformatted => output.push_str("</b>"),
        "code" if is_preformatted => output.push_str("</code>"),
        "em" => output.push_str("**"),
        "code" => output.push('`'),
        "pre" => output.push_str("</pre>"),
        "a" => {
            let link = Regex::new(r#"href="(.+?)""#)
                .unwrap()
                .captures(&whole_tag)
                .map(|caps| caps.get(1).unwrap().as_str())
                .unwrap();
            output.push_str(&format!("]({})", link));
        }
        "p" | "span" | "h2" | "ul" | "li" => {}
        _ => {
            panic!("unknown tag: {}", tag)
        }
    }

    output = output.trim_end().to_string();

    // hack to ensure emphasised code blocks have the correct operation order
    if output.starts_with('`') && output.contains("**") {
        output = format!("**{}**", &output.replace("**", ""));
    }

    output
}

fn add_ts_template(opts: &Opts, readme: &str, day: usize) {
    let lang_instruction_dir = Path::new("ts").join(format!("day_{}", &day));

    if !lang_instruction_dir.exists() {
        create_dir_all(&lang_instruction_dir).unwrap();
    }
    write(lang_instruction_dir.join("README.md"), &readme).unwrap();

    if opts.skip_templates {
        return;
    }

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

fn add_go_template(opts: &Opts, readme: &str, day: usize) {
    let lang_instruction_dir = Path::new("go").join(format!("day_{}", &day));

    if !lang_instruction_dir.exists() {
        create_dir_all(&lang_instruction_dir).unwrap();
    }
    write(lang_instruction_dir.join("README.md"), &readme).unwrap();

    if opts.skip_templates {
        return;
    }

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

func TestPart1(t *testing.T) {
	tests := map[string]struct {
		data     []string
		expected string
	}{
		"actual": {
			data:     readData(),
			expected: "",
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := part1(test.data)

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := map[string]struct {
		data     []string
		expected string
	}{
		"actual": {
			data:     readData(),
			expected: "",
		},
	}

	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			actual := part2(test.data)

			if actual != test.expected {
				t.Fatalf("Expected: %v\nActual: %v", test.expected, actual)
			}
		})
	}
}"#,
    )
    .unwrap();
}

fn add_rs_template(opts: &Opts, readme: &str, day: usize) {
    let lang_instruction_dir = Path::new("rs").join(format!("day_{}", &day));

    if !lang_instruction_dir.exists() {
        create_dir_all(&lang_instruction_dir).unwrap();
    }
    write(lang_instruction_dir.join("README.md"), &readme).unwrap();

    if opts.skip_templates {
        return;
    }

    let src_dir = lang_instruction_dir.join("src");
    if !src_dir.exists() {
        create_dir_all(&src_dir).unwrap();
    }

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
        src_dir.join("main.rs"),
        r#"#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

use std::path::Path;
        
fn main() {
    let data = read_data("./data");

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data(data_dir: &str) -> Vec<String> {
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
        let data = read_data("../../data");

        assert_eq!("", part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data("../../data");

        assert_eq!("", part_2(&data));
    }
}
"#,
    )
    .unwrap();

    write(
        "Cargo.toml",
        format!(
            r#"{}, "rs/day_{}"]
"#,
            read_to_string("Cargo.toml")
                .unwrap()
                .trim_end_matches(&['\n', ']'][..]),
            day
        ),
    )
    .unwrap();
}
