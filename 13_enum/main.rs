enum Shape{
Circle,
Square,
}
fn print_shape_name(shape:Shape){
    match shape{
    Shape::Circle=>println!("This is a circle"),
    Shape::Square=>println!("This is a square"),
};
}
fn main(){
    let c = Shape::Circle;
    let s= Shape::Square;

    print_shape_name(c);
}