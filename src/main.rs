use std::io;

fn main() {
    let mut base_input = String::new();
    println!("Введите основание системы счислений:");
    io::stdin()
        .read_line(&mut base_input)
        .expect("Не удалось прочитать основание");

    let base_number: i8 = match base_input.trim().parse() {
        Ok(n) => {
            println!("Отлично, вы выбрали основание {}", n);
            n
        }
        Err(_) => {
            eprintln!("Ошибка, неверный формат основания");
            return;
        }
    };

    let mut number_input = String::new();
    println!("Введите число в системе с основанием {}:", base_number);
    io::stdin()
        .read_line(&mut number_input)
        .expect("Не удалось прочитать число");

    let number: isize = match number_input.trim().parse() {
        Ok(n) => {
            println!("Отлично, вы выбрали число {}", n);
            n
        }
        Err(_) => {
            eprintln!("Ошибка, неверный формат числа");
            return;
        }
    };

    let result: String = calculate_universal(base_number, number);
    println!("\nРезультат конвертации: {}", result);
}

fn calculate_universal(base: i8, mut number: isize) -> String {
    let mut result:String  = String::new();
    loop {
        if number <= 0 {
            break;
        }
        let char_base: i64 = (number % base as isize) as i64;
        result.push_str(&char_base.to_string());
        number /= base as isize
    }
    result.chars().rev().collect()
}