use trust::add;
use trust::div;
use trust::mul;
use trust::sub;

fn main() {
    print!("Hello, world!");
    print!("add(1, 2) = {}", add(1, 2));
    print!("sub(1, 2) = {}", sub(1, 2));

    print!("mul(2, 3) = {}", mul(2, 3));
    print!("div(10, 5) = {}", div(10, 5));
}
