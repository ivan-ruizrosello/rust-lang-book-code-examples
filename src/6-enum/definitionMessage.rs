/** Struct version */
struct QuitMessage;
struct MoveMessage { 
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);


/** Enum version */
enum Message { 
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message { 
    fn call(&self) { 

    }
}

fn main() { 
    let m = Message::Write(String::from("Hello"));

    m.call();

    let some_number = Some(5);
    let some_string = Some("A string");

    let absent_number: Option<i32> = None;
}