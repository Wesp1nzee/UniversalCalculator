use std::io;

fn main() {
    loop {
        println!("Вы находитесь в меню. Выберите действие:");
        println!("1. Конвертация из десятичной системы счисления в любую другую от 2 до 16.");
        println!("2. Конвертация в десятичную систему счисления из любой другой с основанием от 2 до 16.");
        println!("3. Выйти");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Не удалось прочитать основание");
        let choice: u32 = choice.trim()
                            .parse()
                            .expect("Введите корректное число");
        match choice {
            1 => {
                let mut base_input = String::new();
                println!("Введите основание системы счислений:");
                io::stdin()
                    .read_line(&mut base_input)
                    .expect("Не удалось прочитать основание");

                let base_number: i8 = match base_input.trim().parse() {
                    Ok(n) => {
                        if 2 <= n && n <= 16 {
                            println!("Отлично, вы выбрали основание {}", n);
                            n
                        }
                        else {
                            println!("Основание не может быть меньше 2 и больеш 16");
                            return;
                        }
                    },
                    Err(_) => {
                        eprintln!("Ошибка, неверный формат основания");
                        return;
                    }
                };

                let mut number_input = String::new();
                println!("Введите число в десятичной системе:");
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
            },
            2 => {
                continue; // TODO: Дописать второе действие.
            },
            3 => {
                break;
            },
            _ => {
                println!("Вы выбрали некорректное число.");
                continue;
            }
        }
    }
}

fn calculate_universal(base: i8, mut number: isize) -> String {
    let mut result:String  = String::new();
    loop {
        if number <= 0 {
            break;
        }
        let char_base: i8 = (number % base as isize) as i8;
        match char_base {
            0..=9 => result.push_str(&char_base.to_string()),
            10 => result.push('A'),
            11 => result.push('B'),
            12 => result.push('C'),
            13 => result.push('D'),
            14 => result.push('E'),
            15 => result.push('F'),
            _ => {continue}          
        };
        number /= base as isize
    }
    result.chars().rev().collect()
}