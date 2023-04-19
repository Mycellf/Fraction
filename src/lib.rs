pub mod lib
{
    #[derive(Clone, Copy)]
    pub struct Fraction
    {
        numerator: i32,
        denominator: u32,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct DivByZeroError;

    impl Fraction
    {
        /// Creates a fraction that is fully simplified. 
        /// Will return `DivByZeroError` if denominator is 0. 
        pub fn new(numerator: i32, denominator: u32) -> Result<Fraction, DivByZeroError>
        {
            let fraction = Fraction::unsimplified_new(numerator, denominator)?;

            Ok(fraction.simplify())
        }
        
        /// Creates a fraction that has no fractional simplification applied to it. 
        /// Will return `DivByZeroError` if denominator is 0. 
        pub fn unsimplified_new(numerator: i32, denominator: u32) -> Result<Fraction, DivByZeroError>
        {
            if denominator == 0
            {
                return Err(DivByZeroError);
            }

            Ok(Fraction {numerator, denominator})
        }

        /// Creates a fraction with no checks on the input. 
        /// Can cause arithmatic issues if the denominator is 0. 
        pub fn unchecked_new(numerator: i32, denominator: u32) -> Fraction
        {
            Fraction {numerator, denominator}
        }

        /// Note that fractions created with `Fraction::new` will be simplified on creation. 
        pub fn simplify(&self) -> Fraction
        {
            let gcd = gcd(self.numerator.abs() as u32, self.denominator);

            let numerator = self.numerator / gcd as i32;
            let denominator = self.denominator / gcd;
            
            Fraction {numerator, denominator}
        }
        
        /// Creates a fraction with `value` as the numerator and 1 as the denominator. 
        pub fn from_i32(value: i32) -> Fraction
        {
            Fraction::unchecked_new(value, 1)
        }
        
        pub fn get_components(&self) -> (i32, u32)
        {
            (self.numerator, self.denominator)
        }

        pub fn get_numerator(&self) -> i32
        {
            self.numerator
        }

        pub fn get_denominator(&self) -> u32
        {
            self.denominator
        }

        pub fn to_f64(&self) -> f64
        {
            (self.numerator as f64) / (self.denominator as f64)
        }

        pub fn from_f64(value: f64, error: f64) -> Fraction
        {
            let integer_part = value.floor();
            let decimal_part = value - integer_part;

            if decimal_part < error
            {
                return Fraction::from_i32(integer_part as i32);
            }
            else if decimal_part > 1.0 - error
            {
                return Fraction::from_i32(integer_part as i32 + 1);
            }

            let mut lower = Fraction::from_i32(0);
            let mut upper = Fraction::from_i32(1);

            loop
            {
                let middle = Fraction::unchecked_new
                (
                    lower.numerator + upper.numerator,
                    lower.denominator + upper.denominator
                );

                if (middle.numerator as f64) > middle.denominator as f64 * (decimal_part + error)
                {
                    upper = middle;
                }
                else if (middle.numerator as f64) < middle.denominator as f64 * (decimal_part - error)
                {
                    lower = middle;
                }
                else
                {
                    return Fraction::new
                    (
                        integer_part as i32 * middle.denominator as i32 + middle.numerator,
                        middle.denominator
                    )
                    .expect("Denominator will not be 0");
                }
            }
        }
    }

    impl std::fmt::Display for Fraction
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            if self.denominator != 1
            {
                write!(f, "{}/{}", self.numerator, self.denominator)
            }
            else
            {
                write!(f, "{}", self.numerator)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ParseFractionError;

    impl std::str::FromStr for Fraction
    {
        type Err = ParseFractionError;

        fn from_str(s: &str) -> Result<Self, Self::Err>
        {
            let (numerator_str, denominator_str) = s
                .split_once('/')
                .ok_or(ParseFractionError)?;

            let numerator = numerator_str.trim().parse::<i32>().map_err(|_| ParseFractionError)?;
            let denominator = denominator_str.trim().parse::<u32>().map_err(|_| ParseFractionError)?;

            Fraction::new(numerator, denominator).map_err(|_| ParseFractionError)
        }
    }

    impl PartialEq for Fraction
    {
        fn eq(&self, other: &Self) -> bool
        {
            self.cmp(other) == std::cmp::Ordering::Equal
        }
    }

    impl Eq for Fraction {}

    impl PartialOrd for Fraction
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
        {
            Some(self.cmp(other))
        }
    }

    impl Ord for Fraction
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering
        {
            (self.numerator * other.denominator as i32).cmp(&(other.numerator * self.denominator as i32))
        }
    }

    impl std::ops::Add<Fraction> for Fraction
    {
        type Output = Fraction;

        fn add(self, rhs: Fraction) -> Self::Output
        {
            let denominator_gcd = gcd(self.denominator, rhs.denominator);

            let mut numerator = self.numerator * (rhs.denominator / denominator_gcd) as i32;
            numerator += rhs.numerator * (self.denominator / denominator_gcd) as i32;

            let denominator = self.denominator * rhs.denominator / denominator_gcd;

            Fraction::new(numerator, denominator).expect("Fraction should not have 0 for denominator")
        }
    }

    impl std::ops::AddAssign for Fraction
    {
        fn add_assign(&mut self, rhs: Self)
        {
            *self = *self + rhs;
        }
    }

    impl std::ops::Neg for Fraction
    {
        type Output = Fraction;

        fn neg(self) -> Self::Output
        {
            Fraction::unchecked_new(-self.numerator, self.denominator)
        }
    }

    impl std::ops::Sub<Fraction> for Fraction
    {
        type Output = Fraction;

        fn sub(self, rhs: Fraction) -> Self::Output
        {
            self + (-rhs)
        }
    }

    impl std::ops::SubAssign for Fraction
    {
        fn sub_assign(&mut self, rhs: Self)
        {
            *self = *self - rhs;
        }
    }

    impl std::ops::Mul<Fraction> for Fraction
    {
        type Output = Fraction;

        fn mul(self, rhs: Fraction) -> Self::Output
        {
            Fraction::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator)
                .expect("Fraction should not have 0 for denominator")
        }
    }

    impl std::ops::MulAssign for Fraction
    {
        fn mul_assign(&mut self, rhs: Self)
        {
            *self = *self * rhs;
        }
    }

    impl Fraction
    {
        pub fn signum(&self) -> i32
        {
            self.numerator.signum()
        }
    }

    impl Fraction
    {
        pub fn reciprocal(self) -> Result<Fraction, DivByZeroError>
        {
            Fraction::unsimplified_new(self.denominator as i32 * self.numerator.signum(), self.numerator.abs() as u32)
        }
    }

    impl std::ops::Div<Fraction> for Fraction
    {
        type Output = Fraction;

        fn div(self, rhs: Fraction) -> Self::Output
        {
            self * rhs.reciprocal().expect("Divide by 0")
        }
    }

    impl std::ops::DivAssign for Fraction
    {
        fn div_assign(&mut self, rhs: Self)
        {
            *self = *self / rhs;
        }
    }

    fn gcd(a: u32, b: u32) -> u32
    {
        let (mut small, mut large) = get_ordering(a, b);

        let mut i = 1;
        let mut result = 1;

        while i <= small
        {
            if small % i == 0 && large % i == 0
            {
                small /= i;
                large /= i;
                result *= i;
                i = 1;
            }

            i += 1;
        }

        result
    }

    /// returns a pair with the smallest value first
    fn get_ordering<T: PartialOrd>(a: T, b: T) -> (T, T)
    {
        if a < b
        {
            (a, b)
        }
        else
        {
            (b, a)
        }
    }
}
