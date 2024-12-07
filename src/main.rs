struct Position(usize);

impl From<usize> for Position {
    fn from(index: usize) -> Self {
        assert!(index < Self::all_positions_count());
        Self(index)
    }
}

impl Position {
    pub const fn all_positions_count() -> usize {
        // The indexes in the array are `last_swiped + 9 * index`
        //
        // `last_swiped` is the position (0..9) of the last swiped point and
        // `index` is the sum of (1<<point) for every swiped point.
        //
        // For example, if you swap the point 1, 2 and 5, the index will be
        // 5 (last swapped) + 9*(1<<1) + 9*(1<<2) + 9*(1<<5)
        //
        // NB: There is a +1 because of the empty board
        9 * (1 + (1 << 9))        
    }

    pub fn base(&self) -> usize {
        self.0 / 9
    }

    pub fn last_swiped(&self) -> usize {
        self.0 % 9
    }

    fn index_of_point(point: usize) -> usize {
        9 /* last_swiped */ * (1 << point) /* 1,2,4,8,… */
    }

    pub fn from_point(point: usize) -> Self {
        assert!(point < 9);
        
        Self(Self::index_of_point(point) + point)
    }

    pub fn is_swiped(&self, point: usize) -> bool {
        assert!(point < 9);
        
        // if the bit at the position of `point` is set, then it was already swiped
        (self.base() & (1<<point)) != 0
    }

    pub fn swipe_to(&self, next_swiped_point: usize) -> Option<Self> {
        assert!(next_swiped_point < 9);

        // NB: The keypad looks like:
        // 0 1 2
        // 3 4 5
        // 6 7 8
        
        // To be allowed to swipe …
        //
        // _ the new point must not have have already been swiped
        if self.is_swiped(next_swiped_point) {
            return None;
        }

        // _ the point must not go throught an unswiped point
        match (self.last_swiped(), next_swiped_point) {
            // corners
            (0, 2) if !self.is_swiped(1) => return None,
            (0, 8) if !self.is_swiped(4) => return None,
            (0, 6) if !self.is_swiped(3) => return None,
            (2, 0) if !self.is_swiped(1) => return None,
            (2, 6) if !self.is_swiped(4) => return None,
            (2, 8) if !self.is_swiped(5) => return None,
            (6, 0) if !self.is_swiped(3) => return None,
            (6, 2) if !self.is_swiped(4) => return None,
            (6, 8) if !self.is_swiped(7) => return None,
            (8, 6) if !self.is_swiped(7) => return None,
            (8, 0) if !self.is_swiped(4) => return None,
            (8, 2) if !self.is_swiped(5) => return None,
            // border
            (1, 7) if !self.is_swiped(4) => return None,
            (7, 1) if !self.is_swiped(4) => return None,
            (3, 5) if !self.is_swiped(4) => return None,
            (5, 3) if !self.is_swiped(4) => return None,
            // all combinations are valid from the center of the keypad
            // all other combination are valid
            _ => (),
        }

        Some(Self((self.base() * 9) + Self::index_of_point(next_swiped_point) + next_swiped_point /*last_swiped*/))
    }

    pub fn swiped_points(&self) -> u32 {
        self.base().count_ones()
    }
}

struct States([u32 /* possibilities to hit this state */; Position::all_positions_count()]);

impl std::ops::Index<Position> for States {
    type Output = u32;
    fn index(&self, position: Position) -> &Self::Output {
        &self.0[position.0]        
    }
}

impl std::ops::IndexMut<Position> for States {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.0[position.0]        
    }
}

impl Default for States {
    fn default() -> Self {
        Self (
            [0; Position::all_positions_count()],
        )
    }
}

impl States {
    fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.0.iter()
    }
}

#[derive(Default)]
struct Step {
    possibilities: States,
}

impl Step {
    pub fn init() -> Self {
        let mut current_step = Self::default();

        for i in 0..9 {
            current_step.possibilities[Position::from_point(i)] = 1;
        }

        current_step
    }

    pub fn current_possibilities(&self) -> u32 {
        self.possibilities.iter().sum()
    }

    pub fn validate(&self, step: u32) {
        for (index, &count) in self.possibilities.iter().enumerate() {
            let position: Position = index.into();
            assert!(count == 0 || position.swiped_points() == step);
        }
    }

    pub fn next_step(&self) -> Self {
        let mut next = Self::default();
        for (index, &count) in self.possibilities.iter().enumerate() {
            let current_position: Position = index.into();
            if count == 0 {
                continue; // this point is not yet swiped
            }
            for next_swiped_point in 0..9 {
                if let Some(next_position) = current_position.swipe_to(next_swiped_point) {
                    next.possibilities[next_position] += count;
                }
            }
        }
        next
    }
}

fn main() {
    fn display(step: u32, possibilities: u32) {
        println!( "{step} point swiped: {possibilities} possibilities");
    }

    let mut current = Step::init();

    current.validate(1);
    display(1, current.current_possibilities());

    for step in 2..5 {
        current = current.next_step();

        current.validate(step);
        display(step, current.current_possibilities());

        if step == 2 {
            assert!(current.current_possibilities() == 56);
        }

    }

    let mut sum_step_5_to_9 = 0;
    for step in 5..=9 {
        current = current.next_step();

        current.validate(step);
        display(step, current.current_possibilities());

        sum_step_5_to_9 += current.current_possibilities();
    }

    println!("total of possible combination: {sum_step_5_to_9}");
}
