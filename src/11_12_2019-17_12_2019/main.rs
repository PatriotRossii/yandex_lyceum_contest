/*
Узнав на уроке информатики про шифрование данных при помощи открытых ключей,
Вика и Катя решили создать собственный шифр. У Кати есть открытый ключ —
набор из N чисел a1, a2, ..., aN. Вика передаёт ей N текстовых строк.
Расшифровка будет являться словом, составленным из букв переданных строк.
Первая буква должна встречаться в первой строке a1 раз, вторая — во второй
a2 раз, и так далее. Помогите Кате расшифровать слово.
*/

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum ReadError {
    FailedToRead,
    FailedToParse,
}

pub fn parse_read<T>() -> Result<T, ReadError>
where
    T: std::str::FromStr,
{
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|_| ReadError::FailedToParse)?;
    Ok(buffer
        .trim()
        .parse::<T>()
        .map_err(|_| ReadError::FailedToParse)?)
}

pub fn main() {
    let count_of_strings: usize = parse_read().expect("Please, enter valid number");

    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read a line");

    let mut keys = VecDeque::new();
    for key in buffer.trim().split(" ") {
        keys.push_back(key.parse::<usize>().expect("Please, enter valid number"))
    }

    let mut strings: Vec<String> = Vec::new();
    for _ in 0..count_of_strings {
        let mut text = String::new();
        std::io::stdin()
            .read_line(&mut text)
            .expect("Failed to read a line");
        strings.push(text);
    }

    let mut result = String::new();
    for string in &strings {
        let mut count: HashMap<char, usize> = HashMap::new();
        for char in string.trim().chars() {
            count
                .entry(char)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }

        let required_count = keys.pop_front().unwrap();
        for (element, count) in count {
            if count == required_count {
                result.push(element);
            }
        }
    }

    println!("{}", result);
}
