//build a function return the dice roll result
pub fn zzconvert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() < num_rows as usize {
        return s;
    }
    let s_len = s.len();
    let mut rows: Vec<String> = vec![String::new(); s_len];
    let mut reverse: bool = false;
    let mut row = 0;
    let last_row: usize = num_rows as usize - 1;

    for i in 0..s_len {
        rows[row] += &s[i..=i]; // range must be used for substring slice

        if reverse {
            row -= 1;
        } else {
            row += 1
        }

        if row == last_row || row == 0 {
            reverse = !reverse;
        }
    }

    rows.into_iter().collect()
}
