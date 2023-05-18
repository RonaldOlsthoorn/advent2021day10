use std::collections::VecDeque;
use std::io::{BufReader, BufRead};
use std::fs::File;

enum Chunk {
    Round(Vec<Chunk>),
    Square(Vec<Chunk>),
    Curly(Vec<Chunk>),
    Diamond(Vec<Chunk>)
}

impl Chunk {

    const ERROR_SCORE_ROUND: u16 = 3;
    const ERROR_SCORE_SQUARE: u16 = 57;
    const ERROR_SCORE_CURLY: u16 = 1197;
    const ERROR_SCORE_DIAMOND: u16 = 25137;


    fn parse(line: &String) -> Result<usize, u16> {

        let mut opening_stack: VecDeque<(Chunk, Vec<Chunk>)> = VecDeque::new();

        for c in line.chars() {

            match c {
                '(' => opening_stack.push_front((Chunk::Round(vec![]), vec![])),
                '[' => opening_stack.push_front((Chunk::Square(vec![]), vec![])),
                '{' => opening_stack.push_front((Chunk::Curly(vec![]), vec![])),
                '<' => opening_stack.push_front((Chunk::Diamond(vec![]), vec![])),
                ')' => {
                    let (mut top_chunk, cache) = opening_stack.pop_front().ok_or(Chunk::ERROR_SCORE_ROUND)?;

                    if let Chunk::Round(ref mut content) = top_chunk {
                        *content = cache;
                    } else {
                        return Err(Chunk::ERROR_SCORE_ROUND);
                    }

                    if let Some((_, parent_content)) = opening_stack.front_mut() {
                        parent_content.push(top_chunk);
                    } else {
                        return Ok(0);
                    }
                },
                ']' => {
                    let (mut top_chunk, cache) = opening_stack.pop_front().ok_or(Chunk::ERROR_SCORE_SQUARE)?;

                    if let Chunk::Square(ref mut content) = top_chunk {
                        *content = cache;
                    } else {
                        return Err(Chunk::ERROR_SCORE_SQUARE);
                    }

                    if let Some((_, parent_content)) = opening_stack.front_mut() {
                        parent_content.push(top_chunk);
                    } else {
                        return Ok(0);
                    }
                },
                '}' => {
                    let (mut top_chunk, cache) = opening_stack.pop_front().ok_or(Chunk::ERROR_SCORE_CURLY)?;

                    if let Chunk::Curly(ref mut content) = top_chunk {
                        *content = cache;
                    } else {
                        return Err(Chunk::ERROR_SCORE_CURLY);
                    }

                    if let Some((_, parent_content)) = opening_stack.front_mut() {
                        parent_content.push(top_chunk);
                    } else {
                        return Ok(0);
                    }
                },
                '>' => {
                    let (mut top_chunk, cache) = opening_stack.pop_front().ok_or(Chunk::ERROR_SCORE_DIAMOND)?;

                    if let Chunk::Diamond(ref mut content) = top_chunk {
                        *content = cache;
                    } else {
                        return Err(Chunk::ERROR_SCORE_DIAMOND);
                    }

                    if let Some((_, parent_content)) = opening_stack.front_mut() {
                        parent_content.push(top_chunk);
                    } else {
                        return Ok(0);
                    }
                },
                _ => panic!("Illegal character in line"),
                
            }
        }

        let mut res = 0;

        while !opening_stack.is_empty() {
            let (new_crunch, _) = opening_stack.pop_front().unwrap();

            match new_crunch {
                Chunk::Round(_) => res = (res * 5) + 1,
                Chunk::Square(_) => res = (res * 5) + 2,
                Chunk::Curly(_) => res = (res * 5) + 3,
                Chunk::Diamond(_) => res = (res * 5) + 4,
            }
        }

        return Ok(res);
    }
}

fn main() {

    let lines = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap());

    let mut scores: Vec<usize> = lines
    .map(|l| Chunk::parse(&l))
    .filter_map(|r| r.ok()).collect();

    scores.sort();

    println!("middle score {}", scores[scores.len() / 2]);
}
