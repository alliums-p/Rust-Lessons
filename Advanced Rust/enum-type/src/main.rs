enum LampState {
    Red,
    Yellow,
    Green
}

fn print_lamp_state(lamp_value: LampState) -> () {
    match lamp_value {
        LampState::Red => println!("Light is Red!"),
        LampState::Yellow => println!("Light is Yellow!"),
        LampState::Green => println!("Light is Green!")
    }
}

fn print_optional_lamp_state(optional_lamp_value: Option<LampState>) -> () {
    match optional_lamp_value {
        Some(lamp_state) => print_lamp_state(lamp_state),
        None => println!("Lamp has been ruined!")
    }
}

fn main() {

    let lamp_value1: LampState = LampState::Red;
    let lamp_value2: LampState = LampState::Yellow;
    let lamp_value3: LampState = LampState::Green;

    let optional_lamp_value: Option<LampState> = None;
    let optional_lamp_value2: Option<LampState> = Some(LampState::Red);

    print_lamp_state(lamp_value1);
    print_lamp_state(lamp_value2);
    print_lamp_state(lamp_value3);

    print_optional_lamp_state(optional_lamp_value);
    print_optional_lamp_state(optional_lamp_value2);
    
}
