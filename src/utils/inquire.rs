pub fn ask_input(hints: &str, default: &str) -> String {
    let result = inquire::Text::new(hints).with_default(default).prompt();
    match result {
        Ok(result) => result,
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}

pub fn ask_input_without_hint(hints: &str) -> String {
    let result = inquire::Text::new(hints).prompt();
    match result {
        Ok(result) => result,
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}

pub fn ask_password(hints: &str) -> String {
    let result = inquire::Password::new(hints).without_confirmation().prompt();
    match result {
        Ok(result) => result,
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}

pub fn ask_select(hints: &str, options: Vec<&str>) -> String {
    let result = inquire::Select::new(hints, options.clone()).prompt();
    match result {
        Ok(result) => result.to_string(),
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}

pub fn ask_yes(hints: &str) -> bool {
    let result = inquire::Confirm::new(hints).prompt();
    match result {
        Ok(result) => result,
        Err(err) => {
            panic!("Error: {:?}", err);
        },
    }
}
