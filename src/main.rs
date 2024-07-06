use plotters::prelude::*;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("bezier_curve.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Curva de Bézier", ("sans-serif", 50))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1.5..1.5, -1.5..1.5)?;

    chart.configure_mesh().draw()?;

    let p0 = (0.0, 0.0);
    let p1 = (1.0, 2.0);
    let p2 = (2.0, 2.0);
    let p3 = (3.0, 0.0);

    let bezier_curve = |t: f64| {
        let (x0, y0) = p0;
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        let (x3, y3) = p3;
        
        let x = (1.0 - t).powi(3) * x0
              + 3.0 * (1.0 - t).powi(2) * t * x1
              + 3.0 * (1.0 - t) * t.powi(2) * x2
              + t.powi(3) * x3;

        let y = (1.0 - t).powi(3) * y0
              + 3.0 * (1.0 - t).powi(2) * t * y1
              + 3.0 * (1.0 - t) * t.powi(2) * y2
              + t.powi(3) * y3;

        (x, y)
    };

    chart.draw_series(LineSeries::new(
        (0..=1000).map(|i| {
            let t = i as f64 / 1000.0;
            bezier_curve(t)
        }),
        &RED,
    ))?;

    root_area.present()?;

    println!("El gráfico se ha guardado en 'bezier_curve.png'");

    Ok(())
}
