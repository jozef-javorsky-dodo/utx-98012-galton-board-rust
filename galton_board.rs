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
    num_levels: u32,
    column_counts: Vec<u32>,
    rng: StdRng,
}

impl GaltonBoard {
    fn new(board_width: u32, board_height: u32, num_balls: u32) -> Self {
        let num_levels = board_height;
        GaltonBoard {
            board_width,
            board_height,
            num_balls,
            num_levels,
            column_counts: vec![0; (num_levels + 1) as usize],
            rng: StdRng::from_entropy(),
        }
    }

    fn run_simulation(&mut self) {
        for _ in 0..self.num_balls {
            let final_bin = self.calculate_final_bin();
            self.column_counts[final_bin] += 1;
        }
    }

    fn calculate_final_bin(&mut self) -> usize {
        (0..self.num_levels)
            .filter(|_| self.rng.gen::<bool>())
            .count()
    }

    fn generate_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let max_count = *self.column_counts.iter().max().unwrap_or(&1);
        let mut image_buffer = ImageBuffer::new(self.board_width, self.board_height);
        let num_bins = self.column_counts.len();
        let bar_width = self.board_width as f32 / num_bins as f32;

        for (bin_index, &count) in self.column_counts.iter().enumerate() {
            if count == 0 {
                continue;
            }
            let bar_height = (count as f32 / max_count as f32 * self.board_height as f32) as u32;
            let x_start = (bin_index as f32 * bar_width) as u32;
            let x_end = ((bin_index + 1) as f32 * bar_width) as u32;

            for x in x_start..x_end.min(self.board_width) {
                for y in 0..bar_height.min(self.board_height) {
                    let pixel = image_buffer.get_pixel_mut(x, self.board_height - 1 - y);
                    *pixel = Rgb([255, 0, 0]);
                }
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
