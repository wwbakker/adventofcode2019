use crate::advent08::image_reading::read;
use crate::advent08::model::Image;

pub fn start_a(){
    let input_vector = read("input/advent08/input.txt").unwrap();
    let image = Image::from_vec(input_vector, 25, 6);
    let fewest_zero_digit_layer = image.fewest_zero_digits();
    let number_of_one_digits = fewest_zero_digit_layer.count_digit(1);
    let number_of_two_digits = fewest_zero_digit_layer.count_digit(2);
    println!("day 8 answer is: {}", number_of_one_digits * number_of_two_digits);
}