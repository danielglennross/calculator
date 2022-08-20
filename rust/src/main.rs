use std::borrow::{Borrow};
use std::collections::{HashMap};

#[derive(Clone)]
enum Expression {
    Binary(Vec<Box<Option<Binary>>>),
    Value(i32)
}

#[derive(Clone)]
struct Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    op: String,
}

fn main() {
    let input = "1 + 2 * (3 + 4) - 5 * (6 * (7 + 8))".to_owned();
    let result = calculate(input.clone());

    println!("{} = {}", input, result);
}

fn calculate(input: String) -> i32 {
    let mut char_index = -1i32;
    let mut token_map = HashMap::<String, Vec<String>>::new();
    let mut next_token = token_generator();

    let chars = to_chars(input);
    let tokenized_chars = tokenize_brackets(&mut char_index, &chars, &mut token_map, &mut next_token);

    let groups = create_binary_groups(&tokenized_chars, &mut token_map);
    let result = build_compute_from(
        vec![
            compute_binary("*".to_owned()),
            compute_binary("/".to_owned()),
            compute_binary("+".to_owned()),
            compute_binary("-".to_owned())
        ]
    )(groups);

    result
}

fn build_compute_from(compute: Vec<Box<dyn Fn(Vec<Box<Option<Binary>>>) -> Vec<Box<Option<Binary>>>>>) -> Box<dyn Fn(Vec<Box<Option<Binary>>>) -> i32> {
    Box::new(move |ops: Vec<Box<Option<Binary>>>| {
        let r = compute.iter().fold(ops, |c, a|
            if c.len() > 1 { a(c) } else { c }
        );
        eval_binary(r.into_iter().nth(0).unwrap().unwrap())
    })
}

fn compute_binary(symbol: String) -> Box<dyn Fn(Vec<Box<Option<Binary>>>) -> Vec<Box<Option<Binary>>>> {
    let maybe_set_prev_binary = |l: &mut usize, ops: &mut Vec<Box<Option<Binary>>>, res: i32| {
        while *l > 0usize {
            match *ops[*l-1] {
                Some(ref mut left_side) => {
                    left_side.right = Box::new(Expression::Value(res));
                    break;
                },
                None => {}
            }
            *l -= 1;
        }
    };

    let maybe_set_next_binary = |r: &mut usize, ops: &mut Vec<Box<Option<Binary>>>, res: i32| {
        while *r+1 < ops.len() {
            match *ops[*r+1] {
                Some(ref mut right_side) => {
                    right_side.left = Box::new(Expression::Value(res));
                    break;
                }
                None => {}
            }
            *r += 1;
        }
    };

    Box::new(move |mut ops: Vec<Box<Option<Binary>>>| {
        for i in 0..ops.len() {
            let is_single_binary_remaining = ops.iter()
                .filter(|v| v.is_some())
                .count() == 1;
            if is_single_binary_remaining {
                break
            }

            let item = ops[i].clone().unwrap();

            if item.op != symbol {
                continue;
            }

            let res = eval_binary(item.clone());
            ops[i] = Box::new(None);

            maybe_set_prev_binary(&mut i.clone(), &mut ops, res);
            maybe_set_next_binary(&mut i.clone(), &mut ops, res);
        }

        return ops.into_iter()
            .filter(|v| v.is_some())
            .map(|v| Box::new(Option::Some(v.unwrap())))
            .collect();
    })
}

fn eval_binary(b: Binary) -> i32 {
    let compute = build_compute_from(
        vec![
            compute_binary("*".to_owned()),
            compute_binary("/".to_owned()),
            compute_binary("+".to_owned()),
            compute_binary("-".to_owned())
        ]
    );

    let l = match *b.left {
        Expression::Binary(bin) => compute(bin),
        Expression::Value(v) => v
    };

    let r = match *b.right {
        Expression::Binary(bin) => compute(bin),
        Expression::Value(v) => v
    };

    match b.op.borrow() {
        "*" => l * r,
        "/" => l / r,
        "+" => l + r,
        "-" => l - r,
        _ => 0
    }
}

fn create_binary_groups(chars: &Vec<String>, token_map: &mut HashMap<String, Vec<String>>) -> Vec<Box<Option<Binary>>> {
    chars.into_iter().enumerate().fold(Vec::<Box<Option<Binary>>>::new(), | mut acc, (i, c)| {
        match c.borrow() {
            "*" | "/" | "+" | "-" => {
                let left = match chars[i-1].parse::<i32>() {
                    Ok(v) => Expression::Value(v),
                    _ => {
                        let (_, v) = token_map.get_key_value(&chars[i-1]).unwrap();
                        let l = create_binary_groups(v, &mut token_map.clone());
                        Expression::Binary(l)
                    }
                };
                let right = match chars[i+1].parse::<i32>() {
                    Ok(v) => Expression::Value(v),
                    _ => {
                        let (_, v) = token_map.get_key_value(&chars[i+1]).unwrap();
                        let r = create_binary_groups(v, &mut token_map.clone());
                        Expression::Binary(r)
                    }
                };
                acc.push(Box::new(Option::Some(Binary {
                    left: Box::new(left),
                    right: Box::new(right),
                    op: c.to_owned()
                })));
                acc
            }
            _ => acc,
        }
    })
}

fn tokenize_brackets(char_index: &mut i32, chars: &Vec<char>, tokens: &mut HashMap<String, Vec<String>>, next_token: &mut Box<dyn FnMut() -> String>) -> Vec<String> {
    let mut chars_with_tokens = Vec::default();
    let mut i = 0usize;
    while i < chars.len() {
        *char_index += 1;
        let c = chars[i];
        match c {
            ')' => {
                return chars_with_tokens
            },
            '(' => {
                let k = next_token();
                let t = tokenize_brackets(char_index, &chars[i+1..].to_vec(), tokens, next_token);
                tokens.insert(k.to_owned(), t);
                chars_with_tokens.push(k);
                i = *char_index as usize + 1
            },
            _ => {
                chars_with_tokens.push(c.to_string());
                i += 1
            }
        }
    }
    chars_with_tokens
}

fn token_generator() -> Box<dyn FnMut() -> String> {
    let (tokens, mut i, mut r) = ("abcdefghijklmnopqrstuvwxyz", 0usize, 0usize);
    Box::new(move || {
        if i%tokens.len() == 0 {
            i = 0;
            r += 1;
        }
        let t = tokens.chars().nth(i).unwrap();
        i += 1;
        t.to_string().repeat(r)
    })
}

fn to_chars(input: String) -> Vec<char> {
    input.chars().filter(|c| *c != ' ').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! calculator_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let result = calculate(input.to_string());
                    assert_eq!(expected, result);
                }
            )*
        }
    }

    calculator_tests! {
        simple_1: ("1 + 2 * 3 + 4 - 5 * 6 + 7", -26),
        simple_2: ("1 + 2 - 3 * 4 * 5 - 6 + 7", -70),
        simple_3: ("1 * 2 + 3 + 4 - 5 - 6 * 7", -38),
        brackets_1: ("1 + 2 * (3 + 4) - 5 * (6 * (7 + 8))", -435),
    }
}

