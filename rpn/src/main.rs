fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

pub fn rpn(imput: &str) {
    let values: Vec<_> = imput.split_whitespace().collect();
    let mut res: Vec<_> = vec![];
    let oprs = "+-*/%";
    for value in values {
        let mut nb1 = 0;
        let mut nb2 = 0;
        let mut popet = false;
        if oprs.contains(value) && res.len() < 2 {
            println!("Eroor");
            return;
        }
        if res.len() > 1 {
            popet = true;
            nb1 = res.pop().unwrap();
            nb2 = res.pop().unwrap();
        }
        match value {
            "+" => {
                res.push(nb2 + nb1);
            }
            "-" => {
                res.push(nb2 - nb1);
            }
            "*" => {
                res.push(nb2 * nb1);
            }
            "/" => {
                res.push(nb2 / nb1);
            }
            "%" => {
                res.push(nb2 % nb1);
            }
            _ => {
                if popet {
                    res.push(nb2);
                    res.push(nb1);
                }
                match value.parse::<i32>() {
                    Err(_) => {
                        println!("Eroor");
                        return;
                    }
                    Ok(nb) => {
                        res.push(nb);
                    }
                }
            }
        }
    }
    if res.len() > 1 {
        println!("Eroor");
        return;
    }
    println!("{:?}", res[0])
}
