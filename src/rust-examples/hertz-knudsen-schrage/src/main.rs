use plotters::prelude::*;
use plotters::element::PathElement;
use statrs::function::erf::erf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate erf data on a dense grid for smooth plotting
    let xs: Vec<f64> = (-50..=50).map(|i| i as f64 / 10.0).collect(); // [-5, 5] step 0.1
    let data: Vec<(f64, f64)> = xs.iter().map(|&x| (x, erf(x))).collect();

    let x_min = -5.0;
    let x_max = 5.0;
    let y_min = -1.0;
    let y_max = 1.0;

    // Render to an SVG file (no system fontconfig dependency)
    // 140 mm / 25.4 * 96 * 4 = 2116 
    // 60 mm / 25.4 * 96 * 4 = 907
    let filename = "hertz-knudsen-schrage-1.svg";
    let root = SVGBackend::new(filename, (2116, 907)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .margin_left(150)   
        .margin_right(180)  
        .margin_top(32)     
        .margin_bottom(0)  
        .x_label_area_size(120)
        .y_label_area_size(176)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Configure only axes; we'll draw custom grid lines with explicit intervals
    chart
        .configure_mesh()
        .x_desc("x")
        .y_desc("erf(x)")
        .axis_desc_style(("Arial", 64))  // 16*4
        .label_style(("Arial", 64))  // 16*4
        .x_label_formatter(&|x| format!("{:.1}", x).replace('-', "−")) // Unicode minus sign
        .y_label_formatter(&|y| format!("{:.1}", y).replace('-', "−")) // Unicode minus sign
        .disable_mesh()
        .draw()?;

    // Explicit minor/major grid intervals
    let dx_minor = 0.5; // set desired x minor interval
    let dy_minor = 0.1; // set desired y minor interval
    let dx_major = 1.0; // set desired x major interval
    let dy_major = 0.5; // set desired y major interval

    // Helper to generate a sequence from the first multiple within range
    let gen_ticks = |min_v: f64, max_v: f64, step: f64| -> Vec<f64> {
        if step <= 0.0 {
            return Vec::new();
        }
        let start = (min_v / step).ceil() * step;
        let mut v = start;
        let mut out = Vec::new();
        // Add a small epsilon to avoid floating drift missing the last line
        while v <= max_v + step * 1e-6 {
            // Round to 12 dp to reduce float artifacts in SVG
            out.push((v * 1e12).round() / 1e12);
            v += step;
        }
        out
    };

    // Minor gridlines (draw first)
    let x_minor = gen_ticks(x_min, x_max, dx_minor);
    let y_minor = gen_ticks(y_min, y_max, dy_minor);
    chart.draw_series(
        x_minor.into_iter().map(|x| PathElement::new(vec![(x, y_min), (x, y_max)], &BLACK.mix(0.08)))
    )?;
    chart.draw_series(
        y_minor.into_iter().map(|y| PathElement::new(vec![(x_min, y), (x_max, y)], &BLACK.mix(0.08)))
    )?;

    // Major gridlines (thicker/lighter differently)
    let x_major = gen_ticks(x_min, x_max, dx_major);
    let y_major = gen_ticks(y_min, y_max, dy_major);
    chart.draw_series(
        x_major.into_iter().map(|x| PathElement::new(vec![(x, y_min), (x, y_max)], &BLACK.mix(0.18)))
    )?;
    chart.draw_series(
        y_major.into_iter().map(|y| PathElement::new(vec![(x_min, y), (x_max, y)], &BLACK.mix(0.18)))
    )?;

    // Draw scatter points instead of a line
    // BLACK, WHITE, RED, GREEN, BLUE, CYAN, MAGENTA, YELLOW
    chart.draw_series(
        data.iter().map(|&(x, y)| Circle::new((x, y), 8, BLUE.filled()))  // 2*4
    )?;

    // Axis zero lines for reference (draw on top)
    chart.draw_series(vec![
        PathElement::new(vec![(x_min, 0.0), (x_max, 0.0)], &BLACK.mix(0.35)),
        PathElement::new(vec![(0.0, y_min), (0.0, y_max)], &BLACK.mix(0.35)),
    ])?;

    root.present()?;
    Ok(())
}