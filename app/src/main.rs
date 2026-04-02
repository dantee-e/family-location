mod create_user;
mod get_user_locations;
mod helpers;
mod keys_struct;
mod submit_location;

use crate::get_user_locations::get_locations;
use crate::helpers::LatLong;
use rand::Rng;
use rand::thread_rng;
use submit_location::submit_location;

use crate::create_user::register_user;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg1 = args.get(1).unwrap().as_str();
    match arg1 {
        "--new-user" => {
            let username = args.get(2).unwrap().clone();
            register_user(username);
        }
        "--random-loc" => {
            let username = args.get(2).unwrap().clone();
            let mut rng = thread_rng();

            let rand_loc = LatLong {
                latitude: rng.gen_range(-90.0..=90.0),
                longitude: rng.gen_range(-180.0..=180.0),
            };
            submit_location(&username, rand_loc);
        }
        "--get-user-loc" => {
            println!("Runnign get user loc");
            let username = args.get(2).unwrap().clone();
            get_locations(username);
        }
        _ => {}
    }
}
