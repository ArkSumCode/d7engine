/*
classic collision algorithm
to determin wether a point is in 
a rectangle shape
*/
pub fn point_in_rect(point_x: f32, point_y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
    let in_x_range = rect_x <= point_x && point_x <= rect_x + rect_width;
    let in_y_range = rect_y <= point_y && point_y <= rect_y + rect_height;
    in_x_range && in_y_range
}

pub trait Collision {
    fn collides(&self, x: f32, y: f32) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_point_in_rect() {
        let rect_x = 1.0;
        let rect_y = 1.0;
        let rect_width = 100.0;
        let rect_height = 100.0;

        assert_eq!(true, collision::point_in_rect(2.0, 1.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(false, collision::point_in_rect(0.5, 50.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(true, collision::point_in_rect(101.0, 101.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(false, collision::point_in_rect(101.0, 102.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(true, collision::point_in_rect(1.0, 1.0, rect_x, rect_y, rect_width, rect_height));
    } 
}