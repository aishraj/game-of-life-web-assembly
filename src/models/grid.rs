use models::size::Size;

pub struct Grid {
    state: Vec<Vec<bool>>, //State is a 2d vector of boolean representing the grid
    pub size: Size,
}

impl Grid {
    pub fn new(s: Size) -> Grid {
        Grid {
            state: create_nested_blanks(s.clone()),
            size: s,
        }
    }

    pub fn is_living(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.size.width || y < 0 || y >= self.size.height {
            return false;
        }
        let x = x as usize;
        let y = y as usize;
        self.state[y][x]
    }

    pub fn get_neighbour_count(&self, x: i32, y: i32) -> i32 {
        let xy_pairs = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let c = xy_pairs.into_iter().fold(0, |cnt, &(x_off, y_off)| {
            let res = if self.is_living(x + x_off, y + y_off) {
                cnt + 1
            } else {
                cnt
            };
            res
        });
        c
    }

    pub fn set_living(&mut self, x: i32, y: i32) {
        let x = x as usize;
        let y = y as usize;
        self.state[y][x] = true;
    }

    pub fn set_dead(&mut self, x: i32, y: i32) {
        let x = x as usize;
        let y = y as usize;
        self.state[y][x] = false;
    }
}

fn create_nested_blanks(s: Size) -> Vec<Vec<bool>> {
    let max_row = s.height;
    let max_col = s.width;
    let mut matrix = Vec::new();
    for _i in 0..max_row {
        let mut v2 = Vec::new();
        for _j in 0..max_col {
            v2.push(false);
        }
        matrix.push(v2);
    }
    matrix
}
