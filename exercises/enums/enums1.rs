// enums1.rs
//
// No hints this time! ;)

// I AM DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit=1,
    Echo=2,
    Move=3,
    ChangeColor=4,

}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
