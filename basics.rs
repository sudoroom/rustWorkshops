fn main() {
    println!("Hello world!");
    let answer = 42;
    assert_eq!(answer, 42);
    // range is not inclusive
    for i in 0..5 {
        println!("hello {}", i);
        if i %2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }

        for num in 0..10 {
            // in rust nearly everything has a value and can be an expression
            let even_odd = if num % 2 == 0 { "even" } else { "odd" };
            println!("{} {}", even_odd, num);
        }

        // example of rust mutable
        // note that rust is strongly typed unlike javascript or python
        let mut sum = 0.0;
        for i in 0..5 {
            // rust is explicit - it will not silently conver the integer into a float for you
            // cast as a floating point value
            sum += i as f64;
        }
        println!("sum is {}", sum);

    }

    let result = sqr(2);
    println!("Square is {}", res);
}

// explicit function types
fun sqr(x: f64) -> f64 {
    return x * x
}