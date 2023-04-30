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
    let accurred = compound_interest(p, r, n, t);
    // A = P(1+(r/n))^(nt) + (PMT(1+(r/n))^(nt)-1) / (r/n)
    let accurred = (p*(1.0+(r/n)).powf(n*t)) +
        ((pmt*(1.0+(r/n)).powf(n*t)-1.0) / (r/n));
    println!("{accurred}");
    accurred
}

pub fn compound_interest_with_contribution(p: f32, r: f32, n: f32, t: f32, c: f32) -> f32 {
    let nt = n *t;
    let a = p* (1.0 + r/n).powf(nt) + c * ((1.0 + r/n).powf(nt) - 1.0) * (r/n);
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
        let accurred_amount = compound_interest(data.starting_balance, data.yield_before_retire/100.0, 12.0, stats.yrs_until_retirement);
        println!("{accurred_amount}");
        
        
        let accurred_amount_w_contrib = compound_interest_with_contribution(
            data.starting_balance, 
            data.yield_before_retire/100.0,
            12.0,
            stats.yrs_until_retirement,
            data.contrib_monthly);
        println!("With contrib ${accurred_amount}");
        let sb = data.starting_balance;
        let yrs = stats.yrs_until_retirement;
        ui.set_temp_result(format!("${sb} over {yrs} years:\nWith Contribution: ${accurred_amount_w_contrib}").into());
        
        
    });
    ui.run()
}
