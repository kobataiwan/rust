fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
    let mut cells_cp = cells;
    let mut tmp = cells_cp.to_vec();
    let mut _day_remainder = if n % 14 != 0 {
        n % 14 + 1
    } else {
        15
    };

    tmp[0] = 0;
    tmp[7] = 0;

    for _day in 1.._day_remainder {
        for cell in  0..8 {
            if cell == 0 || cell == 7 {
                tmp[cell] = 0;
                continue;
            }
            if (cells_cp[cell - 1] == 1 && cells_cp[cell + 1] == 1) || 
                (cells_cp[cell - 1] == 0 && cells_cp[cell + 1] == 0) {
                tmp[cell] = 1; 
            } else {
                tmp[cell] = 0; 
            }
        }
        cells_cp = tmp.to_vec();
        println!("day {:? }, {:?}", _day, cells_cp);
    }
    cells_cp
}

pub fn run () {
    let num = vec![0,1,0,1,1,0,0,1];
    let days = 7;
    println!("{:?}", prison_after_n_days(num, days));
    let num_1 = vec![1,0,0,1,0,0,1,0];
    let days_1 = 1000000000;
    println!("{:?}", prison_after_n_days(num_1, days_1));
    let num_2 = vec![1,0,0,1,0,0,0,1];
    let days_2 = 826;
    println!("{:?}", prison_after_n_days(num_2, days_2));
}