const START: usize = 0;
const MIDDLE: usize = 1;
const END: usize = 2;

#[derive(Debug, PartialEq, Eq)]
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

    fn rotate_front(&mut self, counterclockwise: bool) {
        let switching_start = if counterclockwise { END } else { START };
        let switching_end = if counterclockwise { START } else { END };

        self.0[START][START][END].x = self.0[switching_end][switching_start][END].y;
        self.0[START][START][END].y = self.0[switching_end][switching_start][END].x;

        self.0[START][MIDDLE][END].x = self.0[MIDDLE][switching_start][END].y;

        self.0[START][END][END].x = self.0[switching_start][switching_start][END].y;
        self.0[START][END][END].y = self.0[switching_start][switching_start][END].x;

        // ---

        self.0[MIDDLE][START][END].y = self.0[switching_end][MIDDLE][END].x;

        self.0[MIDDLE][END][END].y = self.0[switching_start][MIDDLE][END].x;

        // ---

        self.0[END][START][END].x = self.0[switching_end][switching_end][END].y;
        self.0[END][START][END].y = self.0[switching_end][switching_end][END].x;

        self.0[END][MIDDLE][END].x = self.0[MIDDLE][switching_end][END].y;

        self.0[END][END][END].x = self.0[switching_start][switching_end][END].y;
        self.0[END][END][END].y = self.0[switching_start][switching_end][END].x;
    }

    fn rotate_right(&mut self, counterclockwise: bool) {
        let switching_start = if counterclockwise { END } else { START };
        let switching_end = if counterclockwise { START } else { END };

        self.0[END][START][START].y = self.0[END][switching_end][switching_start].z;
        self.0[END][START][START].z = self.0[END][switching_end][switching_start].y;

        self.0[END][START][MIDDLE].y = self.0[END][MIDDLE][switching_start].z;

        self.0[END][START][END].y = self.0[END][switching_start][switching_start].z;
        self.0[END][START][END].z = self.0[END][switching_start][switching_start].y;

        // ---

        self.0[END][MIDDLE][START].z = self.0[END][switching_end][MIDDLE].y;

        self.0[END][MIDDLE][END].z = self.0[END][switching_start][MIDDLE].y;

        // ---

        self.0[END][END][START].y = self.0[END][switching_end][switching_end].z;
        self.0[END][END][START].z = self.0[END][switching_end][switching_end].y;

        self.0[END][END][MIDDLE].y = self.0[END][MIDDLE][switching_end].z;

        self.0[END][END][END].y = self.0[END][switching_start][switching_end].z;
        self.0[END][END][END].z = self.0[END][switching_start][switching_end].y;
    }

    fn rotate_up(&mut self, counterclockwise: bool) {
        let switching_start = if counterclockwise { END } else { START };
        let switching_end = if counterclockwise { START } else { END };

        self.0[START][END][START].x = self.0[switching_start][END][switching_end].z;
        self.0[START][END][START].z = self.0[switching_start][END][switching_end].x;

        self.0[START][END][MIDDLE].x = self.0[MIDDLE][END][switching_end].z;

        self.0[START][END][END].x = self.0[switching_end][END][switching_end].z;
        self.0[START][END][END].z = self.0[switching_end][END][switching_end].x;

        // ---

        self.0[MIDDLE][END][START].z = self.0[switching_start][END][MIDDLE].x;

        self.0[MIDDLE][END][END].z = self.0[switching_end][END][MIDDLE].x;

        // ---

        self.0[END][END][START].x = self.0[switching_start][END][switching_start].z;
        self.0[END][END][START].z = self.0[switching_start][END][switching_start].x;

        self.0[END][END][MIDDLE].x = self.0[MIDDLE][END][switching_start].z;

        self.0[END][END][END].x = self.0[switching_end][END][switching_start].z;
        self.0[END][END][END].z = self.0[switching_end][END][switching_start].x;
    }

    fn rotate_left(&mut self, counterclockwise: bool) {
        todo!();
    }

    fn rotate_back(&mut self, counterclockwise: bool) {
        todo!();
    }

    fn rotate_down(&mut self, counterclockwise: bool) {
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Green,
    Red,
    White,
    Orange,
    Blue,
    Yellow,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_up() {
        let mut rubiks_cube = RubiksCube::new();

        rubiks_cube.rotate_up(false);

        assert_eq!(
            rubiks_cube,
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
                        Cube::new(Color::Green, Color::White, Color::Orange),
                        Cube::new(Color::Green, Color::White, Color::None),
                        Cube::new(Color::Green, Color::White, Color::Red),
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
                        Cube::new(Color::None, Color::White, Color::Orange),
                        Cube::new(Color::None, Color::White, Color::None),
                        Cube::new(Color::None, Color::White, Color::Red),
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
                        Cube::new(Color::Blue, Color::White, Color::Orange),
                        Cube::new(Color::Blue, Color::White, Color::None),
                        Cube::new(Color::Blue, Color::White, Color::Red),
                    ],
                ],
            ]),
        );

        rubiks_cube.rotate_up(true);
        rubiks_cube.rotate_up(true);

        assert_eq!(
            rubiks_cube,
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
                        Cube::new(Color::Blue, Color::White, Color::Red),
                        Cube::new(Color::Blue, Color::White, Color::None),
                        Cube::new(Color::Blue, Color::White, Color::Orange),
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
                        Cube::new(Color::None, Color::White, Color::Red),
                        Cube::new(Color::None, Color::White, Color::None),
                        Cube::new(Color::None, Color::White, Color::Orange),
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
                        Cube::new(Color::Green, Color::White, Color::Red),
                        Cube::new(Color::Green, Color::White, Color::None),
                        Cube::new(Color::Green, Color::White, Color::Orange),
                    ],
                ],
            ]),
        );
    }
}
