pub const SIZE : usize = 3;
const GAMECOUNT : usize = SIZE * SIZE;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Turns {
    Zero,
    X,
    None,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum WinOrNot {
    PlayerWin,
    CompWin,
    Equal,
    None,
}

pub struct Game {
    fields : Vec<Turns>,
    comp_turn : Turns,
    player_turn : Turns,
}

impl Game {
    pub fn new(comp_turn : Turns) -> Self {
        let player_turn = if Turns::Zero == comp_turn {
            Turns::X
        } else {
            Turns::Zero
        };
        Game { 
            fields : vec![Turns::None; GAMECOUNT], 
            comp_turn,
            player_turn
        }
    }

    pub fn player_turn(&mut self, x : usize, y : usize) -> WinOrNot {
        if self.is_equal() {
            return WinOrNot::Equal
        }
        if self.fields[xy_to_idx(x, y)] != Turns::None {
            panic!("field is owned!!!");
        }
        self.fields[xy_to_idx(x, y)] = self.player_turn;
        self.win_or_not()
    }

    pub fn print_fields(&self) {
        let mut count = 1;
        println!("+---+---+---+");
        for elem in self.fields.iter() {
            if count % 4 == 0 {
                println!("|");
                count = 1;
            }
            let symb = match elem {
                Turns::Zero => '0',
                Turns::X => 'X',
                Turns::None => ' ',
            };
            print!("| {} ", symb);
            count += 1;
        }
        println!("|");
        println!("+---+---+---+");
    }

    pub fn computer_turn(&mut self) -> WinOrNot {
        let lines = vec![vec![]; 8];
        let lines = self.fill_lines(lines);
        
        // try to win or defense
        for line in lines.iter() {
            if line.iter().filter(|&item| (*item).0 == self.comp_turn).count() == 2 {
                if let Some((_turn, pos)) = line.iter().find(|&item| (*item).0 == Turns::None) {
                    self.fields[*pos] = self.comp_turn;
                    return WinOrNot::CompWin
                }
            } else if line.iter().filter(|&item| (*item).0 == self.player_turn).count() == 2 {
                if let Some((_turn, pos)) = line.iter().find(|&item| (*item).0 == Turns::None) {
                    self.fields[*pos] = self.comp_turn;
                    return WinOrNot::None
                }
            }
        }
        let diagonals = vec![0_usize, 4, 8, 2, 6];
        // computer makes turn
        if self.fields[xy_to_idx(1, 1)] == Turns::None {
            self.fields[xy_to_idx(1, 1)] = self.comp_turn;
            return WinOrNot::None
        }
        for line in lines.iter() {
            if line.iter().filter(|&item| (*item).0 == self.comp_turn).count() >= 1 {
                if let Some((_turn, pos)) = line.iter().find(|&item| diagonals.contains(&(*item).1) && (*item).0 == Turns::None) {
                    self.fields[*pos] = self.comp_turn;
                    return WinOrNot::None
                }
            }
        }
        for line in lines.iter() {
            if let Some((_turn, pos)) = line.iter().find(|&item| (*item).0 == Turns::None) {
                self.fields[*pos] = self.comp_turn;
                return WinOrNot::None
            }
        }
        WinOrNot::None
    }

    fn win_or_not(&self) -> WinOrNot {
        let lines = vec![vec![]; 8];
        let lines = self.fill_lines(lines);

        for line in lines {
            if line.iter().filter(|&item| (*item).0 == self.player_turn).count() == 3 {
                return WinOrNot::PlayerWin
            }
        }
        WinOrNot::None
    }

    fn fill_lines(&self, mut lines: Vec<Vec<(Turns, usize)>>) -> Vec<Vec<(Turns, usize)>> {
        let mut count = 0;
        // read vertical and gorizontal lines
        for i in 0..SIZE {
            for j in 0..SIZE {
                lines[count].push((self.fields[xy_to_idx(i, j)], xy_to_idx(i, j)));
                lines[count + 1].push((self.fields[xy_to_idx(j, i)], xy_to_idx(j, i)));
            }
            count += 2;
        }
        lines[count].push((self.fields[xy_to_idx(0, 0)], xy_to_idx(0, 0)));
        lines[count].push((self.fields[xy_to_idx(1, 1)], xy_to_idx(1, 1)));
        lines[count].push((self.fields[xy_to_idx(2, 2)], xy_to_idx(2, 2)));

        count += 1;

        lines[count].push((self.fields[xy_to_idx(0, 2)], xy_to_idx(0, 2)));
        lines[count].push((self.fields[xy_to_idx(1, 1)], xy_to_idx(1, 1)));
        lines[count].push((self.fields[xy_to_idx(2, 0)], xy_to_idx(2, 0)));
        lines
    }

    pub fn is_equal(&self) -> bool {
        if self.fields.iter().filter(|&p| *p != Turns::None).count() == GAMECOUNT {
            return true
        }
        false
    }
}

fn xy_to_idx(x : usize, y : usize) -> usize {
    x + y * SIZE
}