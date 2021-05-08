/*
На олимпиаду по программированию приходит N человек. На регистрации вместо
своего имени они передают организатору шифр — строку, состоящую из букв и
цифр. Гарантируется, что в этой строке содержится единственная
последовательность цифр, образующая целое положительное число M. Если
символы шифра, стоящие на позициях кратных M (нумерация с единицы),
составляют корректное имя участника (регистр не учитывается), он может
пройти на олимпиаду. Желающих попасть на олимпиаду достаточно много,
поэтому участники могут шифровать одно и то же имя несколько раз — в таком
случае проходит только первый из них. Помогите организаторам определить,
кого из пришедших участников нужно пустить на олимпиаду.
*/

use std::collections::HashMap;

pub fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read a line");
    let mut values = buffer
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().expect("Please, enter valid number"));

    let participants = values.next().unwrap();
    let comes = values.next().unwrap();

    let mut names: HashMap<String, String> = HashMap::with_capacity(participants);
    for _ in 0..participants {
        let mut name = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read a line");
        let name = name.trim().to_string();

        names.insert(name.to_lowercase(), name);
    }

    let mut result: Vec<String> = Vec::new();
    for _ in 0..comes {
        let mut cipher = String::new();
        std::io::stdin()
            .read_line(&mut cipher)
            .expect("Failed to read a line");

        let trimmed = cipher.trim();

        let factor: usize = trimmed
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .unwrap();
        let decoded: String = trimmed
            .chars()
            .skip(factor - 1)
            .step_by(factor)
            .collect::<String>()
            .to_lowercase();

        let contains = names.contains_key(&decoded);
        if contains {
            result.push(names[&decoded].clone());
            names.remove(&decoded);
        }
    }

    let output = result.join("\n");
    println!("{}", if result.is_empty() { "NO" } else { &output });
}
