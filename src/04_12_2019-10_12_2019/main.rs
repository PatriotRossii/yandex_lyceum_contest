use std::str::FromStr;

/*
Геолог Борис работает на умном экскаваторе. После извлечения кубометра
горной породы экскаватор проводит анализ и передаёт строку из N чисел,
в которой записано, сколько в излечённом кубометре ценных элементов
A1, A2, ..., AN. Последнее число в строке — процент примесей в породе,
на который уменьшается содержание каждого ценного элемента.
Борис за день извлёк M кубометров породы. После рабочего дня он идёт к
подрядчику и получает номер ценного элемента, за который подрядчик готов
дать премию, равную количеству добытого элемента за день. Помогите Борису
определить размер премии.
*/

#[derive(Debug)]
pub enum ReadError {
    FailedToRead,
    FailedToParse,
}

pub fn parse_read<T>() -> Result<T, ReadError>
where
    T: FromStr,
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
    let cubometers: usize = parse_read().expect("Please, enter valid value");
    let mut minerals: Vec<Vec<f64>> = Vec::with_capacity(cubometers);

    for _ in 0..cubometers {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let mut mineral = Vec::new();
        for element in buffer.trim().split(' ') {
            mineral.push(element.parse::<f64>().expect("Please, enter valid number"));
        }

        let impurity = mineral.pop().unwrap();
        for mineral in &mut mineral {
            *mineral -= *mineral * (impurity / 100.0);
        }

        minerals.push(mineral);
    }

    let nth: usize = parse_read().expect("Please, enter valid value");
    let mut result: f64 = 0_f64;

    for mineral in &minerals {
        if let Some(e) = mineral.get(nth - 1) {
            result += *e;
        }
    }

    println!("{}", result);
}
