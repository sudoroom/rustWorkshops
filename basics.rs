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
    }
}