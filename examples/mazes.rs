use rand::random_range;
use rusty_console_game_engine::*;

const CELL_PATH_N: i32 = 0x01;
const CELL_PATH_E: i32 = 0x02;
const CELL_PATH_S: i32 = 0x04;
const CELL_PATH_W: i32 = 0x08;
const CELL_VISITED: i32 = 0x10;

struct Maze {
    maze_width: i32,
    maze_height: i32,
    maze: Vec<i32>,

    visited_cells: i32,
    stack: Vec<(i32, i32)>,
    path_width: i32,

    speed: f32,
    step_timer: f32,
}

impl Maze {
    fn new() -> Self {
        Self {
            maze_width: 0,
            maze_height: 0,
            maze: Vec::new(),
            visited_cells: 0,
            stack: Vec::new(),
            path_width: 0,
            speed: 0.05,
            step_timer: 0.0,
        }
    }
}

impl ConsoleGame for Maze {
    fn app_name(&self) -> &str {
        "Maze"
    }

    fn create(&mut self, _engine: &mut ConsoleGameEngine<Self>) -> bool {
        self.maze_width = 40;
        self.maze_height = 25;
        self.path_width = 3;
        self.maze = vec![0; (self.maze_width * self.maze_height) as usize];

        let x = random_range(0..self.maze_width);
        let y = random_range(0..self.maze_height);

        self.stack.push((x, y));
        self.maze[(y * self.maze_width + x) as usize] = CELL_VISITED;
        self.visited_cells = 1;

        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool {
        if engine.key_pressed(K_UP) {
            self.speed = (self.speed * 0.8).max(0.003);
        }
        if engine.key_pressed(K_DOWN) {
            self.speed = (self.speed * 1.25).min(1.0);
        }
        if engine.key_pressed(K_SPACE) {
            self.maze.fill(0);
            self.stack.clear();
            let x = random_range(0..self.maze_width);
            let y = random_range(0..self.maze_height);
            self.stack.push((x, y));
            self.maze[(y * self.maze_width + x) as usize] = CELL_VISITED;
            self.visited_cells = 1;
        }

        self.step_timer += elapsed_time;
        if self.step_timer < self.speed {
            return true;
        }
        self.step_timer = 0.0;

        let offset = |x: i32, y: i32, stack: &[(i32, i32)], width: i32| -> usize {
            ((stack.last().unwrap().1 + y) * width + (stack.last().unwrap().0 + x)) as usize
        };

        if self.visited_cells < self.maze_width * self.maze_height {
            let mut neighbors = Vec::new();

            if self.stack.last().unwrap().1 > 0
                && (self.maze[offset(0, -1, &self.stack, self.maze_width)] & CELL_VISITED) == 0
            {
                neighbors.push(0);
            }
            if self.stack.last().unwrap().0 < self.maze_width - 1
                && (self.maze[offset(1, 0, &self.stack, self.maze_width)] & CELL_VISITED) == 0
            {
                neighbors.push(1);
            }
            if self.stack.last().unwrap().1 < self.maze_height - 1
                && (self.maze[offset(0, 1, &self.stack, self.maze_width)] & CELL_VISITED) == 0
            {
                neighbors.push(2);
            }
            if self.stack.last().unwrap().0 > 0
                && (self.maze[offset(-1, 0, &self.stack, self.maze_width)] & CELL_VISITED) == 0
            {
                neighbors.push(3);
            }

            if !neighbors.is_empty() {
                let next_dir = neighbors[random_range(0..neighbors.len())];

                match next_dir {
                    0 => {
                        self.maze[offset(0, -1, &self.stack, self.maze_width)] |=
                            CELL_VISITED | CELL_PATH_S;
                        self.maze[offset(0, 0, &self.stack, self.maze_width)] |= CELL_PATH_N;
                        self.stack.push((
                            self.stack.last().unwrap().0,
                            self.stack.last().unwrap().1 - 1,
                        ));
                    }
                    1 => {
                        self.maze[offset(1, 0, &self.stack, self.maze_width)] |=
                            CELL_VISITED | CELL_PATH_W;
                        self.maze[offset(0, 0, &self.stack, self.maze_width)] |= CELL_PATH_E;
                        self.stack.push((
                            self.stack.last().unwrap().0 + 1,
                            self.stack.last().unwrap().1,
                        ));
                    }
                    2 => {
                        self.maze[offset(0, 1, &self.stack, self.maze_width)] |=
                            CELL_VISITED | CELL_PATH_N;
                        self.maze[offset(0, 0, &self.stack, self.maze_width)] |= CELL_PATH_S;
                        self.stack.push((
                            self.stack.last().unwrap().0,
                            self.stack.last().unwrap().1 + 1,
                        ));
                    }
                    3 => {
                        self.maze[offset(-1, 0, &self.stack, self.maze_width)] |=
                            CELL_VISITED | CELL_PATH_E;
                        self.maze[offset(0, 0, &self.stack, self.maze_width)] |= CELL_PATH_W;
                        self.stack.push((
                            self.stack.last().unwrap().0 - 1,
                            self.stack.last().unwrap().1,
                        ));
                    }
                    _ => {}
                }

                self.visited_cells += 1;
            } else {
                self.stack.pop();
            }
        }

        engine.clear(FG_BLACK);

        for x in 0..self.maze_width {
            for y in 0..self.maze_height {
                for py in 0..self.path_width {
                    for px in 0..self.path_width {
                        if self.maze[(y * self.maze_width + x) as usize] & CELL_VISITED != 0 {
                            engine.draw(
                                x * (self.path_width + 1) + px,
                                y * (self.path_width + 1) + py,
                            );
                        } else {
                            engine.draw_with(
                                x * (self.path_width + 1) + px,
                                y * (self.path_width + 1) + py,
                                PIXEL_SOLID,
                                FG_BLUE,
                            );
                        }
                    }
                }

                for p in 0..self.path_width {
                    if self.maze[(y * self.maze_width + x) as usize] & CELL_PATH_S != 0 {
                        engine.draw(
                            x * (self.path_width + 1) + p,
                            y * (self.path_width + 1) + self.path_width,
                        );
                    }
                    if self.maze[(y * self.maze_width + x) as usize] & CELL_PATH_E != 0 {
                        engine.draw(
                            x * (self.path_width + 1) + self.path_width,
                            y * (self.path_width + 1) + p,
                        );
                    }
                }
            }
        }

        for py in 0..self.path_width {
            for px in 0..self.path_width {
                let (cx, cy) = *self.stack.last().unwrap();
                engine.draw_with(
                    cx * (self.path_width + 1) + px,
                    cy * (self.path_width + 1) + py,
                    PIXEL_SOLID,
                    FG_GREEN,
                );
            }
        }

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(Maze::new());
    engine
        .construct_console(160, 100, 8, 8)
        .expect("Console Construction Failed");
    engine.start();
}
