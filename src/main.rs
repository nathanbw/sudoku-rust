use std::collections::HashSet;
struct DokuCell {
    pub current_value: u32,
    pub is_locked: bool,
}

impl DokuCell {
    fn new() -> Self {
        let new_cell = DokuCell {
            current_value: 0,
            is_locked: false,
        };
        new_cell
    }
}

struct Sudoku {
    cells: Vec<DokuCell>,
    dimension: usize,
}

impl Sudoku {
    // This is a mini-sudoku board of size 4x4; it looks kind of like this.
    // In the diagram below, you can see which x, y pair corresponds to which
    // index in the cells vector:
    //
    //   X:   0   1    2   3
    //      +---+---++---+---+
    // Y: 0 | 0 | 1 || 2 | 3 |
    //      |---+---++---+---|
    //    1 | 4 | 5 || 6 | 7 |
    //      |===+===++===+===|
    //    2 | 8 | 9 || 10| 11|
    //      |---+---++---+---|
    //    3 | 12| 13|| 14| 15|
    //      +---+---++---+---+
    //
    // Based on the above, we should be able to calculate the cell index with
    // the formula:
    // cell_index = y * 4 + x
    fn new(dimension: usize) -> Self {
        let mut my_doku = Sudoku {
            cells: Vec::new(),
            dimension,
        };
        for _i in 0..(dimension * dimension) {
            my_doku.cells.push(DokuCell::new());
        }
        my_doku
    }

    fn cell_at(&self, x: usize, y: usize) -> &DokuCell {
        let cell_index = (y * self.dimension) + x;
        self.cells.get(cell_index).unwrap()
    }

    fn get_coords(&self, index: usize) -> (usize, usize) {
        let x = index % self.dimension;
        let y = (index - x) / self.dimension;
        (x, y)
    }

    fn load_board(&mut self, board: Vec<u32>) {
        assert_eq!(board.len(), (self.dimension * self.dimension),
                   "Tried to load board of wrong dimension!");
        for i in 0..board.len() {
            let cell = self.cells.get_mut(i).unwrap();
            cell.current_value = board[i];
            cell.is_locked = true;
        }
    }

