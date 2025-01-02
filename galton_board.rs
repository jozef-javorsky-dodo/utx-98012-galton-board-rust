use image::{ImageBuffer, Rgb};
use rand::{rngs::StdRng, Rng, SeedableRng};

const BOARD_WIDTH: u32 = 800;
const BOARD_HEIGHT: u32 = 400;
const NUM_BALLS: u32 = 10000;
const IMAGE_PATH: &str = "galton_board.png";

struct GaltonBoard {
    board_width: u32,
    board_height: u32,
    num_balls: u32,
    column_counts: Vec<u32>,
    rng: StdRng,
}

impl GaltonBoard {
    fn new(board_width: u32, board_height: u32, num_balls: u32) -> Self {
        GaltonBoard {
            board_width,
            board_height,
            num_balls,
            column_counts: vec![0; board_width as usize],
            rng: StdRng::from_entropy(),
        }
    }

    fn run_simulation(&mut self) {
        (0..self.num_balls).for_each(|_| {
            let final_column = self.calculate_final_column();
            self.column_counts[final_column as usize] += 1;
        });
    }

    fn calculate_final_column(&mut self) -> i32 {
        let mut column_index = (self.board_width / 2) as i32;
        (0..self.board_height).for_each(|_| {
            column_index += if self.rng.gen::<bool>() { -1 } else { 1 };
        });
        column_index.min(self.board_width as i32 - 1).max(0)
    }

    fn generate_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let max_count = *self.column_counts.iter().max().unwrap_or(&0);
        let mut image_buffer = ImageBuffer::new(self.board_width, self.board_height);

        for (column_index, &count) in self.column_counts.iter().enumerate() {
            let bar_height = (count as f32 / max_count as f32 * self.board_height as f32) as u32;
            for row_index in 0..bar_height {
                let pixel = image_buffer
                    .get_pixel_mut(column_index as u32, self.board_height - 1 - row_index);
                *pixel = Rgb([255, 0, 0]);
            }
        }
        image_buffer
    }
}

fn main() {
    let mut galton_board = GaltonBoard::new(BOARD_WIDTH, BOARD_HEIGHT, NUM_BALLS);
    galton_board.run_simulation();
    let image = galton_board.generate_image();

    if image.save(IMAGE_PATH).is_err() {
        eprintln!("Error saving the image to {}", IMAGE_PATH);
        std::process::exit(1);
    }

    println!(
        "Galton Board simulation completed. Image saved to: {}",
        IMAGE_PATH
    );
}
