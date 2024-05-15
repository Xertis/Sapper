use rand::Rng;
use std::io;

fn print_pole(field: [[i8; 16]; 16]) {
    let mut line: i8 = 0;
    for row in field.iter() {
        line += 1;
        print!("{}.\t", line-1);
        for &element in row.iter() {
            print!("{} ", element);
        }
        println!();
    }
}

fn create_zone_16() -> [[i8; 16]; 16] {
    let zone: [[i8; 16]; 16] = [[0; 16]; 16];
    return zone;
}

fn gen_mines(zone: & mut [[i8; 16]; 16]) {
    let mut rng = rand::thread_rng();
    for _i in 0..=26 {
        loop {
            let mines_x: i8 = rng.gen_range(0..16);
            let mines_y: i8 = rng.gen_range(0..16);
            if zone[mines_y as usize][mines_x as usize] == 0 {
                zone[mines_y as usize][mines_x as usize] = 1;
                break;
            }
        }
    }
}

fn is_mines(zone: & mut [[i8; 16]; 16], x: u8, y: u8) -> bool {
    if zone[y as usize][x as usize] == 1 {
        return true;
    } else {
        return false;
    }
}

fn get_count_mines(zone: & mut [[i8; 16]; 16], x: isize, y: isize) -> i8 {
    let mut count_mines: i8 = 0;
    for yf in y-1..=y+1 {
        for xf in x-1..=x+1 {
            if (yf <= 15 && xf <= 15) && (yf >= 0 && xf >= 0) {
                if zone[yf as usize][xf as usize] == 1 {
                    count_mines += 1
                }
            }
        }
    }

    if count_mines == 0 {
        count_mines = 9
    }

    return count_mines;
}

/**
 * Внутренний слой:
 * 0. Пустая зона
 * 1. Мина
 * 2. Открытая зона
 * 
 * Верхний слой:
 * 0. Закрытая клетка
 * 1..8 Кол-во мин вокруг
 * 9. Открытая клетка
 * **/

fn main() {

    let mut zone = create_zone_16();
    let mut field = create_zone_16();

    gen_mines(&mut zone);

    loop {
        print_pole(field);

        println!("Y:");
        let mut keyword = String::new();
        io::stdin().read_line(& mut keyword).unwrap();
        let quest_y: u8 = keyword.trim().parse().unwrap();

        println!("X:");
        keyword = String::new();
        io::stdin().read_line(& mut keyword).unwrap();
        let quest_x: u8 = keyword.trim().parse().unwrap();
        

        if is_mines(&mut zone, quest_x, quest_y) {
            print_pole(zone);
            println!("Соре, ты проиграл");
            break;
        }

        field[quest_y as usize][quest_x as usize] = get_count_mines(&mut zone, quest_x as isize, quest_y as isize)

        
    }
}
