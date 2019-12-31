use crate::advent08::image_reading::read;
use crate::advent08::model::{Image, GrayScale};
use crate::advent08::model::GrayScale::*;

pub fn start_a(){
    let input_vector = read("input/advent08/input.txt").unwrap();
    let image : Image<GrayScale> = Image::from_vec(input_vector, 25, 6);
    let fewest_zero_digit_layer = image.fewest_of_color(BLACK);
    let number_of_one_digits = fewest_zero_digit_layer.count_color(WHITE);
    let number_of_two_digits = fewest_zero_digit_layer.count_color(TRANSPARENT);
    println!("day 8 answer is: {}", number_of_one_digits * number_of_two_digits);
}

pub fn start_b(){
    let input_vector = read("input/advent08/input.txt").unwrap();
    let image : Image<GrayScale> = Image::from_vec(input_vector, 25, 6);
    image.print();
}