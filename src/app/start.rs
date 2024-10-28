use dotenv::dotenv;

// define here values
#[allow(dead_code)]
pub struct Values {}


// Initialize the values struct
pub async fn start() -> Values {
    dotenv().ok();
    // Get neccessary .env variables

    Values {  }
}
