// Тотализатор
// Есть футбольный тотализатор. Каждый участник делает ставку на результат матча (счет).
// После окончания матча происходит проверка: если человек угадал счет, система начисляет ему 2 балла; если человек угадывает только исход матча — 1 балл, и если не угадывает ничего — 0 баллов.

// Напишите функцию, которая возвращает словарь (или список) баллов выигранных участниками (юзер1:баллы_юзера1, юзер2:баллы_юзера2, ...). Функция принимает 2 аргумента:
// ставки нескольких участников в виде словаря/списка (юзер1:счет1, юзер2:счет2, ...)
// результат матча строкой (“3:4”)

fn main() {
    let players = get_result(
        vec![
            ("a", "3:4".to_string()),
            ("b", "4:3".to_string()),
            ("c", "2:4".to_string()),
            ("d", "3:3".to_string()),
            ("e", "2:2".to_string()),
        ],
        "3:3".to_string(),
    );
    println!("{:?}", players);
}

#[derive(Debug, PartialEq)]
enum TeamWon {
    First,
    Second,
    Draw,
}

fn get_result(bets: Vec<(&str, String)>, result: String) -> Vec<(&str, u32)> {
    let mut results = vec![];

    let real_tw = who_won(&result);

    for bet in bets {
        let bet_score = bet.1.clone();
        if result == bet_score {
            results.push((bet.0, 2));
        } else {
            let points = match who_won(&bet.1) == real_tw {
                true => 1,
                false => 0,
            };
            results.push((bet.0, points));
        }
    }

    results
}

fn who_won(result: &str) -> TeamWon {
    let tw: Vec<&str> = result.rsplit(":").collect();
    if tw[0] > tw[1] {
        TeamWon::First
    } else if tw[0] < tw[1] {
        TeamWon::Second
    } else {
        TeamWon::Draw
    }
}
