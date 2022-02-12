pub fn eval(expr: &str) -> f64 {
    fn parse(num: &str) -> f64 {
        return match num.parse::<f64>() {
            Ok(n) => n,
            Err(_) => 0.0,
        };
    }
    fn step_4_split_div(expr: &str) -> f64 {
        let mut div: f64 = 1.0;
        let mut is_div_set = false;
        for i in expr.split("/").collect::<Vec<&str>>() {
            if i != "" {
                if !is_div_set {
                    div = parse(i);
                    is_div_set = true;
                } else {
                    div = div / parse(i);
                }
            }
        }
        return div;
    }

    fn step_3_split_mul(expr: &str) -> f64 {
        let mut pro: f64 = 1.0;
        for i in expr.split("*").collect::<Vec<&str>>() {
            if i != "" {
                pro *= step_4_split_div(i);
            }
        }
        return pro;
    }

    fn step_2_split_sub(expr: &str) -> f64 {
        let mut negative = false;
        if expr.chars().nth(0) == Some('-') {
            negative = true;
        }
        let mut is_sub_set = false;
        let mut sub: f64 = 0.0;
        for i in expr.split("-").collect::<Vec<&str>>() {
            if i != "" {
                if !is_sub_set {
                    if negative {
                        sub = -1.0 * step_3_split_mul(i);
                    } else {
                        sub = step_3_split_mul(i);
                    }
                    is_sub_set = true;
                } else {
                    sub -= step_3_split_mul(i);
                }
            }
        }
        return sub;
    }

    fn step_1_split_add(expr: &str) -> f64 {
        let mut sum: f64 = 0.0;
        for i in expr.split("+").collect::<Vec<&str>>() {
            if i != "" {
                sum += step_2_split_sub(i);
            }
        }
        println!("ADD: {}", sum);
        return sum;
    }

    return step_1_split_add(expr);
}
