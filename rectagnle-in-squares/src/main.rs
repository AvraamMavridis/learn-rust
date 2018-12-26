use std::cmp::min;

trait Shape {
   fn new(lng: i32, wdth: i32) -> Self;
   fn get_number_of_squares(&self) -> i32;
}

#[derive(Debug)]
struct Rectangle {
    lng: i32,
    wdth: i32,
}

#[derive(Debug)]
struct Square {
    lng: i32,
    wdth: i32,
}

impl Shape for Rectangle {
    fn new(lng: i32, wdth: i32) -> Rectangle {
        Rectangle { lng, wdth }
    }

    fn get_number_of_squares(&self) -> i32 {
        self.lng * self.wdth
    }
}

impl Shape for Square {
    fn new(lng: i32, wdth: i32) -> Square {
        Square { lng, wdth }
    }

    fn get_number_of_squares(&self) -> i32 {
        self.lng * self.wdth
    }
}

impl Rectangle {
    fn get_max_square_that_fits_in(&self) -> Square {
        let min_side = min(self.lng, self.wdth);
        Square {
            lng: min_side,
            wdth: min_side,
        }
    }
}


fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth  {
        return None;
    }

  let mut width = wdth;
  let mut long = lng;
  let mut rect = Rectangle::new(lng, wdth);
  let mut number_of_squares_to_cover = rect.get_number_of_squares();
  let mut result: Vec<i32> = Vec::new();

  loop {
    let square = rect.get_max_square_that_fits_in();
    number_of_squares_to_cover -= square.get_number_of_squares();
    if width> square.wdth {
        width -= square.wdth;
    }

    if long> square.lng {
        long -= square.lng;
    }

    result.push(square.lng);

    if number_of_squares_to_cover <= 0 || width == 0 || long == 0 {
        break;
    }

    rect = Rectangle::new(long, width);
  }

  
  Some(result)
}



#[cfg(test)]
mod test {
    use super::*;

    fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
      assert_eq!(sq_in_rect(lng, wdth), exp)
    }

    #[test]
    fn tests_sq_in_rect() {

        testing(5, 3, Some(vec![3, 2, 1, 1]));
        testing(3, 5, Some(vec![3, 2, 1, 1]));
        testing(5, 5, None);
    
    }
}



fn main() {
    println!("Run the tests");
}
