use documentaion::kinds::PrimaryColor;
use documentaion::utils::mix;


fn main() {
    let first_color = PrimaryColor::Red;
    let second_color = PrimaryColor::Yellow;
    let result = mix(first_color, second_color);
    println!("The final color is {:?}", result);
}
