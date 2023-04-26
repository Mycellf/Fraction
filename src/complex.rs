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
    /// let real = Fraction::unchecked_from(1, 2);
    /// let imaginary = Fraction::unchecked_from(3, 4);
    /// 
    /// let complex = Complex::from(real, imaginary);
    /// 
    /// assert_eq!(complex.get_components(), (real, imaginary));
    /// ```
    pub fn from(real: Fraction, imaginary: Fraction) -> Complex
    {
        Complex {real, imaginary}
    }

    /// Creates a complex number with the given fraction as its real component, 
    /// and 0 for its imaginary component. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let real = Fraction::unchecked_from(1, 2);
    /// 
    /// let complex = Complex::from_fraction(real);
    /// 
    /// assert_eq!(complex.get_real(), real);
    /// ```
    pub fn from_fraction(value: Fraction) -> Complex
    {
        Complex::from(value, Fraction::from_i32(0))
    }

    /// Creates a complex number with the given integer as its real component, 
    /// and 0 for its imaginary component. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let real = 10;
    /// 
    /// let complex = Complex::from_i32(real);
    /// 
    /// assert_eq!(complex.get_real(), Fraction::from_i32(real));
    /// ```
    pub fn from_i32(value: i32) -> Complex
    {
        Complex::from_i32_pair(value, 0)
    }

    /// Creates a complex number with the given integer argumments for
    /// its real and imaginary components. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let real = 10;
    /// let imaginary = 4;
    /// 
    /// let complex = Complex::from_i32_pair(real, imaginary);
    /// 
    /// assert_eq!(complex.get_components(), (Fraction::from_i32(real), Fraction::from_i32(imaginary)));
    /// ```
    pub fn from_i32_pair(real: i32, imaginary: i32) -> Complex
    {
        Complex {real: Fraction::from_i32(real), imaginary: Fraction::from_i32(imaginary)}
    }

    /// Returns the real and imaginary components of the complex
    /// number in a tuple. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let real = Fraction::unchecked_from(1, 2);
    /// let imaginary = Fraction::unchecked_from(3, 4);
    /// 
    /// let complex = Complex::from(real, imaginary);
    /// 
    /// assert_eq!(complex.get_components(), (real, imaginary));
    /// ```
    pub fn get_components(&self) -> (Fraction, Fraction)
    {
        (self.real, self.imaginary)
    }

    /// Returns the real component of the complex number. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let real = Fraction::unchecked_from(1, 2);
    /// let imaginary = Fraction::unchecked_from(3, 4);
    /// 
    /// let complex = Complex::from(real, imaginary);
    /// 
    /// assert_eq!(complex.get_real(), real);
    /// ```
    pub fn get_real(&self) -> Fraction
    {
        self.real
    }

    /// Returns the imaginary component of the complex number. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let real = Fraction::unchecked_from(1, 2);
    /// let imaginary = Fraction::unchecked_from(3, 4);
    /// 
    /// let complex = Complex::from(real, imaginary);
    /// 
    /// assert_eq!(complex.get_imaginary(), imaginary);
    /// ```
    pub fn get_imaginary(&self) -> Fraction
    {
        self.imaginary
    }
}

impl std::fmt::Display for Complex
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if self.imaginary.get_numerator() == 0
        {
            return  write!(f, "{}", self.real);
        }

        if self.real.get_numerator() == 0
        {
            return write!(f, "{}i", self.imaginary);
        }

        if self.imaginary >= Fraction::from_i32(0)
        {
            return write!(f, "{} + {}i", self.real, self.imaginary);
        }
        else
        {
            return write!(f, "{} - {}i", self.real, self.imaginary.abs());
        }
    }
}

impl PartialEq for Complex
{
    fn eq(&self, other: &Self) -> bool
    {
        self.real == other.real && self.imaginary == other.imaginary
    }
}

impl Eq for Complex {}

impl std::ops::Add<Complex> for Complex
{
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output
    {
        Complex::from(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}

impl std::ops::AddAssign for Complex
{
    fn add_assign(&mut self, rhs: Self)
    {
        *self = *self + rhs;
    }
}

impl std::ops::Neg for Complex
{
    type Output = Complex;

    fn neg(self) -> Self::Output
    {
        Complex::from(-self.real, -self.imaginary)
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

impl std::ops::SubAssign for Complex
{
    fn sub_assign(&mut self, rhs: Self)
    {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<Complex> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output
    {
        Complex::from
        (
            self.real * rhs.real - self.imaginary * rhs.imaginary,
            self.real * rhs.imaginary + self.imaginary * rhs.real
        )
    }
}

impl std::ops::MulAssign for Complex
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs;
    }
}

impl Complex
{
    /// Returns the complex conjugate of the number. 
    /// 
    /// ```
    /// use complex::{Complex, Fraction};
    /// 
    /// let complex = Complex::from_i32_pair(1, 5);
    /// 
    /// assert_eq!(complex.conjugate(), Complex::from_i32_pair(1, -5));
    /// ```
    pub fn conjugate(self) -> Complex
    {
        Complex::from(self.real, -self.imaginary)
    }
}

impl std::ops::Div<Complex> for Complex
{
    type Output = Complex;

    fn div(self, rhs: Complex) -> Self::Output
    {
        let numerator = self * rhs.conjugate();

        // Product of rhs and rhs.conjugate()
        let denominator = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;

        Complex::from(numerator.real / denominator, numerator.imaginary / denominator)
    }
}

impl std::ops::DivAssign for Complex
{
    fn div_assign(&mut self, rhs: Self)
    {
        *self = *self / rhs;
    }
}

impl Complex
{
    /// Returns a complex number representing the real and imaginary signs of this value. 
    /// 
    /// ```
    /// use complex::Complex;
    /// 
    /// let value = Complex::from_i32_pair(10, -2);
    /// 
    /// assert_eq!(value.signum(), (1, -1));
    /// 
    /// assert_eq!(value.signum(), Complex::from_i32_pair(value.get_real().signum(), value.get_imaginary().signum()));
    /// ```
    pub fn signum(self) -> Complex
    {
        Complex::from_i32_pair(self.real.signum(), self.imaginary.signum())
    }

    /// Returns the absolute value of this complex number, squared. 
    pub fn abs_squared(self) -> Fraction
    {
        self.real * self.real + self.imaginary * self.imaginary
    }

    /// Returns the absolute value of this complex number. 
    pub fn abs(self) -> Fraction
    {
        self.abs_squared().sqrt().real
    }
}
