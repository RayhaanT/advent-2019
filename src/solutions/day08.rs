use crate::Solution;

pub fn solve(input: String) -> Solution {
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let pixels: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .rev()
        .collect();
    let mut image: Vec<u32> = vec![2; layer_size];

    let mut least_zeroes: Option<i32> = None;
    let mut least_zeroes_ind = 0;
    let mut pixel_ind = 0;
    while pixel_ind * layer_size < pixels.len() {
        let mut zero_count = 0;
        let start_ind = pixel_ind * layer_size;
        for i in start_ind..(start_ind + layer_size) {
            if pixels[i] == 0 {
                zero_count += 1;
            }
            if pixels[i] != 2 {
                image[(layer_size - 1) - (i - start_ind)] = pixels[i];
            }
        }
        if least_zeroes.is_none() || zero_count < least_zeroes.unwrap() {
            least_zeroes_ind = pixel_ind;
            least_zeroes = Some(zero_count);
        }
        pixel_ind += 1;
    }

    let mut ones = 0;
    let mut twos = 0;
    for i in (least_zeroes_ind * layer_size)..((least_zeroes_ind + 1) * layer_size) {
        if pixels[i] == 1 {
            ones += 1;
        }
        if pixels[i] == 2 {
            twos += 1;
        }
    }

    let mut printed: String = String::from("\n");
    for y in 0..height {
        for x in 0..width {
            printed = format!(
                "{}{}",
                printed,
                if image[x + y * width] == 0 { " " } else { "#" }
            );
        }
        printed = format!("{}\n", printed);
    }

    Solution {
        first: (ones * twos).to_string(),
        second: printed,
    }
}
