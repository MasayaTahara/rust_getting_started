fn main() {
    let rect = Rectange {
        width: 10.0,
        height: 20.0,
    };
    let tri = Triangle {
        base: 10.0,
        height: 20.0,
    };
    let cir = Circle {
        radius: 10.0,
    };
    println!("rect area is {}{}", rect.expr_str(), rect.calc_area());
    println!("tri area is {}{}", tri.expr_str(), tri.calc_area());
    println!("cir area is {}{}", cir.expr_str(), cir.calc_area());
}

struct Rectange {
    width: f32,
    height: f32,
}

struct Triangle {
    base: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

trait CalcArea {
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectange {
    fn calc_area(&self) -> f32 {
        self.width * self.height
    }
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f32 {
        self.base * self.height * 0.5
    }
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
}

trait ExprString {
    fn expr_str(&self) -> String {
        "Width * Height = ".to_string()
    }
}

impl ExprString for Rectange {}
impl ExprString for Triangle {
    fn expr_str(&self) -> String {
        "Base * Height / 2 = ".to_string()
    }
}
impl ExprString for Circle {
    fn expr_str(&self) -> String {
        "Radius ^ 2 * Pi = ".to_string()
    }
}
