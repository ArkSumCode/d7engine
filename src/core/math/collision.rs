/// returns the distance 
/// between two points in a 2D space
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let b = x1 - x2;
    let b = b.powi(2);
    let c = y1 - y2;
    let c = c.powi(2);
    let a = b + c;
    a.sqrt()
}

/// to determin wether a point is in 
/// a rectangle shape or not
pub fn point_in_rect(point_x: f32, point_y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
    let in_x_range = rect_x <= point_x && point_x <= rect_x + rect_width;
    let in_y_range = rect_y <= point_y && point_y <= rect_y + rect_height;
    in_x_range && in_y_range
}

/// returns true if given
/// point is in given circle
pub fn point_in_circle(x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) -> bool {
    let d = distance(x1, y1, x2, y2);
    d < radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_in_rect() {
        let rect_x = 1.0;
        let rect_y = 1.0;
        let rect_width = 100.0;
        let rect_height = 100.0;

        assert_eq!(true, point_in_rect(2.0, 1.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(false, point_in_rect(0.5, 50.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(true, point_in_rect(101.0, 101.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(false, point_in_rect(101.0, 102.0, rect_x, rect_y, rect_width, rect_height));
        assert_eq!(true, point_in_rect(1.0, 1.0, rect_x, rect_y, rect_width, rect_height));
    } 

    #[test]
    fn test_distance() {
        assert_eq!(10.0, distance(0.0, 0.0, 10.0, 0.0));
        assert_eq!(26.196373, distance(-7.0, -4.0, 17.0, 6.5));
    }

    #[test]
    fn test_point_in_circle() {
        let (cirlce_x, circle_y) = (5.0, 6.0);
        let circle_r = 3.0;
        assert_eq!(true, point_in_circle(4.0, 4.0, cirlce_x, circle_y, circle_r));
        assert_eq!(false, point_in_circle(15.0, 15.0, cirlce_x, circle_y, circle_r));
    }
}