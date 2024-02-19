use dxf::entities::*;
use dxf::{Drawing, DxfResult};
use dxf::Point;
use dxf_test::{point, shape};

fn main() -> DxfResult<()> {
    let drawing = shape!(point![0.0,0.0,0.0], point![0.0,1.0,0.0], point![1.0,1.0,0.0]);
    drawing.save_file("triangle_test_macro.dxf")?;

    Ok(())
}

