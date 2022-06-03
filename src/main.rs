use device_query::{DeviceEvents, DeviceState};
use std::fs::{OpenOptions, read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time;
use std::collections::HashMap;
use std::sync::Mutex;

const FILE_NAME: &str = "input.txt";

fn main() {
    let key_presses: Mutex<HashMap<String, u32>> = Mutex::from(read_file());

    // Stuff to get the keypresses
    let device_state = DeviceState::default();

    // Bascially guards for when a key press occurs
    let _guard = device_state.on_key_down(move |key| {
        let mut k_p = key_presses.lock().unwrap();
        {
            let num_pressed = k_p.entry(key.to_string()).or_insert(0);
            *num_pressed += 1;
            println!("{} times pressed: {}", key.to_string(), num_pressed);
        }
    
        match save_info_to_file(k_p.clone()){
            Ok(_) => println!("the file was updated"),
            Err(_) => panic!("Something went wrong with the file"),
        }
    });

    // todo figure out multithreading so that key presses are 
    // todo accessible across multiple threads

    // let _guard = device_state.on_mouse_down(move |key| {
    //     let mouse_str = format!("Mouse #{}", &key.to_string());
    //     let mut k_p = key_presses.lock().unwrap();
    //     {
    //         let num_pressed = k_p.entry(mouse_str).or_insert(0);
    //         *num_pressed += 1;
    //         println!("{} times pressed: {}", key.to_string(), num_pressed);
    //     }
    
    //     save_info_to_file(k_p.clone());
    // });
    
    let sleep_time = time::Duration::from_secs(1);
    loop {
        sleep(sleep_time)   
    }

    
}

fn read_file() -> HashMap<String, u32> {
    // Make sure the file exists
    if Path::new(FILE_NAME).exists() == false{
        File::create(FILE_NAME).unwrap();
    }

    // Read the file_string and get rid of extra quotations
    let data = read_to_string(FILE_NAME)
        .unwrap()
        .replace('\"', "");

    // Convert the file data to a useable iterator
    let data_iter = data
        .lines()
        .into_iter()
        .skip(1) // Skip because of the "key", "pressed"
        .map(|s| s.split(",").collect::<Vec<&str>>())
        .map(
            |v| 
            (v[0].to_string(), v[1].parse::<u32>().expect("Failed to parse"))
        );

    // add the data into the hashmap
    let mut key_pressed = HashMap::new();
    for (key, pressed) in data_iter{
        key_pressed.insert(key, pressed);
    }

    // return the result
    key_pressed
}

fn save_info_to_file(key_presses: HashMap<String, u32> ) -> Result<(), std::io::Error>{
    let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(FILE_NAME)?;


    let mut file_output: String = String::from("\"key\",\"presses\"\n");

    for (key, presses) in key_presses{
        file_output.push_str(
            &format!("\"{}\",{}\n",key, presses)
        );
    }
    
    f.write_all(file_output.as_bytes())?;
    Ok(())
}

