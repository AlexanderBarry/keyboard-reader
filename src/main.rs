use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::{HashSet, BTreeMap};
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::{thread, time};

const FILE_NAME: &str = "input.csv";

fn main() {
    // holds the key presses
    // Used BTreeMap so that the keys are alphabetized
    let mut key_presses: BTreeMap<String, u32> = read_file();

    // Keeps track of device state
    let device_state = DeviceState::new();

    // Sleep duration
    let sleep_time = time::Duration::from_millis(25);

    // used for finding when a key is actually pressed
    let mut old_key_state: HashSet<Keycode> = HashSet::new();

    loop {
        let new_presses = update_presses(
            device_state.get_keys(),
            &mut old_key_state,
            &mut key_presses,
        );

        if new_presses == true {
            save_presses_to_file(&key_presses);
        }

        thread::sleep(sleep_time);
    }
}

fn update_presses(
    current_keys: Vec<Keycode>,
    old: &mut HashSet<Keycode>,
    presses: &mut BTreeMap<String, u32>,
) -> bool {
    // hashset that will keep track of current keys
    let mut current = HashSet::new();

    // tracks if there were new presses
    let mut new_keydown = false;

    // loop throught the keys currently being pressed
    for key in current_keys {
        // Checks for if there was a new press
        if !old.contains(&key) {
            // If there was a new press then update the presses
            new_keydown = true;
            let num_pressed = presses.entry(key.to_string()).or_insert(0);
            *num_pressed += 1;
            println!("{} times pressed: {}", key.to_string(), num_pressed);
        }

        // update the current hash
        current.insert(key);
    }

    // update the old hashset
    *old = current;

    new_keydown
}

fn read_file() -> BTreeMap<String, u32> {
    // Make sure the file exists
    if Path::new(FILE_NAME).exists() == false {
        File::create(FILE_NAME).unwrap();
    }

    // Read the file_string and get rid of extra quotations
    let data = read_to_string(FILE_NAME).unwrap().replace('\"', "");

    // Convert the file data to a useable iterator
    let data_iter = data
        .lines()
        .into_iter()
        .skip(1) // Skip because of the "key", "pressed"
        .map(|s| s.split(",").collect::<Vec<&str>>())
        .map(|v| {
            (
                v[0].to_string(),
                v[1].parse::<u32>().expect("Failed to parse"),
            )
        });

    // add the data into the hashmap
    let mut key_pressed = BTreeMap::new();
    for (key, pressed) in data_iter {
        key_pressed.insert(key, pressed);
    }

    // return the result
    key_pressed
}

/// Saves key presses to file in a csv format
fn save_presses_to_file(key_presses: &BTreeMap<String, u32>) {
    // gets the file
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(FILE_NAME)
        .expect("The file could not be opened");

    // string that will be written to the file
    let mut file_output: String = String::from("key,presses\n");

    // adds each key info to the file string
    for (key, presses) in key_presses {
        file_output.push_str(&format!("{},{}\n", key, presses));
    }

    // writes the info to the file
    f.write_all(file_output.as_bytes())
        .expect("error writing to file");

    println!("saved presses to file");
}
