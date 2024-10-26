# Express clone in Rust!

Welp, this is actaully my first big project in rust, overall very nice languege with a lot of nice to have features.

I decided I hate javascript and node servers in particular, so I've came up with the so original idea of trying to build my own HTTP 1.1 client using rust which was a meaningful learning experience for me the mere infrerior javascript developer.

Express seemed like a very comfortable place to take inspiration from

## Features

- HTTP 1.1 - Who would've thought about this😶
- Text, Json and File Responses
- COOKIES
- `GET` `POST` `PUT` `DELETE`
- ERRORS
- Expandable and very easy to manage folder structure!
- Core network code is on the project!
- Query params support

## Usage

1. Installation

   ```bash
   git clone https://github.com/roee1454/rust-http-server
   ```
   And thats it! Shocking!!!
   From there just hit

   ```bash
   cargo run
   ```
2. Explanation

   So I am going to be pretty straight forward here, If you are here I assume you know what you are doing, so try to understand my fuzzy explanation

   You want to start to code at the `src/app/index.rs` file, this is the file where all the endpoints are created

   There you'll have a `&mut Router` passed down to you
   `Router` is a class which gives you the ability to define whatever endpoint you want

   important methods:

   `router.get(endpoint, handler);`

   `router.post(endpoint, handler);`

   `router.put(endpoint, handler);`

   `router.delete(endpoint, handler); `

   `Handler` is a clousere which contains a `&Request`

   this `&Request` contains all the info about the request's url, headers and body
3. Examples

   - Hello world example:

   ```rust
   router.get("/", |_| {
       let response = Response::new();
       response.text("Hello, World", 200) // Returns hello, world with a status code 200
   });
   ```
   - Json example:

   ```rust
   router.get("/", |_| {
        let response = Response::new();
        response.json(json!({ "id": 1 }), 200) // Returns a json object with a status code 200
   });
   ```
   - Send File example:

   ```rust
   router.get("/file", |_| {
       let response = Response::new();
       response.send_file("/path/to/file", 200) // Filename is the original's filename. Returns an http response contains the file as a downloadable with status code 200
   })
   ```
   - Query Params and Error example:

   ```rust
   router.get("/user", |req: &Request| {
       let response = Response::new(); // Initiating a new response
       let query: &HashMap<String, String> = &req.query; // Query is just a reference to a hashmap, which contains all the query params recieved from your request
       match query.get("id") { // Getting a param from the hashmap
           Some(id) => {
               response.text(&format!("Found user with id: {id}"), 200) // Returns a regular text response with his user id, status code 200
           }
           None => {
               response.error("You need to provide an id", 400) // Returns a response with a status text of "Internal Server Error", status code 400
           }
        }
   })
   ```
   - Cookies and requset body example:

   ```rust
   router.post("new-user", |req: &Request| {
       let request_body: &HashMap<String, String> = &req.body; // The request body is just a reference to a hashmap, contains all the json data sent to the backend, if the data is only text, you'll need to get the 'body' param

       let text = request_body.get("body").unwrap(); // If not json, get this param!!!
       let user_data = request_body.get("user_data").unwrap(); // Getting a json param
       // Create user in the data base..

       let mut response = Response::new(); // Initiating a new response
       response.cookies.insert("session_id".to_string(), "ILoveCookies".to_string()); // Inserting a cookie into the cookies hashmap
       response.json(json!({
           "message": "user is created"
       }), 200) // Returns a json response with a status code 200, cookies are also sent back to the client
   })
   ```
