#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn new_game_has_score_of_zero() {
        let test_game = Game::new_game();
        assert_eq!(test_game.get_score(), 0)
    }

    #[test]
    fn first_roll_scores_simply() {
        let mut test_game = Game::new_game();
        test_game.roll(5);
        assert_eq!(test_game.get_score(), 5)
    }

    #[test]
    fn two_rolls_score_simply() {
        let mut test_game = Game::new_game();
        test_game.roll(3);
        test_game.roll(4);
        assert_eq!(test_game.get_score(), 7)
    }

    #[test]
    fn spare_doubles_third_roll() {
        let mut test_game = Game::new_game();
        test_game.roll(3);
        test_game.roll(7);
        test_game.roll(2);
        test_game.roll(3);
        assert_eq!(test_game.get_score(), 17)
    }

    #[test]
    fn strike_doubles_third_and_fourth_roll() {
        let mut test_game = Game::new_game();
        test_game.roll(10);
        test_game.roll(2);
        test_game.roll(3);
        assert_eq!(test_game.get_score(), 20)
    }
}

struct Frame {
    rolls: Vec<i32>
}

struct Game {
    frames: Vec<Frame>,
    current_frame: usize
}

#[derive(PartialEq, Clone, Copy)]
enum FrameResult {
    Normal,
    Spare,
    Strike
}

impl Frame {
    fn get_frame_result(&self) -> FrameResult {
        if self.rolls.len() == 1 &&  self.rolls[0] == 10 {
            return FrameResult::Strike;
        } else if self.rolls.len() == 2 && self.rolls[0] + self.rolls[1] == 10 {
            return FrameResult::Spare;
        }
        return FrameResult::Normal
    }
}

fn score_multiplier(prev_frame_result: FrameResult, current_roll: i32) -> i32 {
    if prev_frame_result == FrameResult::Strike {
        return 2;
    } else if prev_frame_result == FrameResult::Spare && current_roll == 1 {
        return 2;
    }
    return 1;
}

impl Game {
    fn new_game() -> Game {
        let mut result = Game {
            frames: Vec::with_capacity(10),
            current_frame: 0,
        };
        result.frames.push(Frame {
            rolls: Vec::with_capacity(2)
        });
        return result;
    }

    fn roll(&mut self, pins: i32) {
        self.frames[self.current_frame].rolls.push(pins);
        if self.frames[self.current_frame].rolls.len() == 2 || pins == 10 {
            self.current_frame = self.current_frame + 1;
            self.frames.push(Frame {
                rolls: Vec::with_capacity(2)
            });
        }
    }

    fn get_score(&self) -> i32 {
        let mut total_score = 0;
        let mut prev_frame_result = FrameResult::Normal;
        for frame in &self.frames {
            let mut current_roll = 1;
            for roll in &frame.rolls {
                total_score += roll * score_multiplier(prev_frame_result, current_roll);
                current_roll = current_roll + 1;
            }
            prev_frame_result = frame.get_frame_result();
        }
        return total_score;
    }
}
