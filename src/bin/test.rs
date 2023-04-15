struct A {
    t: u32,
}

impl A {
    fn f(&self) -> u32 {
        self.t
    }
}

fn ff() {

}
fn main() {
    let a = A{t: 1};
    let c = || {};

    a.f();

    ff();

    c();
}