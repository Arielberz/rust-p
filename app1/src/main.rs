fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let sum: i32 = numbers.iter().sum();
    println!("Sum = {}", sum);

    pub struct Car {
        pub color: String,
        pub fast: i32
}
    fn increase_speed(car: &mut Car, amount: i32) {
        car.fast += amount;
    }

    fn print_car_info(car: &Car) {
        println!("Car color: {}, Speed: {}", car.color, car.fast);
    }
    let mut car = Car {
        color: String::from("Red"),
        fast: 100,
    };
    increase_speed(&mut car, 50);
    print_car_info(&car);

}