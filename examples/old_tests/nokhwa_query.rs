use nokhwa::query;
use nokhwa::utils::ApiBackend;

// NOTES: there are 2 cameras found, and the second one should be ignored.

fn main() {
    // this is to test getting the values from nokhwa on the current available cameras
    for each in query(ApiBackend::Auto).unwrap() {
        println!("{:#?}", each);
    }
}
