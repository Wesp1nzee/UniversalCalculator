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
                let base_number = match read_number_base_input() {
                    Ok(num) => {num},
                    Err(err) => {
                        eprintln!("{err}");
                        continue;
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

                let result: String = convert_to_base(base_number, number);
                println!("\nРезультат конвертации: {}", result);
            },
            2 => {
                let base_number = match read_number_base_input() {
                    Ok(num) => {num},
                    Err(err) => {
                        eprintln!("{err}");
                        continue;
                    }
                };
                let mut number_input = String::new();
                println!("Введите число с основанием {base_number}");
                io::stdin()
                    .read_line(&mut number_input)
                    .expect("Не удалось прочитать число");
                let result: i64 = convert_to_decimal(base_number as u32, number_input.clone());
                println!("Число {} с основанием {} в в число {} с основанием 10", number_input.trim(), base_number, result)

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

fn read_number_base_input() -> Result<i8, String> {
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
                return Err("Основание должно быть от 2 до 16".to_string());
            }
        },
        Err(_) => {
            return Err("Ошибка, неверный формат основания".to_string());
        }
    };
    Ok(base_number)
}

fn convert_to_base(base: i8, mut number: isize) -> String {
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

fn convert_to_decimal(base: u32, number: String) -> i64 {
    let mut result: i64 = 0;
    let mut valid_digits_found = false;
    
    for ch in number.chars() {
        let digit = match ch.to_digit(base) {
            Some(d) => {
                valid_digits_found = true;
                d as i64
            },
            None => continue,
        };
        result = result * (base as i64) + digit;
    }
    
    if !valid_digits_found {
        0
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_base_binary() {
        assert_eq!(convert_to_base(2, 10), "1010");
        assert_eq!(convert_to_base(2, 0), "");
        assert_eq!(convert_to_base(2, 1), "1");
        assert_eq!(convert_to_base(2, 255), "11111111");
    }

    #[test]
    fn test_convert_to_base_octal() {
        assert_eq!(convert_to_base(8, 10), "12");
        assert_eq!(convert_to_base(8, 0), "");
        assert_eq!(convert_to_base(8, 63), "77");
        assert_eq!(convert_to_base(8, 64), "100");
    }

    #[test]
    fn test_convert_to_base_decimal() {
        assert_eq!(convert_to_base(10, 10), "10");
        assert_eq!(convert_to_base(10, 0), "");
        assert_eq!(convert_to_base(10, 12345), "12345");
    }

    #[test]
    fn test_convert_to_base_hex() {
        assert_eq!(convert_to_base(16, 10), "A");
        assert_eq!(convert_to_base(16, 0), "");
        assert_eq!(convert_to_base(16, 255), "FF");
        assert_eq!(convert_to_base(16, 4095), "FFF");
        assert_eq!(convert_to_base(16, 65535), "FFFF");
    }

    #[test]
    fn test_convert_to_decimal_binary() {
        assert_eq!(convert_to_decimal(2, "1010".to_string()), 10);
        assert_eq!(convert_to_decimal(2, "0".to_string()), 0);
        assert_eq!(convert_to_decimal(2, "11111111".to_string()), 255);
    }

    #[test]
    fn test_convert_to_decimal_octal() {
        assert_eq!(convert_to_decimal(8, "12".to_string()), 10);
        assert_eq!(convert_to_decimal(8, "0".to_string()), 0);
        assert_eq!(convert_to_decimal(8, "77".to_string()), 63);
        assert_eq!(convert_to_decimal(8, "100".to_string()), 64);
    }

    #[test]
    fn test_convert_to_decimal_decimal() {
        assert_eq!(convert_to_decimal(10, "10".to_string()), 10);
        assert_eq!(convert_to_decimal(10, "0".to_string()), 0);
        assert_eq!(convert_to_decimal(10, "12345".to_string()), 12345);
    }

    #[test]
    fn test_convert_to_decimal_hex() {
        assert_eq!(convert_to_decimal(16, "A".to_string()), 10);
        assert_eq!(convert_to_decimal(16, "0".to_string()), 0);
        assert_eq!(convert_to_decimal(16, "FF".to_string()), 255);
        assert_eq!(convert_to_decimal(16, "FFF".to_string()), 4095);
        assert_eq!(convert_to_decimal(16, "FFFF".to_string()), 65535);
    }

    #[test]
    fn test_convert_to_decimal_invalid_chars() {
        assert_eq!(convert_to_decimal(2, "102".to_string()), 2); // '2' is invalid in binary
        assert_eq!(convert_to_decimal(8, "89".to_string()), 0); // '8' and '9' are invalid in octal
        assert_eq!(convert_to_decimal(16, "FG".to_string()), 15); // 'G' is invalid in hex
    }

    #[test]
    fn test_round_trip_conversion() {
        let original = 123456;
        let base = 16;
        let converted = convert_to_base(base, original);
        let back = convert_to_decimal(base as u32, converted);
        assert_eq!(back, original as i64);
    }
}