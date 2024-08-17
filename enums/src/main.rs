fn main() {
    // For getting types and structures we can use enums and they give us more flexibilities over structs
    #[derive(Debug)]
    enum IpAddr {
        V4,
        V6
    }

    let v4_addr = IpAddr::V4;

    dbg!(v4_addr);

    // using enum with structs
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6
    }

    #[derive(Debug)]
    struct IpAddrr {
        kind: IpAddrKind,
        address: String
    }
    let home = IpAddrr{
        kind:IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    dbg!(home);

    // using enum alone without the struct
    #[derive(Debug)]
    enum IpAdrrr{
        V4(String),
        v6(String)
    }
    let home = IpAdrrr::V4(String::from("127.0.0.1"));
    dbg!(home);

    // another advantage of enums is that we can give it a single variable different types and amounts of associated data
    #[derive(Debug)]
    enum IpAdrrrr{
        V4(u8,u8,u8,u8),
        v6(String)

    }

    let my_home = IpAdrrrr::V4(127,0,1,1);
    dbg!(&my_home);

    // we can also define methods on enums like structs

    impl IpAdrrrr {
        fn call(&self){
            println!("Calling an enum");
        }
    };

    
    my_home.call();

    // match expressions with enums

    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter
    }

    fn value_in_cents(coin:Coin) -> u8{
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    #[derive(Debug)]
    enum UsState {
        Wisconsin,
        Dakota,
        Alabama,
        Alaska
    }

    enum Coinn {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents2(coin:Coinn) -> u8{
        match coin {
            Coinn::Penny => 1,
            Coinn::Nickel => 5,
            Coinn::Dime => 10,
            Coinn::Quarter(state) => {
                println!("State quarter is from {:?}.",state);
                25
            }
        }
    }

    value_in_cents2(Coinn::Quarter(UsState::Wisconsin));

    // match expressions with options

    fn pluse_one(x: Option <i32>) -> Option <i32> {
        match x {
            None => None,
            Some(i) => Some(i+1)
        }
    }

    let five = Some(5);
    let six = pluse_one(five);
    println!("{:?}", six);
    let none = pluse_one(None);
    println!("{:?}", none);

    // handle different cases using match and other and _

    let dice_to_roll = 4;
    match dice_to_roll{
        3 => go_back(),
        7 => go_forward(),
        other => take_step(other),
    }

    fn go_back(){
        println!("Go back!")
    }

    fn go_forward(){
        println!("Go forward")
    }

    fn take_step(steps:u8){
        println!("Take {steps} step.")
    }

    // in this case it does nothing if it doesn't matches the first two scenarios

    let dice_to_roll = 4;
    match dice_to_roll{
        3 => go_back(),
        7 => go_forward(),
        _ => (),
    }

    // matches are exhaustive meaning that they consider all possibilities, this following code

    let my_some = Some(6);
    // match my_some {
    //     Some(x) => {
    //         println!("It is some {x}"); ******** this code results in an error, stating that None is not covered.
    //     }
    // }

    // using if let instead of the exhaustive match
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is {max}"),
        _ => (),
        
    }

    //if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }


    


}
