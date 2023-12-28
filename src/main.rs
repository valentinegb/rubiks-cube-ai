const START: usize = 0;
const MIDDLE: usize = 1;
const END: usize = 2;

struct RubiksCube(Vec<Vec<Vec<Cube>>>);

impl RubiksCube {
    fn new() -> Self {
        RubiksCube(vec![
            vec![
                vec![
                    Cube::new(Color::Orange, Color::Yellow, Color::Blue),
                    Cube::new(Color::Orange, Color::Yellow, Color::None),
                    Cube::new(Color::Orange, Color::Yellow, Color::Green),
                ],
                vec![
                    Cube::new(Color::Orange, Color::None, Color::Blue),
                    Cube::new(Color::Orange, Color::None, Color::None),
                    Cube::new(Color::Orange, Color::None, Color::Green),
                ],
                vec![
                    Cube::new(Color::Orange, Color::White, Color::Blue),
                    Cube::new(Color::Orange, Color::White, Color::None),
                    Cube::new(Color::Orange, Color::White, Color::Green),
                ],
            ],
            vec![
                vec![
                    Cube::new(Color::None, Color::Yellow, Color::Blue),
                    Cube::new(Color::None, Color::Yellow, Color::None),
                    Cube::new(Color::None, Color::Yellow, Color::Green),
                ],
                vec![
                    Cube::new(Color::None, Color::None, Color::Blue),
                    Cube::new(Color::None, Color::None, Color::None),
                    Cube::new(Color::None, Color::None, Color::Green),
                ],
                vec![
                    Cube::new(Color::None, Color::White, Color::Blue),
                    Cube::new(Color::None, Color::White, Color::None),
                    Cube::new(Color::None, Color::White, Color::Green),
                ],
            ],
            vec![
                vec![
                    Cube::new(Color::Red, Color::Yellow, Color::Blue),
                    Cube::new(Color::Red, Color::Yellow, Color::None),
                    Cube::new(Color::Red, Color::Yellow, Color::Green),
                ],
                vec![
                    Cube::new(Color::Red, Color::None, Color::Blue),
                    Cube::new(Color::Red, Color::None, Color::None),
                    Cube::new(Color::Red, Color::None, Color::Green),
                ],
                vec![
                    Cube::new(Color::Red, Color::White, Color::Blue),
                    Cube::new(Color::Red, Color::White, Color::None),
                    Cube::new(Color::Red, Color::White, Color::Green),
                ],
            ],
        ])
    }

    fn rotate_front_clockwise(&mut self) {
        self.0[START][START][END].x = self.0[END][START][END].y;
        self.0[START][START][END].y = self.0[END][START][END].x;

        self.0[START][MIDDLE][END].x = self.0[MIDDLE][START][END].y;

        self.0[START][END][END].x = self.0[START][START][END].y;
        self.0[START][END][END].y = self.0[START][START][END].x;

        // ---

        self.0[MIDDLE][START][END].y = self.0[END][MIDDLE][END].x;

        self.0[MIDDLE][END][END].y = self.0[START][MIDDLE][END].x;

        // ---

        self.0[END][START][END].x = self.0[END][END][END].y;
        self.0[END][START][END].y = self.0[END][END][END].x;

        self.0[END][MIDDLE][END].x = self.0[MIDDLE][END][END].y;

        self.0[END][END][END].x = self.0[START][END][END].y;
        self.0[END][END][END].y = self.0[START][END][END].x;
    }

    fn rotate_front_counterclockwise(&mut self) {
        self.0[START][START][END].x = self.0[START][END][END].y;
        self.0[START][START][END].y = self.0[START][END][END].x;

        self.0[START][MIDDLE][END].x = self.0[MIDDLE][END][END].y;

        self.0[START][END][END].x = self.0[END][END][END].y;
        self.0[START][END][END].y = self.0[END][END][END].x;

        // ---

        self.0[MIDDLE][START][END].y = self.0[START][MIDDLE][END].x;

        self.0[MIDDLE][END][END].y = self.0[END][MIDDLE][END].x;

        // ---

        self.0[END][START][END].x = self.0[START][START][END].y;
        self.0[END][START][END].y = self.0[START][START][END].x;

        self.0[END][MIDDLE][END].x = self.0[MIDDLE][START][END].y;

        self.0[END][END][END].x = self.0[END][START][END].y;
        self.0[END][END][END].y = self.0[END][START][END].x;
    }

    fn rotate_right_clockwise(&mut self) {
        todo!();
    }

    fn rotate_right_counterclockwise(&mut self) {
        todo!();
    }

    fn rotate_up_clockwise(&mut self) {
        todo!();
    }

    fn rotate_up_counterclockwise(&mut self) {
        todo!();
    }

    fn rotate_left_clockwise(&mut self) {
        todo!();
    }

    fn rotate_left_counterclockwise(&mut self) {
        todo!();
    }

    fn rotate_back_clockwise(&mut self) {
        todo!();
    }

    fn rotate_back_counterclockwise(&mut self) {
        todo!();
    }

    fn rotate_down_clockwise(&mut self) {
        todo!();
    }

    fn rotate_down_counterclockwise(&mut self) {
        todo!();
    }
}

struct Cube {
    x: Color,
    y: Color,
    z: Color,
}

impl Cube {
    fn new(x: Color, y: Color, z: Color) -> Self {
        Cube { x, y, z }
    }
}

#[derive(Clone, Copy)]
enum Color {
    Green,
    Red,
    White,
    Orange,
    Blue,
    Yellow,
    None,
}

fn main() {
    todo!();
}
