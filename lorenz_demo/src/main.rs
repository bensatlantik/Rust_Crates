use plotters::prelude::*;

fn main() {
    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;

    let dt = 0.01;
    let steps = 10000;

    let mut x = 1.0;
    let mut y = 1.0;
    let mut z = 1.0;

    let mut data = Vec::new();

    for _ in 0..steps {
        let dx = sigma * (y - x);
        let dy = x * (rho - z) - y;
        let dz = x * y - beta * z;

        x += dx * dt;
        y += dy * dt;
        z += dz * dt;

        data.push((x, y, z));
    }

    plot_lorenz(&data).expect("Plotting failed");
}

fn plot_lorenz(data: &[(f64, f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("lorenz.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (min_x, max_x) = (-30.0, 30.0);
    let (min_y, max_y) = (-30.0, 30.0);

    let mut chart = ChartBuilder::on(&root)
        .caption("Lorenz Attractor", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        data.iter().map(|(x, y, _z)| (*x, *y)),
        &RED,
    ))?;

    root.present()?;
    println!("Plot saved to 'lorenz.png'");

    Ok(())
}
