use async_std::task;
use surf;

// fetch data from a url and return the results as a string.
// if an error occurs, return the error.
async fn fetch(url: &str) -> Result<String, surf::Exception> {
    surf::get(url).recv_string().await
}

// execute the fetch function and print the results
async fn execute() {
    match fetch("https://pokeapi.co/api/v2/move/surf").await {
        Ok(s) => println!("Fetched results: {:#?}", s),
        Err(e) => println!("Got an error: {:?}", e),
    };
}

fn main() {
    task::block_on(execute());
    // ^ start the future and wait for it to finish
}