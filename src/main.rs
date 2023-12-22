use std::io;
use std::fs;
use std::path::Path;
use indoc::indoc;

fn capitalize(text: &str) -> String {
    if let Some(first_char) = text.chars().next() {
        let capitalized = first_char.to_uppercase().to_string();
        let mut result = capitalized;
        result.push_str(&text[1..]);
        result
    } else {
        String::new()
    }
}

fn main() {
    println!("Enter the name of the file");
    let mut arg = String::new();

    io::stdin().read_line(&mut arg).expect("Failed to read");

    let path = format!("src/components/{}", arg.trim());
    if !Path::new(&path).exists() {
        fs::create_dir_all(&path).expect("Failed to create directory");
    } else {
        println!("Directory already exists.");
    }

    let index_path = format!("src/components/{}/index.tsx", arg.trim());
    if !Path::new(&index_path).exists() {
        let file_content = indoc!(
            "
            import React from 'react'
            import * as S from './styles'

            export const {arg} = () => {{
                return (<></>)
            }}
        "
        );

        let s = file_content.replace("{arg}", &capitalize(arg.trim()));

        fs::write(&index_path, s).expect("Failed to write index file");
    } else {
        println!("Index file already exists.");
    }

    let style_path = format!("src/components/{}/style.ts", arg.trim());
    if !Path::new(&style_path).exists() {
        fs::File::create(&style_path).expect("Failed to create style file");
    } else {
        println!("Style file already exists.");
    }
}
