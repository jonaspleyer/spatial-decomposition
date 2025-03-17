use spatial_decomposition::{Rectangle, kmr_decompose, kmr_digitize_1};

use plotters::prelude::*;

pub fn plot_digitized_rectangles(
    rectangle: Rectangle<f64>,
    n_subdomains: core::num::NonZeroUsize,
    n_digits: core::num::NonZeroUsize,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let Rectangle { min, max } = rectangle;
    let ratio = (max[1] - min[1]) / (max[0] - min[0]);
    let nx = 1000;
    let ny = (ratio * nx as f64).round() as u32;
    let root = BitMapBackend::new(filename, (nx, ny)).into_drawing_area();
    root.fill(&WHITE)?;

    let draw_rect =
        |subspace_min: [f64; 2], subspace_max: [f64; 2], style: plotters::style::ShapeStyle| {
            let x0 = ((subspace_min[0] - min[0]) / (max[0] - min[0]) * nx as f64).round() as i32;
            let y0 = ((subspace_min[1] - min[1]) / (max[1] - min[1]) * ny as f64).round() as i32;
            let x1 = ((subspace_max[0] - min[0]) / (max[0] - min[0]) * nx as f64).round() as i32;
            let y1 = ((subspace_max[1] - min[1]) / (max[1] - min[1]) * ny as f64).round() as i32;
            let rect = plotters::prelude::Rectangle::new([(x0, y0), (x1, y1)], style);
            root.draw(&rect)
        };

    let cmap = plotters::style::colors::colormaps::Bone;
    let voxels = kmr_decompose(&rectangle, n_digits);
    let digits = kmr_digitize_1(
        &rectangle,
        n_subdomains,
        voxels.clone().into_iter().enumerate(),
    )?;
    for (n_subdomain, (_, voxels)) in digits.iter().enumerate() {
        for (_, rect) in voxels.iter() {
            let boxed_style = plotters::style::ShapeStyle {
                color: plotters::style::RGBColor(30, 30, 30).into(),
                filled: false,
                stroke_width: 1,
            };
            let color_style = cmap
                .get_color((n_subdomain + 1) as f64 / n_subdomains.get() as f64)
                .filled();
            draw_rect(rect.min, rect.max, color_style)?;
            draw_rect(rect.min, rect.max, boxed_style)?;
        }
    }

    for (subdomain, _) in digits.iter() {
        let boxed_style = plotters::style::ShapeStyle {
            color: plotters::style::colors::BLACK.into(),
            filled: false,
            stroke_width: 3,
        };
        draw_rect(subdomain.min, subdomain.max, boxed_style)?;
    }

    root.present()?;
    println!("Result has been saved to {filename}");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_name = "plots/digitize_kmr_1";
    std::fs::create_dir_all(dir_name)?;
    macro_rules! plot(
        ($name:literal $max1:literal $max2:literal $n_digits:literal $n_subdomains:literal) => {
            plot_digitized_rectangles(
                Rectangle {
                    min: [0.; 2],
                    max: [$max1, $max2],
                },
                ($n_subdomains).try_into().unwrap(),
                ($n_digits).try_into().unwrap(),
                &format!("{}/{}{}.png", dir_name, $name, $n_subdomains),
            )?;
        }
    );

    plot! {"square_3x3_in_" 100. 100. 9  2}
    plot! {"square_3x3_in_" 100. 100. 9  3}
    plot! {"square_4x4_in_" 100. 100. 16 3}
    plot! {"square_4x4_in_" 100. 100. 16 4}
    plot! {"square_9x9_in_" 100. 100. 81 3}
    plot! {"square_9x9_in_" 100. 100. 81 4}
    plot! {"square_9x9_in_" 100. 100. 81 5}
    plot! {"square_9x9_in_" 100. 100. 81 6}
    plot! {"square_20x20_in_" 100. 100. 400 11}

    plot! {"rectangle_1_in_" 200. 100. 21 4}
    plot! {"rectangle_2_in_" 200. 100. 23 4}
    plot! {"rectangle_3_in_" 200. 100. 25 4}

    plot! {"rectangle_4_in_" 300. 200. 31 4}
    plot! {"rectangle_5_in_" 300. 200. 31 5}
    plot! {"rectangle_6_in_" 300. 200. 31 6}

    Ok(())
}
