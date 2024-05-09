fn main() {
    println!("Hello, world!");

    //Usamos {} para sustituir por variables.
    println!("Hola {}, hoy es {}", "Cris", "viernes");
    secondfn();
}

fn secondfn() {
    let mut a_number;
    let a_word: &str = "Ten";

    a_number = 10;

    println!("The number is {}", a_number);
    println!("The word is {}", a_word);

    a_number = 3;

    println!("The number changed is {}", a_number);

    shadowing_test();

}

fn shadowing_test() {
    let shadow_num: i32 = 5;

    let shadow_num: i32 = shadow_num + 5;

    let shadow_num: i32 = shadow_num * 2;

    {
        let shadow_num = 1;
    }

    println!("The value of shadow_num is: {}", shadow_num);

    using_text_vars();

}

fn using_text_vars() {

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
    
    tupla_test();

}

fn tupla_test() {

    let tuple = ("first element", 2, true, 4.5);

    println!("Primer elemento: {}", tuple.0);

    structs_test();
}

fn structs_test() {

    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    // Unit struct
    struct Unit;

    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1: Grades = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2: Grades = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);

    using_enum();
    
}


fn using_enum() {

    // enum WebEvent {
    //     // An enum variant can be like a unit struct without fields or data types
    //     WELoad,
    //     // An enum variant can be like a tuple struct with data types but no named fields
    //     WEKeys(String, char),
    //     // An enum variant can be like a classic struct with named fields and their data types
    //     WEClick { x: i64, y: i64 }
    // }

    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick { x: i64, y: i64 }

    let click = MouseClick { x: 10, y: 20 };
    let key = KeyPress(String::from("Control+"),'N');
    
    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    #[derive(Debug)]
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
    
    let we_click = WebEvent::WEClick((click));
    let we_load = WebEvent::WELoad(true);
    let we_keys = WebEvent::WEKeys(key);

    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_keys);

}
