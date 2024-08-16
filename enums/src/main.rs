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


    


}
