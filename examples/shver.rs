extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Basic;
use hyper::header::Authorization;


fn main() {
        // Create a client.
        let client = Client::new();
        //let user = String.new("admin");
        let basic = Basic{username: "admin".to_owned(), password: Some("admin".to_owned())};

        //let mut headers = Headers::new();
        //headers.set(auth);
            // Creating an outgoing request.
            let mut res = client.post("https://bleaf4/command-api")
                        // set a header
                        .header(Authorization(basic))
                        .body(r#"{
                                "jsonrpc": "2.0",
                                "method": "runCmds",
                                "params": {
                                    "version": 1,
                                    "cmds": [
                                    "show version"
                                    ],
                                    "format": "json",
                                    "timestamps": false
                                    },
                                "id": "1"
                                  }"#)
                                // let 'er go!
                                .send().unwrap();

                // Read the Response.
                let mut body = String::new();
                res.read_to_string(&mut body).unwrap();

                println!("Response: {}", body);
}
