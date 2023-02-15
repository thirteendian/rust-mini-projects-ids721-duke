//define a function take three integer input: wt, ht, age, calculate the BMR value using the formula: BMR = 66+(13.7*wt)+(5*ht)-(6.8*age)
//return the BMR value
pub fn harris(wt: f32, ht: f32, age: f32, choice: i32) -> () {
    let bmr_hb = 66.47 + (13.75 * wt) + (5.003 * ht) - (6.755 * age);
    let bmr_msj = 10.0 * wt + 6.25 * ht - 5.0 * age + 5.0;
    let bmr_km = 370.0 + 21.6 * (0.407 * wt + 0.267 * ht - 0.47 * age);
    let bmr_rhb = 88.362 + (13.397 * wt) + (4.799 * ht) - (5.677 * age);
    if choice == 1 {
        println!("1. Harris-Benedict Equation : {}", 1.2 * bmr_hb);
        println!("2. Mifflin-St Jeor Equation : {}", 1.2 * bmr_msj);
        println!("3. Katch-McArdle Equation : {}", 1.2 * bmr_km);
        println!("4. Revised Harris-Benedict Equation : {}", 1.2 * bmr_rhb);
    }
    if choice == 2 {
        println!("1. Harris-Benedict Equation : {}", 1.375 * bmr_hb);
        println!("2. Mifflin-St Jeor Equation : {}", bmr_msj);
        println!("3. Katch-McArdle Equation : {}", bmr_km);
        println!("4. Revised Harris-Benedict Equation : {}", bmr_rhb);
    }
    if choice == 3 {
        println!("1. Harris-Benedict Equation : {}", 1.55 * bmr_hb);
        println!("2. Mifflin-St Jeor Equation : {}", 1.55 * bmr_msj);
        println!("3. Katch-McArdle Equation : {}", 1.55 * bmr_km);
        println!("4. Revised Harris-Benedict Equation : {}", 1.55 * bmr_rhb);
    }
    if choice == 4 {
        println!("1. Harris-Benedict Equation : {}", 1.725 * bmr_hb);
        println!("2. Mifflin-St Jeor Equation : {}", 1.725 * bmr_msj);
        println!("3. Katch-McArdle Equation : {}", 1.725 * bmr_km);
        println!("4. Revised Harris-Benedict Equation : {}", 1.725 * bmr_rhb);
    }
    if choice == 5 {
        println!("1. Harris-Benedict Equation : {}", 1.9 * bmr_hb);
        println!("2. Mifflin-St Jeor Equation : {}", 1.9 * bmr_msj);
        println!("3. Katch-McArdle Equation : {}", 1.9 * bmr_km);
        println!("4. Revised Harris-Benedict Equation : {}", 1.9 * bmr_rhb);
    }
}
