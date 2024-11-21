use plotters::prelude::*;

fn plot_equation<F>(equation: F, output: &str) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(f64) -> f64,
{
    let root_area = BitMapBackend::new(output, (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Equation Plotter", ("sans-serif", 50))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10.0..10.0, -10.0..100.0)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| x as f64 / 10.0).map(|x| (x, equation(x))),
        &BLUE,
    ))?
    .label("Equation")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parabola = |x: f64| 2.0 * x * x;
    plot_equation(parabola, "parabola.png")
}
