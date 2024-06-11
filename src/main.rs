use std::cmp::PartialEq;
use std::time::{Duration, Instant};
use macroquad::prelude::*;
use crate::Direction::{Down, Left, Right, Up};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 10;

fn conf() -> Conf {
    let mut window_conf = Conf::default();
    window_conf.window_resizable = false;
    window_conf.window_width = 500;
    window_conf.window_height = 500;
    window_conf
}

#[derive(PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

type Segments = Vec<Vec2>;

struct Snake {
    name: String,
    is_alive: bool,
    length: i32,
    speed: f32,
    head_img: Texture2D,
    body_img: Texture2D,
    tail_img: Texture2D,
    position: Vec2,
    matrix_position: Vec2,
    facing: Direction,
    segments: Segments,
    has_eaten: bool,
    tail_position: Vec2,
    score: i32,
    previous_direction_inverted: Direction
}

impl Snake {
    async fn new() -> Snake {
        // Load the textures for the snake parts
        let head_img = load_texture("assets/snake_head.png").await.expect("Failed to load snake head image");
        let body_img = load_texture("assets/snake_body.png").await.expect("Failed to load snake body image");
        let tail_img = load_texture("assets/snake_tail.png").await.expect("Failed to load snake tail image");

        let matrix_position = Vec2::new(2.0, 2.0);
        let position = Vec2::new(100.0, 100.0);

        let segments = vec![matrix_position];

        let tail_position = Vec2::default();

        let speed:f32 = 1.0;

        let has_eaten = false;

        let score = 0;

        let mut previous_direction_inverted = Direction::Left;

        Snake {
            name: String::from("snake"),
            is_alive: true,
            length: 1,
            speed,
            head_img,
            body_img,
            tail_img,
            position,
            matrix_position,
            facing: Direction::Right,
            segments,
            has_eaten,
            tail_position,
            score,
            previous_direction_inverted
        }
    }

    fn draw(&self) {
        for segment in &self.segments {
            draw_texture(&self.body_img, segment.x * 50.0, segment.y * 50.0, WHITE);
        }
        draw_texture(&self.head_img, self.position.x, self.position.y, WHITE);
        if self.segments.len() > 1 {
            let tail_index = self.segments.len() - 1;
            draw_texture(&self.tail_img, self.segments[tail_index].x * 50.0, self.segments[tail_index].y * 50.0, WHITE);
        }
    }

    fn is_out_of_bounds(&self) -> bool {
        self.segments[0].x < 0.0 || self.segments[0].x >= BOARD_WIDTH as f32 ||
            self.segments[0].y < 0.0 || self.segments[0].y >= BOARD_HEIGHT as f32
    }

    fn is_colliding_with_itself(&self) -> bool {
        for segment in self.segments.iter().skip(1).copied() {
            if self.segments[0] == segment {
                return true;
            }
        }
        false
    }

    fn is_on_snake(&self, position: Vec2) -> bool {
        for segment in &self.segments {
            if *segment == position {
                return true;
            }
        }
        false
    }
}

struct Apple {
    apple_img: Texture2D,
    position: Vec2,
    matrix_position: Vec2,
    collected: bool,
}

impl Apple {
    async fn new(snake: &Snake) -> Apple {
        let apple_img = load_texture("assets/apple.png").await.expect("Failed to load snake head image");

        let mut matrix_position;
        loop {
            matrix_position = Vec2::new(rand::gen_range(1, 10) as f32, rand::gen_range(1, 10) as f32);
            if !snake.is_on_snake(matrix_position) {
                break;
            }
        }
        let position = matrix_position * Vec2::new(50.0, 50.0);

        Apple {
            apple_img,
            position,
            matrix_position,
            collected: false,
        }
    }
    fn draw(&self) {
        draw_texture(&self.apple_img, self.position.x, self.position.y, WHITE);
    }
}


