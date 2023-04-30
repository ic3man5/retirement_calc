/// Calculates compound interest and returns the accrued amount (Principal + Interest)
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
///     assert_eq!(formulas::compound_interest(100.0, 10.0, 12.0, 10.0), 270.7039_f32);
/// ```
pub fn compound_interest(p: f32, r: f32, n: f32, t: f32) -> f32 {
    // A = P*(1+r/n)^(n*t)
    // where 
    //     P is the principal balance, 
    //     r is the interest rate (as a decimal), 
    //     n is the number of times interest is compounded per year
    //     t is the number of years.
    // If the rate isn't a decimal 0.xx convert it down here
    let mut r = r;
    if r > 1.0 {
        r /= 100.0;
    }
    let r = r;
    let compound_interest = p * (1.0 + r/n).powf(n*t);
    return compound_interest;
}

/// Calculates compound interest with a monthly contribution and returns the 
/// accrued amount (Principal + Interest + Contribution)
/// 
/// Parameters:
///     p = Principal amount in dollars
///     r = Annual nominal interest rate as a decimal
///     n = number of compounding periods per unit of time (t)
///     t = time in decimal years (6 months = 0.5 years)
///     pmt = monthly contribution
/// 
/// Returns:
///     Accrued amount (principal + interest)
/// 
/// Example:
/// ```
///     assert_eq!(formulas::compound_interest(100.0, 10.0, 12.0, 10.0), 270.7039_f32);
/// ```
pub fn compound_interest_with_contrib(p: f32, r: f32, n: f32, t: f32, pmt: f32) -> f32 {
    // https://www.bizskinny.com/Finance/Compound-Interest/compound-interest-with-monthly-contributions.php
    // A = P(1+(r/n))^(nt) + (PMT(1+(r/n))^(nt)-1) / (r/n)
    // If the rate isn't a decimal 0.xx convert it down here
    let mut r = r;
    if r > 1.0 {
        r /= 100.0;
    }
    let r = r;
    let nt = n*t;
    let rn = r/n;
    let left_side = p*(1.0+(rn)).powf(nt)-1.0;
    let right_side =  pmt * ((1.0+(rn)).powf(nt) - 1.0) / (rn);
    let a = left_side + right_side;
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_compound_interest() {
        assert_eq!(compound_interest(100.0, 10.0, 12.0, 10.0), 270.7039_f32);
        assert_eq!(compound_interest(100.0, 0.10, 12.0, 10.0), 270.7039_f32);
    }

    #[test]
    pub fn test_compound_interest_with_contrib() {
        assert_eq!(compound_interest_with_contrib(100.0, 10.0, 12.0, 10.0, 100.0), 20754.168_f32);
        assert_eq!(compound_interest_with_contrib(100.0, 0.10, 12.0, 10.0, 100.0), 20754.168_f32);
    }
}
