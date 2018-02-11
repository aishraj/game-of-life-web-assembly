use models::size::Size;

pub struct Grid {
  state: Vec<Vec<bool>>, //State is a 2d vector of boolean representing the grid
  pub size: Size
}

impl Grid {
    pub fn new(s: Size) -> Grid {
        Grid {
            state: create_nested_blanks(s.height, s.width),
            size: s
        }
    }

    pub fn is_living(&self, x: i32, y: i32) -> bool {
      let x = x as usize;
      let y = y as usize;
      self.state[x][y]
    }

    pub fn get_neighbour_count(&self) -> i32 {
        0 //TODO: Change this
    }

    pub fn set_living(&self, x: i32, y: i32) {
        //TODO: Do something
    }

    pub fn set_dead(&self, x: i32, y: i32) {
        //TODO: Do something
    }
}

fn create_nested_blanks(height: i32, width: i32) -> Vec<Vec<bool>> {
    let mut matrix = Vec::new();
    for _i in 0..height {
        let mut v2 = Vec::new();
        for _j in 0..width {
            v2.push(false);
        }
        matrix.push(v2);
    }
    matrix
}