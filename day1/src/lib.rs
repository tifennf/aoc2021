pub mod ex1 {

    pub fn tortipouss_version(dept_list: impl Iterator<Item = i32>) -> i32 {
        let mut count = 0;
        let mut last_record = 0;

        for dept_record in dept_list {
            if last_record != 0 && (last_record - dept_record) < 0 {
                count += 1;
            }

            last_record = dept_record;
        }

        count
    }

    struct State {
        count: i32,
        last_record: i32,
    }

    pub fn torterra_version(dept_list: impl Iterator<Item = i32>) -> i32 {
        dept_list
            .fold(
                State {
                    count: 0,
                    last_record: 0,
                },
                |mut state, dept_record| {
                    let State { count, last_record } = &mut state;

                    if *last_record != 0 && (*last_record - dept_record) < 0 {
                        *count += 1;
                    }

                    *last_record = dept_record;

                    state
                },
            )
            .count
    }
}

pub mod ex2 {

    pub fn tortipouss_version(dept_list: &Vec<(i32, i32, i32)>) -> i32 {
        let mut count = 0;
        let mut last_sum = 0;

        for (a, b, c) in dept_list.iter() {
            let sum = a + b + c;

            if last_sum != 0 && last_sum < sum {
                count += 1;
            }

            last_sum = sum;
        }

        count
    }

    struct State {
        count: i32,
        last_sum: i32,
    }

    pub fn torterra_version(dept_list: &Vec<(i32, i32, i32)>) -> i32 {
        dept_list
            .iter()
            .fold(
                State {
                    count: 0,
                    last_sum: 0,
                },
                |mut state, (a, b, c)| {
                    let State { count, last_sum } = &mut state;

                    let sum = a + b + c;
                    if *last_sum != 0 && *last_sum < sum {
                        *count += 1;
                    }

                    *last_sum = sum;

                    state
                },
            )
            .count
    }
}
