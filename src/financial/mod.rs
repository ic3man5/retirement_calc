use crate::MainWindow;

#[derive(Debug)]
pub enum Period {
    Annually = 1,
    Quarterly = 4,
    Monthly = 12,
    BiWeekly = 26,
    Weekly = 52,
    Daily = 365,
}

#[derive(Debug, Clone)]
pub struct Info {
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

impl Info {
    pub fn from_ui(ui: &MainWindow) -> Self {
        Self {
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
        }
    }
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