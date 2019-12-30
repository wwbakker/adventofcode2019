#[derive(Debug, PartialEq)]
pub struct Image {
    pub layers: Vec<Layer>,
    pub width: i32,
    pub height: i32,
}

impl Image {
    pub fn from_vec(v: Vec<i32>, width: i32, height: i32) -> Image {
        let layers = v
            .chunks((width * height) as usize)
            .map(|c| Layer::from_vec(c.to_vec()))
            .collect();
        Image { width, height, layers }
    }

    pub fn fewest_zero_digits(&self) -> &Layer {
        self.layers
            .iter()
            .min_by(
                |l1, l2| l1.count_digit(0).cmp(&l2.count_digit(0)))
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub values: Vec<i32>
}

impl Layer {
    pub fn from_vec(values: Vec<i32>) -> Layer {
        Layer { values }
    }

    pub fn count_digit(&self, digit: i32) -> i32 {
        self.values.iter().filter(|v| **v == digit).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let expected_image = Image {
            layers: vec!(
                Layer { values: vec!(1, 2, 3, 4) },
                Layer { values: vec!(5, 6, 7, 8) }),
            width: 2,
            height: 2,
        };
        let image = Image::from_vec(vec!(1, 2, 3, 4, 5, 6, 7, 8), 2, 2);
        assert_eq!(image, expected_image)
    }

    #[test]
    fn test_layer_digit_count() {
        let layer = Layer { values: vec!(1, 1, 0, 0) };
        assert_eq!(layer.count_digit(1), 2)
    }

    #[test]
    fn test_layer_fewest_zero_digit_count() {
        let image = Image {
            layers: vec!(
                Layer { values: vec!(0, 2, 3, 4) },
                Layer { values: vec!(0, 2, 0, 4) },
                Layer { values: vec!(5, 1, 1, 8) },
                Layer { values: vec!(0, 3, 0, 6) }),
            width: 2,
            height: 2,
        };
        assert_eq!(*image.fewest_zero_digits(), Layer { values: vec!(5, 1, 1, 8) })
    }
}