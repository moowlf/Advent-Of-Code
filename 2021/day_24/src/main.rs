fn solve() {
    let divider_z = vec![1, 1, 1, 26, 1, 26, 1, 26, 1, 1, 26, 26, 26, 26];
    let padding_x = vec![10, 13, 15, -12, 14, -2, 13, -12, 15, 11, -3, -13, -12, -13];
    let padding_y = vec![10, 5, 12, 12, 6, 4, 15, 3, 7, 11, 2, 12, 4, 11];

    //println!("{}", solve_1(&divider_z, &padding_x, &padding_y));
    solve_1(&divider_z, &padding_x, &padding_y, 0, 0, 0);
    solve_2(&divider_z, &padding_x, &padding_y, 0, 0, 0);
}

fn solve_1(
    div_z: &Vec<i64>,
    pad_x: &Vec<i64>,
    pad_y: &Vec<i64>,
    curr_z: i64,
    curr_index: u64,
    curr_number: u64,
) -> bool {
    if curr_index == 14 && curr_z == 0 {
        println!(
            "Index: {} \t Number: {} \t Current Z: {}",
            curr_index, curr_number, curr_z
        );
        return true;
    }

    if div_z[curr_index as usize] == 1 {
        for val in (1..10).rev() {
            let new_z = (curr_z * 26) + val + pad_y[curr_index as usize];
            let new_num = curr_number * 10 + val as u64;
            let res = solve_1(div_z, pad_x, pad_y, new_z, curr_index + 1, new_num);

            if res {
                return true;
            }
        }
    } else {
        // div_z[curr_index as usize] == 26

        let possible_w = (curr_z % 26) + pad_x[curr_index as usize];
        if possible_w > 0 && possible_w < 10 {
            let new_z = curr_z / 26;
            let new_num = curr_number * 10 + possible_w as u64;
            let res = solve_1(div_z, pad_x, pad_y, new_z, curr_index + 1, new_num);

            if res {
                return true;
            }
        }
    }

    false
}

fn solve_2(
    div_z: &Vec<i64>,
    pad_x: &Vec<i64>,
    pad_y: &Vec<i64>,
    curr_z: i64,
    curr_index: u64,
    curr_number: u64,
) -> bool {
    if curr_index == 14 && curr_z == 0 {
        println!(
            "Index: {} \t Number: {} \t Current Z: {}",
            curr_index, curr_number, curr_z
        );
        return true;
    }

    if div_z[curr_index as usize] == 1 {
        for val in 1..10 {
            let new_z = (curr_z * 26) + val + pad_y[curr_index as usize];
            let new_num = curr_number * 10 + val as u64;
            let res = solve_2(div_z, pad_x, pad_y, new_z, curr_index + 1, new_num);

            if res {
                return true;
            }
        }
    } else {
        // div_z[curr_index as usize] == 26

        let possible_w = (curr_z % 26) + pad_x[curr_index as usize];
        if possible_w > 0 && possible_w < 10 {
            let new_z = curr_z / 26;
            let new_num = curr_number * 10 + possible_w as u64;
            let res = solve_2(div_z, pad_x, pad_y, new_z, curr_index + 1, new_num);

            if res {
                return true;
            }
        }
    }

    false
}

fn main() {
    solve();
}
