use dxf::{Point, Drawing, entities::{Line, EntityType, Entity}};
#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point{
            x: $x,
            y: $y,
            z: $y
        }
    };
}

#[macro_export]
macro_rules! shape {
    ($( $point:expr ), * ) => {
        {
            let mut points = Vec::new();
            $(
                points.push($point);
             )*
                let mut drawing = Drawing::new();
            for i in 0..points.len()-1 {
                let new_line = Line::new(points[i].clone(), points[i+1].clone());
                drawing.add_entity(Entity::new(EntityType::Line(new_line)));
            }
            let new_line = Line::new(points[points.len()-1].clone(), points[0].clone());
            drawing.add_entity(Entity::new(EntityType::Line(new_line)));

            drawing
        }
    };
}
