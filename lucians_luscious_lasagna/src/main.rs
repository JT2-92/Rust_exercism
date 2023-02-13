mod lucians_luscious_lasagna;


fn main() {
    println!("{}", lucians_luscious_lasagna::expected_minutes_in_oven());
    println!("{}", lucians_luscious_lasagna::remaining_minutes_in_oven(30));
    println!("{}", lucians_luscious_lasagna::preparation_time_in_minutes(2));
    println!("{}", lucians_luscious_lasagna::elapsed_time_in_minutes(3, 20));
}