#[macroquad::main(conf)]
async fn main() {

    let background_image = load_texture("assets/board.png").await.unwrap();

    let mut snake = Snake::new().await;

    let mut apple = Apple::new(&snake).await;

    let mut game_over = false;

    let mut high_score = 0;

    let mut board: [[char; BOARD_WIDTH]; BOARD_HEIGHT] = [[' '; BOARD_WIDTH]; BOARD_HEIGHT];

    let mut last_update: Instant = Instant::now();

    loop {

        if !game_over {
            if is_key_pressed(KeyCode::S) && snake.previous_direction_inverted != Direction::Down && snake.facing != Direction::Down {
                snake.facing = Direction::Down;
            }
            if is_key_pressed(KeyCode::W) && snake.previous_direction_inverted != Direction::Up && snake.facing != Direction::Up {
                snake.facing = Direction::Up;
            }
            if is_key_pressed(KeyCode::A) && snake.previous_direction_inverted != Direction::Left && snake.facing != Direction::Left {
                snake.facing = Direction::Left;
            }
            if is_key_pressed(KeyCode::D) && snake.previous_direction_inverted != Direction::Right && snake.facing != Direction::Right {
                snake.facing = Direction::Right;
            }

            if Instant::now() - last_update >= Duration::from_secs_f32(0.5 * snake.speed) {
                let now = Instant::now();
                last_update = now;

                let mut new_head = snake.segments[0];

                match snake.facing {
                    Direction::Down => snake.previous_direction_inverted = Up,
                    Direction::Up => snake.previous_direction_inverted = Down,
                    Direction::Left => snake.previous_direction_inverted = Right,
                    Direction::Right => snake.previous_direction_inverted = Left,
                }

                match snake.facing {
                    Direction::Down => new_head.y += 1.0,
                    Direction::Up => new_head.y -= 1.0,
                    Direction::Left => new_head.x -= 1.0,
                    Direction::Right => new_head.x += 1.0,
                }

                snake.segments.insert(0, new_head);
                snake.position = new_head * Vec2::new(50.0, 50.0);
                if snake.score > 0 {
                    snake.tail_position = *snake.segments.last().unwrap() * Vec2::new(50.0, 50.0);
                }

                if !snake.has_eaten {
                    snake.segments.pop();
                } else {
                    snake.has_eaten = false;
                }
            }

            if apple.matrix_position == snake.segments[0] {
                apple = Apple::new(&snake).await;
                snake.length += 1;
                snake.score += 1;
                snake.speed -= 0.05;
                snake.has_eaten = true;
            }

            if snake.is_out_of_bounds() {
                println!("Snake is out of bounds!");
                game_over = true; // Exit the loop if snake is out of bounds
            }

            if snake.is_colliding_with_itself() {
                println!("Snake ate itself!");
                game_over = true;
            }

            clear_background(GRAY);

            draw_texture(&background_image, 0.0, 0.0, WHITE);

            board[snake.segments[0].x as usize][snake.segments[0].y as usize] = 's';

            for row in 0..BOARD_WIDTH {
                for col in 0..BOARD_HEIGHT {
                    print!("{}", board[col][row]);
                }
                println!();
            }

            apple.draw();

            snake.draw();

            draw_text(format!("Score: {}", snake.score).as_str(), 10.0, 20.0, 25.0, BLACK);

        } else {

            clear_background(GRAY);

            draw_texture(&background_image, 0.0, 0.0, WHITE);

            if snake.score > high_score {
                high_score = snake.score;
            }

            draw_text("You Died!", 40.0, 150.0, 110.0, RED);
            draw_text(format!("Your score is: {}", snake.score).as_str(), 70.0, 250.0, 50.0, BLACK);
            draw_text(format!("Your high score is: {}", high_score).as_str(),  35.0, 300.0, 50.0, BLACK);
            draw_text("Press 'Space' to play again",  45.0, 335.0, 35.0, DARKGRAY);

            if is_key_down(KeyCode::Space) {
                snake = Snake::new().await;
                apple = Apple::new(&snake).await;
                game_over = false;
            }
        }
        next_frame().await
    }
}