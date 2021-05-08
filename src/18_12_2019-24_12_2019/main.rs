/*
В школе N классов с номерами от 1 до N, в каждом из которых M учеников.
Однажды перед уроками все ученики школы выстроились в линейку. Однако в
каждом классе нашлись опоздавшие, которые, чтобы быть менее заметными, по
мере подхода к линейке занимали в ней места не рядом, а через промежутки,
соответствующие номеру своего класса. Например, все опоздавшие из 3 класса
вставали на места с номерами 3, 6, 9 и так далее, при этом всем остальным
приходилось сдвигаться вправо (гарантируется, что это всегда возможно).
Первыми к линейке подошли опоздавшие из 1 класса, дальше из второго и так
далее. После того, как все опоздавшие заняли свои места в линейке, к ней
вышел директор школы, выбрал каждого K-го ученика начиная с конца и увёл к
себе в кабинет для воспитательной беседы. Определите, сколько опоздавших из
каждого класса оказалось в кабинете директора.
*/

#[derive(Debug)]
pub enum Student {
    Normal(usize),
    Antisocial(usize),
}

pub fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read a line");

    let mut values = buffer
        .trim()
        .split(' ')
        .into_iter()
        .map(|x| x.parse::<usize>().expect("Please, enter valid number"));

    let classes = values.next().unwrap();
    let students_per_class = values.next().unwrap();
    let decimation_factor = values.next().unwrap();

    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read a line");
    let antisocial_per_class: Vec<usize> = buffer
        .trim()
        .split(' ')
        .into_iter()
        .map(|x| x.parse::<usize>().expect("Please, enter valid number"))
        .collect();

    let mut queue: Vec<Student> = (0..classes)
        .into_iter()
        .map(|x| {
            std::iter::once(x)
                .cycle()
                .take(students_per_class - antisocial_per_class[x])
        })
        .flatten()
        .map(Student::Normal)
        .collect();
    for (i, antisocial_count) in antisocial_per_class.iter().enumerate() {
        for j in 0..*antisocial_count {
            queue.insert((i + 1) * (j + 1) - 1, Student::Antisocial(i));
        }
    }

    let mut result: Vec<usize> = std::iter::once(0_usize).cycle().take(classes).collect();
    let lucky_guys = queue
        .iter()
        .rev()
        .skip(decimation_factor - 1)
        .step_by(decimation_factor);

    for e in lucky_guys {
        if let &Student::Antisocial(i) = e {
            result[i] += 1;
        }
    }

    println!(
        "{}",
        result
            .iter()
            .map(usize::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}
