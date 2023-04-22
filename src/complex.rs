use crate::Fraction;

/// Represents a complex number through two `Fraction`s, one for the real
/// component, and one for the imaginary component. 
#[derive(Clone, Copy, Debug)]
pub struct Complex
{
    real: Fraction,
    imaginary: Fraction
}

impl Complex
{
    /// Creates a complex number with the given fractional argumments for
    /// its real and imaginary components. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let real = Fraction::unchecked_new(1, 2);
    /// let imaginary = Fraction::unchecked_new(3, 4);
    /// 
    /// let complex = Complex::new(real, imaginary);
    /// 
    /// assert_eq!(complex.get_components(), (real, imaginary));
    /// ```
    pub fn new(real: Fraction, imaginary: Fraction) -> Complex
    {
        Complex {real, imaginary}
    }

    pub fn from_i32(value: i32) -> Complex
    {
        Complex::from_i32_pair(value, 0)
    }

    pub fn from_i32_pair(real: i32, imaginary: i32) -> Complex
    {
        Complex {real: Fraction::from_i32(real), imaginary: Fraction::from_i32(imaginary)}
    }

    pub fn get_components(&self) -> (Fraction, Fraction)
    {
        (self.real, self.imaginary)
    }

    pub fn get_real(&self) -> Fraction
    {
        self.real
    }

    pub fn get_imaginary(&self) -> Fraction
    {
        self.imaginary
    }
}

impl std::fmt::Display for Complex
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.imaginary >= Fraction::from_i32(0)
        {
            write!(f, "{} + {}i", self.real, self.imaginary)
        }
        else
        {
            write!(f, "{} - {}i", self.real, self.imaginary.abs())
        }
    }
}

impl std::ops::Add<Complex> for Complex
{
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output
    {
        Complex::new(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}

impl std::ops::Neg for Complex
{
    type Output = Complex;

    fn neg(self) -> Self::Output
    {
        Complex::new(-self.real, -self.imaginary)
    }
}

impl std::ops::Sub<Complex> for Complex
{
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output
    {
        self + (-rhs)
    }
}

impl std::ops::Mul<Complex> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output
    {
        Complex::new
        (
            self.real * rhs.real - self.imaginary * rhs.imaginary,
            self.real * rhs.imaginary + self.imaginary * rhs.real
        )
    }
}

impl Complex
{
    pub fn conjugate(self) -> Complex
    {
        Complex::new(self.real, -self.imaginary)
    }
}

impl std::ops::Div<Complex> for Complex
{
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output
    {
        let conjugate = rhs.conjugate();

        let numerator = self * conjugate;

        // Product of rhs and rhs.conjugate
        let denominator = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;

        Complex::new(numerator.real / denominator, numerator.imaginary / denominator)
    }
}
