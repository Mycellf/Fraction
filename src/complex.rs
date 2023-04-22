use crate::Fraction;

#[derive(Clone, Copy, Debug)]
pub struct Complex
{
    real: Fraction,
    imaginary: Fraction
}

impl Complex
{
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
