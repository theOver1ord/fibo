fn main() {
    let mut fib_n_minus1 = 0;
    let mut fib_n_minus2 = 1;
    let mut fib_n = 0;
    let numb = 20;

    for i in 0..numb {
        fib_n = fib_n_minus1 + fib_n_minus2;
        fib_n_minus2 = fib_n_minus1;
        fib_n_minus1 = fib_n;

    }

    println!("{}", fib_n);

}
