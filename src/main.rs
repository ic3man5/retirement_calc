use slint::Image;

slint::include_modules!();

/// Calculates compound interest and returns the Accrued amount (Principal + Interest)
/// 
/// Parameters:
///     p = Principal amount in dollars
///     r = Annual nominal interest rate as a decimal
///     n = number of compounding periods per unit of time (t)
///     t = time in decimal years (6 months = 0.5 years)
/// 
/// Returns:
///     Accrued amount (principal + interest)
/// 
/// Example:
/// ```
///     todo!();
/// ```
///
pub fn compound_interest(p: f32, r: f32, n: f32, t: f32) -> f32 {
    // A = P*(1+r/n)^(n*t)
    // where 
    //     P is the principal balance, 
    //     r is the interest rate (as a decimal), 
    //     n is the number of times interest is compounded per year
    //     t is the number of years.
    let compound_interest = p * (1.0 + r/n).powf(n*t) - 1.0;
    println!("Compound: {compound_interest}");
    return compound_interest;
}

pub fn compound_interest_with_contrib(p: f32, r: f32, n: f32, t: f32, pmt: f32) -> f32 {
    // https://www.bizskinny.com/Finance/Compound-Interest/compound-interest-with-monthly-contributions.php
    // A = P(1+(r/n))^(nt) + (PMT(1+(r/n))^(nt)-1) / (r/n)
    let nt = n*t;
    let rn = r/n;
    let left_side = p*(1.0+(rn)).powf(nt)-1.0;
    let right_side =  pmt * ((1.0+(rn)).powf(nt) - 1.0) / (rn);
    let a = left_side + right_side;
    return a;
}

#[derive(Debug, Clone)]
struct FinancialData {
    pub age: f32,
    pub age_of_death: f32,
    pub age_of_retire: f32,
    pub age_of_social: f32,
    pub starting_balance: f32,
    pub ss_monthly: f32,
    pub contrib_monthly: f32,
    pub yield_before_retire: f32,
    pub yield_after_retire: f32,
    pub inflation_rate: f32,
    pub annual_income_post: f32,
    pub annual_income_post_decay: f32,
}

#[derive(Debug, Clone)]
pub struct TimeStatistics {
    pub yrs_life_remaining: f32,
    pub yrs_until_retirement: f32,
    pub yrs_until_ss: f32,
    pub yrs_of_retirement: f32,
    pub yrs_of_ss: f32,
}

impl FinancialData {
    pub fn get_time_stats(&self) -> TimeStatistics {
        TimeStatistics {
            yrs_life_remaining: self.age_of_death - self.age,
            yrs_until_retirement: self.age_of_retire - self.age,
            yrs_until_ss: self.age_of_social - self.age,
            yrs_of_retirement: self.age_of_death - self.age_of_retire,
            yrs_of_ss: self.age_of_death - self.age_of_social,
        }
    }
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


fn main() -> Result<(), slint::PlatformError> {
    draw_test();
    let ui: MainWindow = MainWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_calculate(move || {
        let ui = ui_handle.unwrap();
        let data = FinancialData {
            age: ui.get_current_age(),
            age_of_death: ui.get_death_age(),
            age_of_retire: ui.get_retirement_age(),
            age_of_social: ui.get_social_security_age(),
            starting_balance: ui.get_starting_balance(),
            ss_monthly: ui.get_social_security_monthly(),
            contrib_monthly: ui.get_monthly_contribution(),
            yield_before_retire: ui.get_yield_before_retirement(),
            yield_after_retire: ui.get_yield_after_retirement(),
            inflation_rate: ui.get_inflation_rate(),
            annual_income_post: ui.get_annual_income_post(),
            annual_income_post_decay: ui.get_annual_income_post_decay(),
        };
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
