use std::io;

fn main() {
    let mut temperature_type = String::new();

    read_line_temperature_type(&mut temperature_type);

    println!("please input temperature value");

    let mut temperature_value = String::new();
    
    io::stdin().read_line(&mut temperature_value)
               .expect("failed to read line");
    
    let temperature_value : f32 = temperature_value.trim().parse().unwrap();
    
    if temperature_type.trim().parse::<char>().unwrap() == 'c' {
        let fahreneit = convert_celcius_to_fahreneit(temperature_value);
        println!("{} celcius is equals {} fahreneit", temperature_value, fahreneit);
    } else {
        let celcius = convert_fahreneit_to_celcius(temperature_value);
        println!("{} fahreneit is equals {} celcius", temperature_value, celcius);
    }
}

fn read_line_temperature_type(temperature_type: &mut String) {
    println!("please input 'c' for convert celcius to fahreneit");
    println!("please input 'f' for convert fahreneit to celcius");
        
    io::stdin().read_line(temperature_type)
                .expect("failed to read line");

    if temperature_type.trim().parse::<char>().unwrap() != 'c' && temperature_type.trim().parse::<char>().unwrap() != 'f'{
        
        println!("input is not valid");

        temperature_type.clear();
        read_line_temperature_type(temperature_type);
    }
}

fn convert_celcius_to_fahreneit(celcius: f32) -> f32 {
   (celcius * 9. / 5.) + 32.
}

fn convert_fahreneit_to_celcius(fahreneit: f32) -> f32 {
   (fahreneit - 32.) * (5. / 9.)
}
