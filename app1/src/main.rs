fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let sum: i32 = numbers.iter().sum();
    //println!("Sum = {}", sum);

    pub struct Car {
        pub color: String,
        pub fast: int
}
    fn increase_speed(car: &mut Car, amount: int) {
        car.fast += amount;
    }

    fn print_car_info(car: &Car) {
        println!("Car color: {}, Speed: {}", car.color, car.fast);
    }

}