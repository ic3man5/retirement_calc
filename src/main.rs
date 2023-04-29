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
    // A = P(1 + r/n)nt
    // A = Accrued amount (principal + interest)
    // P = Principal amount
    // r = Annual nominal interest rate as a decimal
    // R = Annual nominal interest rate as a percent
    // r = R/100
    // n = number of compounding periods per unit of time
    // t = time in decimal years; e.g., 6 months is calculated as 0.5 years. Divide your partial year number of months by 12 to get the decimal years.
    p*(1.0+r/n)*n*t
    //principal * (1.0 + (rate/100.0 / period)) * period * years
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


fn main() -> Result<(), slint::PlatformError> {
    let ui: MainWindow = MainWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_calculate(move || {
        println!("Hello World!");
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
        let accurred_amount = compound_interest(data.starting_balance, data.yield_before_retire/100.0, 12.0, stats.yrs_of_retirement);
        println!("{accurred_amount}");
        
    });
    ui.run()
}