    fn gather_possible_values(&self, x: usize, y: usize) -> HashSet<u32> {
        let mut potential_values: HashSet<u32> = HashSet::new();
        for value in 1..(self.dimension + 1) {
            potential_values.insert(value as u32);
        }
        for x_i in 0..self.dimension {
            if x_i == x {
                continue;
            }
            let cell = self.cell_at(x_i, y);
            potential_values.remove(&cell.current_value);
        }
        for y_i in 0..self.dimension {
            if y_i == y {
                continue;
            }
            let cell = self.cell_at(x, y_i);
            potential_values.remove(&cell.current_value);
        }
        // TODO: Figure out if you can factor this better and maybe even avoid magic numbers?
        if self.dimension == 4 {
            if x < 2 {
                if y < 2 {
                    // Top left quadrant
                    for i_x in 0..2 {
                        for i_y in 0..2 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                } else { // y >= 2
                    // Bottom left quadrant
                    for i_x in 0..2 {
                        for i_y in 2..4 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                }
            } else { // x >= 2
                if y < 2 {
                    // Top right quadrant
                    for i_x in 2..4 {
                        for i_y in 0..2 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                } else { // y >= 2
                    // Bottom left quadrant
                    for i_x in 2..4 {
                        for i_y in 2..4 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                }
            }
        }
        //   X:   0   1   2    3   4   5    6   7   8
        //      +---+---+---++---+---+---++---+---+---+
        // Y: 0 | 0 | 1 | 2 || 3 | 4 | 5 || 6 | 7 | 8 |
        //      |---+---+---+|---+---+---+|---+---+---+
        //    1 | 9 | 10| 11|| 12| 13| 14|| 15| 16| 17|
        //      |---+---+---+|---+---+---+|---+---+---+
        //    2 | 18| 10| 20|| 21| 22| 23|| 8 | 9 | 10|
        //      |===+===+===+|===+===+===+|===+===+===+
        //    3 | 12| 13| 14|| 12| 13| 14|| 12| 13| 14|
        //      |---+---+---+|---+---+---+|---+---+---+
        //    4 | 12| 13| 14|| 12| 13| 14|| 12| 13| 14|
        //      |---+---+---+|---+---+---+|---+---+---+
        //    5 | 12| 13| 14|| 12| 13| 14|| 12| 13| 14|
        //      |===+===+===+|===+===+===+|===+===+===+
        //    6 | 12| 13| 14|| 12| 13| 14|| 12| 13| 14|
        //      +---+---+---++---+---+---++---+---+---+
        //    7 | 12| 13| 14|| 12| 13| 14|| 12| 13| 14|
        //      +---+---+---++---+---+---++---+---+---+
        //    8 | 12| 13| 14|| 12| 13| 14|| 12| 13| 14|
        //      +---+---+---++---+---+---++---+---+---+
        if self.dimension == 9 {
            if x < 3 {
                if y < 3 {
                    // Top left quadrant
                    for i_x in 0..3 {
                        for i_y in 0..3 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                } else if y >= 3 && y < 6 {
                    // Middle left quadrant
                    for i_x in 0..3 {
                        for i_y in 3..6 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }

                } else { // y >= 6
                    // Bottom left quadrant
                    for i_x in 0..3 {
                        for i_y in 6..9 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                }
            } else if x >= 3 && x < 6 {
                if y < 3 {
                    // Top middle quadrant
                    for i_x in 3..6 {
                        for i_y in 0..3 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                } else if y >= 3 && y < 6 {
                    // Middle middle quadrant
                    for i_x in 3..6 {
                        for i_y in 3..6 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }

                } else { // y >= 6
                    // Bottom middle quadrant
                    for i_x in 3..6 {
                        for i_y in 6..9 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                }
            } else { // x >= 6
                if y < 3 {
                    // Top right quadrant
                    for i_x in 6..9 {
                        for i_y in 0..3 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                } else if y >= 3 && y < 6 {
                    // Middle right quadrant
                    for i_x in 6..9 {
                        for i_y in 3..6 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }

                } else { // y >= 6
                    // Bottom right quadrant
                    for i_x in 6..9 {
                        for i_y in 6..9 {
                            let cell = self.cell_at(i_x, i_y);
                            potential_values.remove(&cell.current_value);
                        }
                    }
                }
            }
        }
        potential_values
    }

    fn solve(&mut self, index: usize) -> Result<(), String> {
        let (x, y) = self.get_coords(index);
        let cell = self.cells.get(index).unwrap();
        // let mut potential_values = HashSet::new();
        // Base cases:
        // If we're on the last cell:
        if index == ((self.dimension * self.dimension) - 1) {
            if cell.is_locked { // If the last cell is locked, we're done!
                return Ok(());
            }
            let potential_values = self.gather_possible_values(x, y);
            if potential_values.is_empty() {
                return Err(format!("Cell at {} returning err because no possible values remain", index));
            } else {
                // There's at least one value in potential_values. There should be exactly one, or else
                assert_eq!(potential_values.len(), 1);
                self.cells.get_mut(index).unwrap().current_value =
                    potential_values.clone().into_iter().next().unwrap();
                return Ok(());
            }
        }
        // If we're not on the last cell && it's already locked, then whether I can solve is
        // determined by whether the next neighbor can solve:
        if cell.is_locked {
            return self.solve(index + 1);
        }

        let potential_values = self.gather_possible_values(x, y);
        if potential_values.is_empty() {
            return Err(format!("Cell at {} returning err because no possible values remain", index));
        } else {
            for x in potential_values {
                self.cells.get_mut(index).unwrap().current_value = x;
                match self.solve(index + 1) {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(_) => {
                        self.cells.get_mut(index).unwrap().current_value = 0;
                    }
                }
            }
            return Err(format!("Cell at {}: Tried all values, none led to solve ", index));
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: All the write! macro invocations here should return their result
        //       if it is an error. Does `?` do that? or something else?
        // TODO: Can I make this generic?
        if self.dimension == 4 {
            for y in 0..4 {
                if y == 2 {
                    write!(f, "|===+===++===+===|\n")?;
                } else {
                    write!(f, "+---+---++---+---+\n")?;
                }
                for x in 0..4 {
                    if x == 1 || x == 3 {
                        write!(f, "| {} |", self.cell_at(x, y).current_value)?;
                    } else {
                        write!(f, "| {} ", self.cell_at(x, y).current_value)?;
                    }
                    if x == 3 {
                        write!(f, "\n")?;
                    }
                }
            }
            return write!(f, "+---+---++---+---+")
        }
        if self.dimension == 9 {
            //   X:    0   1   2    3   4   5    6   7   8
            //      ||===+===+===++===+===+===++===+===+===||
            // Y: 0 || 9 | 4 | 3 || 5 | 8 | 7 || 2 | 1 | 6 ||
            //      ++---+---+---++---+---+---++---+---+---++
            //    1 || 2 | 6 | 5 || 9 | 1 | 4 || 7 | 3 | 8 ||
            //      ++---+---+---++---+---+---++---+---+---++
            //    2 || 8 | 1 | 7 || 6 | 2 | 3 || 5 | 4 | 9 ||
            //      ||===+===+===++===+===+===++===+===+===||
            //    3 || 4 | 2 | 6 || 3 | 7 | 8 || 1 | 9 | 5 ||
            //      ++---+---+---++---+---+---++---+---+---++
            //    4 || 5 | 3 | 9 || 1 | 6 | 2 || 4 | 8 | 7 ||
            //      ++---+---+---++---+---+---++---+---+---++
            //    5 || 1 | 7 | 8 || 4 | 5 | 9 || 6 | 2 | 3 ||
            //      ||===+===+===++===+===+===++===+===+===||
            //    6 || 3 | 9 | 2 || 7 | 4 | 6 || 8 | 5 | 1 ||
            //      ++---+---+---++---+---+---++---+---+---++
            //    7 || 7 | 5 | 4 || 8 | 3 | 1 || 9 | 6 | 2 ||
            //      ++---+---+---++---+---+---++---+---+---++
            //    8 || 6 | 8 | 1 || 2 | 9 | 5 || 3 | 7 | 4 ||
            //      ||===+===+===++===+===+===++===+===+===||
            for y in 0..9 {
                if y % 3 == 0 {
                    write!(f, "||===+===+===++===+===+===++===+===+===||\n")?;
                } else {
                    write!(f, "++---+---+---++---+---+---++---+---+---++\n")?;
                    // write!(f, "\n")?;
                }
                for x in 0..9 {
                    if x == 0 {
                        write!(f, "|")?;
                    }
                    if x % 3 == 2 {
                        write!(f, "| {} |", self.cell_at(x, y).current_value)?;
                    } else {
                        write!(f, "| {} ", self.cell_at(x, y).current_value)?;
                    }
                    if x == 8 {
                        write!(f, "|\n")?;
                    }
                }
            }
            return write!(f, "||===+===+===++===+===+===++===+===+===||\n");

        }
        write!(f, "Not implemented for dimension {}", self.dimension)
    }
}

fn main() {
    // let puzzle: Vec<u32> = vec![
    //     1, 2, 3, 4
    //     3, 4, 1, 2
    //
    // ];
    let mut my_doku = Sudoku::new(9);
    // println!("{my_doku}");
    // println!("{}", my_doku.cell_at(3, 2).current_value);
    match my_doku.solve(0) {
        Ok(_) => {
            println!("{}", my_doku);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
