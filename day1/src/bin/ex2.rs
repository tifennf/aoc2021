use std::fs;

fn main() {
    let dept_list = fs::read_to_string("./inputs.txt").unwrap();
    let dept_list = dept_list.lines().map(|dept| dept.parse::<i32>().unwrap());
    let dept_list = dept_list
        .clone()
        .zip(dept_list.clone().skip(1))
        .zip(dept_list.skip(2))
        .map(|((a, b), c)| (a, b, c))
        .collect::<Vec<(i32, i32, i32)>>();

    let increase_count = day1::ex2::tortipouss_version(&dept_list);
    // let increase_count = day1::ex1::torterra_version(dept_list);
    println!("{}", &increase_count);
}
