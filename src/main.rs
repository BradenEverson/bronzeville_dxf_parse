use dxf::entities::*;
use dxf::{Drawing, DxfResult};
use dxf::Point;

fn main() -> DxfResult<()> {
    let points: Vec<Point> = vec![Point::new(0.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(0.0, 0.0, 0.0)]; 
    let drawing = points_to_shape(points);
    drawing.save_file("triangle.dxf")?;

    Ok(())
}

pub fn points_to_shape(points: Vec<Point>) -> Drawing {
    let mut drawing = Drawing::new();
    for i in 0..points.len()-1 {
        let new_line = Line::new(points[i].clone(), points[i+1].clone());
        drawing.add_entity(Entity::new(EntityType::Line(new_line)));
    }
    drawing
}
