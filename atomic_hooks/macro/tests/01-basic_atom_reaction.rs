use atomic_hooks::*;

#[derive(Clone)]
struct Pos(f64, f64);

#[atom]
fn a_pos() -> Atom<Pos> {
    Pos(0., 0.)
}

#[atom]
fn b_pos() -> Atom<Pos> {
    Pos(0., 0.)
}

#[reaction]
fn a_b_distance() -> Reaction<f64> {
    let a = a_pos().observe();
    let b = b_pos().observe();
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn main() {
    let a_pos = a_pos();
    let b_pos = b_pos();
    let a_b_distance = a_b_distance();

    println!("A is at : {}", a_pos.get());
    println!("B is at : {}", b_pos.get());
    println!("The distance between them is : {}", a_b_distance.get());
    assert!((a_b_distance.get() - 0.0).abs() < std::f64::EPSILON);

    a_pos.update(|s| *s = Pos(4., 5.));
    b_pos.update(|s| *s = Pos(1., 1.));
    println!("moving a to {} and b to {}", a_pos.get(), b_pos.get());
    println!("The distance between them is now : {}", a_b_distance.get());

    assert!((a_b_distance.get() - 5.0).abs() < std::f64::EPSILON);
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
