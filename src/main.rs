use std::collections::HashSet;
#[derive(Debug)]
#[derive(Clone)]
struct MiniDokuCell {
    pub index: usize,
    pub current_value: u32,
    pub is_locked: bool,
    pub potential_values: HashSet<u32>,
}

impl MiniDokuCell {
    fn new(index: usize) -> Self {
        let mut new_cell = MiniDokuCell {
            index,
            current_value: 0,
            is_locked: false,
            potential_values: HashSet::new(),
        };
        new_cell
    }

    // fn solve(&mut self, board: &mut MiniDoku) -> Result<(), String> {
    //     let (x, y) = board.get_coords(self.index);
    //     // Base cases:
    //     // If I'm the last cell:
    //     if self.index == 15 {
    //         if self.is_locked { // If I'm locked, we're done!
    //             return Ok(());
    //         }
    //         self.potential_values = board.gather_possible_values(x, y);
    //         if self.potential_values.is_empty() {
    //             return Err(format!("Cell at {} returning err because no possible values remain", self.index));
    //         } else {
    //             // There's at least one value in potential_values. There should be exactly one, or else
    //             assert_eq!(self.potential_values.len(), 1);
    //             self.current_value = self.potential_values.clone().into_iter().next().unwrap();
    //             return Ok(());
    //         }
    //     }
    //     // If I'm not the last cell && I'm already locked, then whether I can solve is determined
    //     // by whether my next neighbor can solve:
    //     if self.is_locked {
    //         let (x, y) = board.get_coords(self.index + 1);
    //         return board.cell_at(x, y).solve(board);
    //     }
    //
    //     self.potential_values = board.gather_possible_values(x, y);
    //
    //     Ok(())
    // }
}

#[derive(Clone)]
struct MiniDoku {
    cells: Vec<MiniDokuCell>,
    // valid_cells: Vec<(u32, u32)>,
}

impl MiniDoku {
    // This is a mini-sudoku board of size 4x4; it looks kind of like this;
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
    fn new() -> Self {
        let mut my_doku = MiniDoku {
            cells: Vec::new(),
        };
        for i in 0..16 {
            my_doku.cells.push(MiniDokuCell::new(i));
        }
        my_doku
    }

    fn cell_at(&self, x: usize, y: usize) -> &MiniDokuCell {
        let cell_index = (y * 4) + x;
        self.cells.get(cell_index).unwrap()
    }

    fn get_coords(&self, index: usize) -> (usize, usize) {
        let x = index % 4;
        let y = (index - x) / 4;
        (x, y)
    }

    fn load_board(&mut self, board: [u32; 16]) {
        for i in 0..16 {
            let mut cell = self.cells.get_mut(i as usize).unwrap();
            cell.current_value = board[i];
            cell.is_locked = true;
        }
    }

    fn gather_possible_values(&self, x: usize, y: usize) -> HashSet<u32> {
        let mut potential_values: HashSet<u32> = HashSet::new();
        potential_values.insert(1);
        potential_values.insert(2);
        potential_values.insert(3);
        potential_values.insert(4);
        for i_x in 0..4 {
            if i_x == x {
                continue;
            }
            let cell = self.cell_at(i_x, y);
            potential_values.remove(&cell.current_value);
        }
        for i_y in 0..4 {
            if i_y == y {
                continue;
            }
            let cell = self.cell_at(x, i_y);
            potential_values.remove(&cell.current_value);
        }
        // How to calculate quadrant?
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
        potential_values
    }

    fn solve(&mut self, index: usize) -> Result<(), String> {
        let (x, y) = self.get_coords(index);
        let cell = self.cells.get(index).unwrap();
        let mut potential_values = HashSet::new();
        // Base cases:
        // If we're on the last cell:
        if index == 15 {
            if cell.is_locked { // If the last cell is locked, we're done!
                return Ok(());
            }
            potential_values = self.clone().gather_possible_values(x, y);
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

        potential_values = self.gather_possible_values(x, y);
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

                    }
                }
            }
            return Err(format!("Cell at {}: Tried all values, none led to solve ", index));
        }

        Ok(())
    }
}

impl std::fmt::Display for MiniDoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: All the write! macro invocations here should return their result
        //       if it is an error. Does `?` do that? or something else?
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
        write!(f, "+---+---++---+---+")
    }
}

fn main() {
    let mut my_doku = MiniDoku::new();
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
