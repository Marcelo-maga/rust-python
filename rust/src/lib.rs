use std::time::{Instant, Duration};

#[no_mangle]
pub extern fn fibo_rust() {
    let start_time = Instant::now();
    
    let mut fibo = 2;
    let mut count = 0;

    loop {
        fibo = fibo+count;
        print!("{}; ", fibo);
        count+=1;

        if count == 10 {
            break;
        }
    }

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("O Loop teve duração de {} segundos", duration.as_secs())
}

#[no_mangle]
pub extern fn teste_msg() {
    println!("lagouuu!");
}

