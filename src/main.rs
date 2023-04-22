use ::fraction::*;

fn main()
{
    let fraction_string = "-4 / 5";

    let a = Fraction::new(10, 3).unwrap();
    let b = fraction_string.parse::<Fraction>().unwrap();

    println!("\"{fraction_string}\" = {b}");
    println!("{a} + {b} = {}", a + b);

    println!();

    let float_value = 144.2;

    let c = Fraction::from_f64(float_value, 0.000000001);

    println!("{float_value} = {c}");
    println!("{c} = {}", c.to_f64());
}
