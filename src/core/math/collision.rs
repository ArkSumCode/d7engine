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