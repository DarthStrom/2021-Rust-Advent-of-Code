use crate::input;

pub fn run() {
    let contents = input::get_lines("day04");

    let numbers = get_numbers(&contents);
    let boards = get_boards(&contents);
    let game = Game::new(numbers, boards);

    let (_winner, score) = game.predict_winner();
    println!("part1: {:?}", score);

    let (_loser, score) = game.predict_loser();
    println!("part2: {:?}", score);
}

fn get_numbers(lines: &[String]) -> Vec<u32> {
    lines[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn get_boards(lines: &[String]) -> Vec<Vec<Vec<u32>>> {
    lines[2..]
        .iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| {
            chunk
                .iter()
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Debug)]
struct Game {
    drawn: Vec<u32>,
    numbers: Vec<u32>,
    boards: Vec<Vec<Vec<u32>>>,
    latest_winner: Option<usize>,
    winners: Vec<usize>,
    score: u32,
}

impl Game {
    fn new(numbers: Vec<u32>, boards: Vec<Vec<Vec<u32>>>) -> Self {
        Self {
            drawn: vec![],
            numbers,
            boards,
            latest_winner: None,
            winners: vec![],
            score: 0,
        }
    }

    fn draw(self) -> Self {
        let called = *self.numbers.first().unwrap();
        let numbers = self.numbers[1..].to_vec();
        let mut drawn = self.drawn.clone();
        drawn.push(called);

        let mut latest_winner = self.latest_winner;
        let winners = get_winners(&drawn, &self.boards);
        let score;
        if self.winners != winners {
            let new_winner = *winners
                .iter()
                .filter(|winner| !self.winners.contains(winner))
                .collect::<Vec<&usize>>()
                .pop()
                .unwrap();
            latest_winner = Some(new_winner);
            score = calculate_score(called, &drawn, &self.boards[new_winner]);
        } else {
            score = self.score
        }

        Self {
            drawn,
            numbers,
            latest_winner,
            winners,
            score,
            ..self
        }
    }

    fn predict_winner(&self) -> (Option<usize>, u32) {
        let mut hypothetical = self.clone().draw();
        while hypothetical.winners.is_empty() || hypothetical.numbers.is_empty() {
            hypothetical = hypothetical.draw();
        }
        (hypothetical.latest_winner, hypothetical.score)
    }

    fn predict_loser(&self) -> (Option<usize>, u32) {
        let mut hypothetical = self.clone().draw();
        while hypothetical.winners.len() < hypothetical.boards.len()
            || hypothetical.numbers.is_empty()
        {
            hypothetical = hypothetical.draw();
        }
        (hypothetical.latest_winner, hypothetical.score)
    }
}

fn get_winners(drawn: &[u32], boards: &[Vec<Vec<u32>>]) -> Vec<usize> {
    let mut result = vec![];

    for (i, board) in boards.iter().enumerate() {
        if winning_row(drawn, board) || winning_column(drawn, board) {
            result.push(i);
        }
    }

    result
}

fn winning_row(drawn: &[u32], board: &[Vec<u32>]) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|x| drawn.contains(x)))
}

fn winning_column(drawn: &[u32], board: &[Vec<u32>]) -> bool {
    for i in 0..board[0].len() {
        if board.iter().all(|row| drawn.contains(&row[i])) {
            return true;
        }
    }
    false
}

fn calculate_score(called: u32, drawn: &[u32], board: &[Vec<u32>]) -> u32 {
    board.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|x| !drawn.contains(x)).sum::<u32>()
    }) * called
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 19] = [
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        "8  2 23  4 24",
        "21  9 14 16  7",
        "6 10  3 18  5",
        "1 12 20 15 19",
        "",
        "3 15  0  2 22",
        "9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        "2  0 12  3  7",
    ];

    #[test]
    fn example1_gets_numbers() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(
            get_numbers(&lines),
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
    }

    #[test]
    fn example1_gets_boards() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(
            get_boards(&lines),
            vec![
                vec![
                    vec![22, 13, 17, 11, 0],
                    vec![8, 2, 23, 4, 24],
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ],
                vec![
                    vec![3, 15, 0, 2, 22],
                    vec![9, 18, 13, 17, 5],
                    vec![19, 8, 7, 25, 23],
                    vec![20, 11, 10, 24, 4],
                    vec![14, 21, 16, 12, 6],
                ],
                vec![
                    vec![14, 21, 17, 24, 4],
                    vec![10, 16, 15, 9, 19],
                    vec![18, 8, 23, 26, 20],
                    vec![22, 11, 13, 6, 5],
                    vec![2, 0, 12, 3, 7]
                ]
            ]
        )
    }

    #[test]
    fn example1() {
        let lines = INPUT.map(|s| s.to_string());
        let numbers = get_numbers(&lines);
        let boards = get_boards(&lines);

        let mut game = Game::new(numbers, boards);

        // first 5

        game = game.draw(); // 7
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 4
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 9
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 5
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 11
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        // next 6

        game = game.draw(); // 17
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 23
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 2
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 0
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 14
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        game = game.draw(); // 21
        assert!(game.winners.is_empty());
        assert_eq!(game.score, 0);

        // final draw

        game = game.draw(); // 24
        assert_eq!(game.winners, vec![2]);
        assert_eq!(game.score, 4512);
    }

    #[test]
    fn predicting_the_winner() {
        let lines = INPUT.map(|s| s.to_string());
        let numbers = get_numbers(&lines);
        let boards = get_boards(&lines);

        let game = Game::new(numbers, boards);

        assert_eq!(game.predict_winner(), (Some(2), 4512));
    }

    #[test]
    fn predicting_the_loser() {
        let lines = INPUT.map(|s| s.to_string());
        let numbers = get_numbers(&lines);
        let boards = get_boards(&lines);

        let game = Game::new(numbers, boards);

        assert_eq!(game.predict_loser(), (Some(1), 1924));
    }
}
