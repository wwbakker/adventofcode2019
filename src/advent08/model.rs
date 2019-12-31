use crate::advent08::model::GrayScale::*;
use std::slice::Iter;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Image<T> where T: ColorScheme<T> + PartialEq + Copy + fmt::Display {
    pub layers: Vec<Layer<T>>,
    pub width: i32,
    pub height: i32,
}

impl<T> Image<T> where T: ColorScheme<T> + PartialEq + Copy + fmt::Display {
    pub fn from_vec(v: Vec<i32>, width: i32, height: i32) -> Image<T> {
        let layers = v
            .chunks((width * height) as usize)
            .map(|c| Layer::from_numbers_vec(c.to_vec()))
            .collect();
        Image { width, height, layers }
    }

    pub fn fewest_of_color(&self, color: T) -> &Layer<T> {
        self.layers
            .iter()
            .min_by(
                |l1, l2| l1.count_color(color).cmp(&l2.count_color(color)))
            .unwrap()
    }

    pub fn print(&self) {
        let flattened_layer = self.flatten();
        flattened_layer.values.chunks(self.width as usize).for_each(|c| {c.iter().for_each(|color| print!("{}", color)); println!(); } );
    }

    pub fn flatten(&self) -> Layer<T> {
        let range = (0..self.width * self.height);
        let flattened_vector = range.map(|i| self.get_flattened_pixel(i as usize)).collect();
        Layer::from_vec(flattened_vector)
    }

    fn get_flattened_pixel(&self, index: usize) -> T {
        self.layers.iter().map(|l| l.values[index]).filter(|p| !T::is_transparent(p)).next().unwrap_or(T::transparent_color())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GrayScale {
    BLACK,
    WHITE,
    TRANSPARENT,
}

pub trait ColorScheme<T> where T: PartialEq + Copy {
    fn from_number(number: i32) -> T;
    fn is_transparent(color: &T) -> bool;
    fn transparent_color() -> T;
}

impl ColorScheme<GrayScale> for GrayScale {
    fn from_number(number: i32) -> GrayScale {
        match number {
            0 => GrayScale::BLACK,
            1 => GrayScale::WHITE,
            2 => GrayScale::TRANSPARENT,
            _ => panic!("Unknown number for enum GrayScale")
        }
    }

    fn is_transparent(color: &GrayScale) -> bool {
        *color == TRANSPARENT
    }

    fn transparent_color() -> GrayScale {
        TRANSPARENT
    }
}

impl fmt::Display for GrayScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            BLACK => "░",
            WHITE => "█",
            TRANSPARENT => " "
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, PartialEq)]
pub struct Layer<T> where T: ColorScheme<T> + PartialEq + Copy {
    pub values: Vec<T>
}

impl<T> Layer<T> where T: ColorScheme<T> + PartialEq + Copy {
    pub fn from_numbers_vec(numbers: Vec<i32>) -> Layer<T> {
        let vec: Vec<T> = numbers.iter().map(|n| T::from_number(*n)).collect();
        Layer::from_vec(vec)
    }

    pub fn from_vec(values: Vec<T>) -> Layer<T> {
        Layer { values }
    }

    pub fn count_color(&self, color: T) -> i32 {
        self.values.iter().filter(|v| **v == color).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let expected_image = Image {
            layers: vec!(
                Layer { values: vec!(WHITE, TRANSPARENT, BLACK, WHITE) },
                Layer { values: vec!(TRANSPARENT, WHITE, BLACK, WHITE) }),
            width: 2,
            height: 2,
        };
        let image: Image<GrayScale> = Image::from_vec(vec!(1, 2, 0, 1, 2, 1, 0, 1), 2, 2);
        assert_eq!(image, expected_image)
    }

    #[test]
    fn test_layer_digit_count() {
        let layer = Layer { values: vec!(BLACK, WHITE, TRANSPARENT, TRANSPARENT) };
        assert_eq!(layer.count_color(TRANSPARENT), 2)
    }

    #[test]
    fn test_layer_fewest_zero_digit_count() {
        let image: Image<GrayScale> = Image::from_vec(vec!(0, 2, 2, 2, 0, 2, 0, 2, 2, 1, 1, 2, 0, 1, 0, 1), 2, 2);
        assert_eq!(*image.fewest_of_color(BLACK), Layer { values: vec!(TRANSPARENT, WHITE, WHITE, TRANSPARENT) })
    }

    #[test]
    fn test_image_flatten() {
        let image: Image<GrayScale> = Image::from_vec(vec!(0, 2, 2, 2,
                                                           0, 2, 0, 2,
                                                           2, 1, 1, 2,
                                                           2, 1, 0, 2), 2, 2);
        assert_eq!(image.flatten(), Layer { values: vec!(BLACK, WHITE, BLACK, TRANSPARENT) })
    }
}