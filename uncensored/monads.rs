#[derive(Debug)]
struct NumWithLogs {
    result: i64,
    logs: Vec<String>
}

impl NumWithLogs {
    fn new(x: i64) -> NumWithLogs {
        NumWithLogs {
            result: x,
            logs: Vec::new()
        }
    }
}

fn run(input: NumWithLogs, transform: &dyn Fn(i64) -> NumWithLogs) -> NumWithLogs {
    let newNWL = transform(input.result);
    NumWithLogs {
        result: newNWL.result,
        logs: input.logs.append(&mut newNWL.logs)
    }
}

// Monads implemented above //
// Code written below is meant for testing //

fn square(x: i64) -> NumWithLogs {
    NumWithLogs {
        result: x * x,
        logs: vec![format!("Squared {} to get {}", x, x*x)]
    }
}

fn addOne(x: i64) -> NumWithLogs {
    NumWithLogs {
        result: x + 1,
        logs: vec![format!("Added 1 to {} to get {}", x, x+1)]
    }
}

fn main() {
    let mut a = NumWithLogs::new(4);    
    println!("{:?}", a);
    let b = run(a, &square);
    println!("{:?}", b);
    let c = run(b, &addOne);
    println!("{:?}", c);
}