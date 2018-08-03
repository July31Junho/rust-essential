fn main() {

    /**
     *  세미콜론 ; 으로 값을 리턴하느냐 안하느냐 이다.
     *  scope라서 변수는 그 scope에서만 활동을 하게되는데,
     * a+b 에 세미콜론으로 결정하게 된다.
     * 
     */
    println!("Hello, world!");

    let n1 = {
        let a = 2;
        let b = 5;
        a+b
    };
    println!("n1 is : {} ", n1);

    let n2 = {
        let a = 2;
        let b = 5;
        a + b;
    };
    println!("n2 is : {:?} ", n2);

}
