fn main() {
    println!("Hello, world!");
    another_function(5, 32);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let a = five();

    println!("The value of a is: {}", a);
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 구문은 명령어들의 나열로 값을 반환하지 않는 동작 수행
// 표현식은 결과 값을 산출
// 표현식은 종결을 나타내는 ;를 사용하지 않음