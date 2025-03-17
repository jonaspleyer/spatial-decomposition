use spatial_decomposition::{Rectangle, kong_mount_roscoe};

use plotters::prelude::*;

pub fn plot_decomposed_rectangle(
    rectangle: Rectangle<f64>,
    n_subdomains: core::num::NonZeroUsize,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let Rectangle { min, max } = rectangle;
    let ratio = (max[1] - min[1]) / (max[0] - min[0]);
    let nx = 1000;
    let ny = (ratio * nx as f64).round() as u32;
    let root = BitMapBackend::new(filename, (nx, ny)).into_drawing_area();
    root.fill(&WHITE)?;

    /* let chart = ChartBuilder::on(&root)
    .margin(0)
    .build_cartesian_2d(min[0]..max[0], min[1]..max[1])?;*/

    let subdomains = kong_mount_roscoe(&rectangle, n_subdomains);
    let cmap = plotters::style::colors::colormaps::Bone;
    let n_subdomains = subdomains.len() + 1;
    for (n, subdomain) in subdomains.into_iter().enumerate() {
        let color_style = cmap
            .get_color((n + 1) as f64 / n_subdomains as f64)
            .filled();
        let style_boxed = plotters::style::ShapeStyle {
            color: plotters::style::colors::BLACK.into(),
            filled: false,
            stroke_width: 2,
        };
        let x0 = ((subdomain.min[0] - min[0]) / (max[0] - min[0]) * nx as f64).round() as i32;
        let y0 = ((subdomain.min[1] - min[1]) / (max[1] - min[1]) * ny as f64).round() as i32;
        let x1 = ((subdomain.max[0] - min[0]) / (max[0] - min[0]) * nx as f64).round() as i32;
        let y1 = ((subdomain.max[1] - min[1]) / (max[1] - min[1]) * ny as f64).round() as i32;
        let rect1 = plotters::prelude::Rectangle::new([(x0, y0), (x1, y1)], color_style);
        let rect2 = plotters::prelude::Rectangle::new([(x0, y0), (x1, y1)], style_boxed);
        root.draw(&rect1)?;
        root.draw(&rect2)?;
    }

    root.present()?;
    println!("Result has been saved to {filename}");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    macro_rules! plot_rect(
        ($name:literal $max1:literal $max2:literal $n_subdomains:literal) => {
            plot_decomposed_rectangle(
                Rectangle {
                    min: [0.; 2],
                    max: [$max1, $max2],
                },
                ($n_subdomains).try_into().unwrap(),
                concat!("plots/", $name, $n_subdomains, ".png"),
            )?;
        }
    );
    plot_rect! {"square" 100. 100. 2}
    plot_rect! {"square" 100. 100. 3}
    plot_rect! {"square" 100. 100. 4}
    plot_rect! {"square" 100. 100. 5}
    plot_rect! {"square" 100. 100. 6}
    plot_rect! {"square" 100. 100. 7}
    plot_rect! {"square" 100. 100. 8}
    plot_rect! {"square" 100. 100. 9}
    plot_rect! {"square" 100. 100. 10}
    plot_rect! {"square" 100. 100. 11}
    plot_rect! {"square" 100. 100. 12}
    plot_rect! {"square" 100. 100. 13}

    plot_rect! {"wide_rect" 100. 20. 2}
    plot_rect! {"wide_rect" 100. 20. 3}
    plot_rect! {"wide_rect" 100. 20. 4}
    plot_rect! {"wide_rect" 100. 20. 5}
    plot_rect! {"wide_rect" 100. 20. 6}
    plot_rect! {"wide_rect" 100. 20. 7}

    plot_rect! {"other_rect" 100. 70. 2}
    plot_rect! {"other_rect" 100. 70. 3}
    plot_rect! {"other_rect" 100. 70. 4}
    plot_rect! {"other_rect" 100. 70. 5}
    plot_rect! {"other_rect" 100. 70. 6}
    plot_rect! {"other_rect" 100. 70. 7}

    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}
