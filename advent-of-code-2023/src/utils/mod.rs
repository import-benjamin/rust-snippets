mod fetch;
mod store;

pub fn get_input(day: u8) -> String {
    if !store::input_is_stored(day) {
        let input = fetch::game_input(day);
        store::store_input(day, input);
    }

    store::get_stored_input(day)
}
