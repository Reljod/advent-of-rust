pub mod day_2 {
    use std::fmt;
    use std::fs;

    enum RockPaperScissors {
        Rock,
        Paper,
        Scissors,
    }

    enum Result {
        Win,
        Lose,
        Draw,
    }

    impl Clone for RockPaperScissors {
        fn clone(&self) -> Self {
            match self {
                RockPaperScissors::Rock => RockPaperScissors::Rock,
                RockPaperScissors::Paper => RockPaperScissors::Paper,
                RockPaperScissors::Scissors => RockPaperScissors::Scissors,
            }
        }
    }

    // // add copy trait
    impl Copy for RockPaperScissors {}

    impl fmt::Display for RockPaperScissors {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RockPaperScissors::Rock => write!(f, "Rock"),
                RockPaperScissors::Paper => write!(f, "Paper"),
                RockPaperScissors::Scissors => write!(f, "Scissors"),
            }
        }
    }

    impl fmt::Display for Result {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Result::Win => write!(f, "Win"),
                Result::Lose => write!(f, "Lose"),
                Result::Draw => write!(f, "Draw"),
            }
        }
    }

    impl Clone for Result {
        fn clone(&self) -> Self {
            match self {
                Result::Win => Result::Win,
                Result::Lose => Result::Lose,
                Result::Draw => Result::Draw,
            }
        }
    }

    impl Copy for Result {}

    pub fn run() {
        let strategy_raw =
            fs::read_to_string("./input/day2.txt").expect("Something went wrong reading the file");

        let strategy: Vec<&str> = strategy_raw.lines().collect();

        let points_first_part = self::first_part::get_match_points(&strategy);
        println!(
            "DAY2 (first question): First Part Points: {}",
            points_first_part
        );

        let points_second_part = self::second_part::get_match_points(&strategy);
        println!(
            "DAY2 (second question): Second Part Points: {}",
            points_second_part
        );
    }

    fn get_battle_result(opponent: &RockPaperScissors, _self: &RockPaperScissors) -> String {
        match (opponent, _self) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => Result::Draw.to_string(),
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => Result::Win.to_string(),
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => Result::Lose.to_string(),
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => Result::Lose.to_string(),
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => Result::Draw.to_string(),
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => Result::Win.to_string(),
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => Result::Win.to_string(),
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => Result::Lose.to_string(),
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => Result::Draw.to_string(),
        }
    }

    fn get_battle_points(result: &str, _self: &RockPaperScissors) -> i32 {
        let shape_points = match _self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        };

        match result {
            "Win" => 6 + shape_points,
            "Lose" => shape_points,
            "Draw" => 3 + shape_points,
            _ => 0,
        }
    }

    pub mod first_part {
        use super::RockPaperScissors;
        use std::collections::HashMap;

        pub fn get_match_points(battle: &Vec<&str>) -> i32 {
            let opponent_map = HashMap::from([
                ("A", RockPaperScissors::Rock),
                ("B", RockPaperScissors::Paper),
                ("C", RockPaperScissors::Scissors),
            ]);

            let self_map = HashMap::from([
                ("X", RockPaperScissors::Rock),
                ("Y", RockPaperScissors::Paper),
                ("Z", RockPaperScissors::Scissors),
            ]);

            let mut points: i32 = 0;
            for round in battle {
                let (opponent_move, self_move) = round.split_once(" ").unwrap();

                let _self = self_map[self_move];
                let opponent = &opponent_map[opponent_move];
                let result = super::get_battle_result(opponent, &_self);
                points += super::get_battle_points(&result, &_self);
            }
            points
        }
    }

    pub mod second_part {
        use super::Result;
        use super::RockPaperScissors;
        use std::collections::HashMap;

        pub fn get_match_points(battle: &Vec<&str>) -> i32 {
            let opponent_map = HashMap::from([
                ("A", RockPaperScissors::Rock),
                ("B", RockPaperScissors::Paper),
                ("C", RockPaperScissors::Scissors),
            ]);

            let expected_outcome_map =
                HashMap::from([("X", Result::Lose), ("Y", Result::Draw), ("Z", Result::Win)]);

            let mut points: i32 = 0;
            for round in battle {
                let (opponent_move, expected_outcome) = round.split_once(" ").unwrap();
                let expected = expected_outcome_map[expected_outcome];

                let opponent = opponent_map[opponent_move];
                let _self = determine_move(&opponent_map[opponent_move], &expected);

                let result = super::get_battle_result(&opponent, &_self);
                points += super::get_battle_points(&result, &_self);
            }
            points
        }

        fn determine_move(
            opponent_move: &RockPaperScissors,
            expected_outcome: &Result,
        ) -> RockPaperScissors {
            match (opponent_move, expected_outcome) {
                (opponent_move, &Result::Win) => match opponent_move {
                    RockPaperScissors::Rock => RockPaperScissors::Paper,
                    RockPaperScissors::Paper => RockPaperScissors::Scissors,
                    RockPaperScissors::Scissors => RockPaperScissors::Rock,
                },
                (opponent_move, &Result::Lose) => match opponent_move {
                    RockPaperScissors::Rock => RockPaperScissors::Scissors,
                    RockPaperScissors::Paper => RockPaperScissors::Rock,
                    RockPaperScissors::Scissors => RockPaperScissors::Paper,
                },
                (opponent_move, &Result::Draw) => opponent_move.clone(),
            }
        }
    }
}
