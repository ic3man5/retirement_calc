use slint::Image;

pub mod formulas;
pub mod financial;

use formulas::{compound_interest_with_contrib};
use financial::{Info};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    draw_test();
    let ui: MainWindow = MainWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_calculate(move || {
        let ui = ui_handle.unwrap();
        let data = Info::from_ui(&ui);
        println!("{data:#?}");
        let stats = data.get_time_stats();
        println!("Time Statistics: {:#?}", stats);     

        let a = compound_interest_with_contrib(
            data.starting_balance, 
            data.yield_before_retire/100.0,
            12.0,
            stats.yrs_until_retirement,
            data.contrib_monthly);

        let report_str = format!("${:.2} over {} year(s) @ {:.1}% with ${:.2} monthly contribution: ${:.2}",
            data.starting_balance,
            stats.yrs_until_retirement,
            data.yield_before_retire,
            data.contrib_monthly,
            a);
        ui.set_temp_result(report_str.into());

        let ss_total = data.ss_monthly*12.0*stats.yrs_of_ss;
        let ss_str = format!("Social Security: ${:.2} monthly over {} year(s): ${:.2}",
            data.ss_monthly,
            stats.yrs_of_ss,
            ss_total);
        ui.set_temp2_result(ss_str.into());

        let total_str = format!("Total with SS: ${:.2}",
            a+ss_total);
        ui.set_temp3_result(total_str.into());

        /*
        let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(320, 200);

        low_level_render(pixel_buffer.width(), pixel_buffer.height(),
                        pixel_buffer.make_mut_bytes());

        let image = Image::from_rgb8(pixel_buffer);
         */
        let image_path = std::path::Path::new("ui/images/test.png");
        ui.set_line_graph_source(Image::load_from_path(&image_path).unwrap());
    });
    ui.run()
}


use plotters::prelude::*;
fn draw_test() {
    // https://slint-ui.com/releases/1.0.0/docs/rust/slint/struct.image
    // https://github.com/slint-ui/slint/discussions/2527
    // https://plotters-rs.github.io/book/basic/chart_context.html
    let root_area = BitMapBackend::new("ui/images/test.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Legend", ("sans-serif", 40))
        .build_cartesian_2d(-4.0..4.0, -1.2..1.2)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let x_kps: Vec<_> = (-80..80).map(|x| x as f64 / 20.0).collect();
    ctx.draw_series(LineSeries::new(x_kps.iter().map(|x| (*x, x.sin())), &RED))
        .unwrap()
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(x_kps.iter().map(|x| (*x, x.cos())), &BLUE))
        .unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}
