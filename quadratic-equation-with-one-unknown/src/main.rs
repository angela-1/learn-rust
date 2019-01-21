use std::f32;
use std::io;

#[derive(Debug)]
struct Coefficient {
    a: i32,
    b: i32,
    c: i32,
}

fn build_coefficient() -> Coefficient {
    println!("请输入系数 a b c，空格分隔：");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("解析输入失败。");

    println!("");

    let mut iter = guess.split_whitespace();

    let a: String = iter.next().expect("无法解析系数 a。").to_string();
    let b: String = iter.next().expect("无法解析系数 b。").to_string();
    let c: String = iter.next().expect("无法解析系数 c。").to_string();
    let coef = Coefficient {
        a: a.parse().expect("解析系数 a 成整数失败。"),
        b: b.parse().expect("解析系数 b 成整数失败。"),
        c: c.parse().expect("解析系数 c 成整数失败。"),
    };
    coef
}

fn calc(coef: Coefficient) -> Result<(f32, f32), String> {
    let delta: f32 = (coef.b * coef.b - 4 * coef.a * coef.c) as f32;
    if delta < 0_f32 {
        return Err("方程没有实数根。".to_string());
    }

    let f: f32 = delta.sqrt();
    let x1: f32 = (-coef.b as f32 + f) / ((2 * coef.a) as f32);
    let x2: f32 = (-coef.b as f32 - f) / ((2 * coef.a) as f32);
    Ok((x1, x2))
}

fn build_equation(coef: &Coefficient) -> String {
    let mut operator1 = "+";
    let mut operator2 = "+";

    match coef.b < 0 {
        true => operator1 = "-",
        false => (),
    }

    match coef.c < 0 {
        true => operator2 = "-",
        false => (),
    }

    let s = format!(
        "{a}x² {operator1} {b}x {operator2} {c} = 0",
        a = coef.a,
        b = coef.b.abs(),
        c = coef.c.abs(),
        operator1 = operator1,
        operator2 = operator2
    );

    s
}

fn main() {
    println!("求解形如 ax² + bx + c = 0 一元二次方程的两个实数根：");

    let coef: Coefficient = build_coefficient();
    let equation = build_equation(&coef);
    println!("一元二次方程 {} 的两个实数根为：", equation);

    let (x1, x2) = calc(coef).expect("解方程失败。");
    println!("x1 = {:?}, x2 = {:?}\n", x1, x2);
}
