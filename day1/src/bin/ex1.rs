use std::fs;

// solution lisible pour n'importe qui dans la version tortipouss
fn main() {
    let dept_list = fs::read_to_string("./inputs.txt").unwrap();
    let dept_list = dept_list.lines().map(|dept| dept.parse::<i32>().unwrap());

    // let increase_count = day1::ex1::tortipouss_version(dept_list);
    let increase_count = day1::ex1::torterra_version(dept_list);
    println!("{}", &increase_count);
}
