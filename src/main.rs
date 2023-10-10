use dotenv::dotenv;

struct Login {
    authenticationkey: String,
}

enum ObjectType {
    WeatherObservation,
}

struct Filter {
}

enum Include {
    AirTemperature,
}

struct Query {
    objectype: ObjectType,
    schemaversion: String,
    limit: i32,
    filter: Vec<Filter>,
    include: Vec<Include>,
}

struct Request {
    login: Login,
    query: Query,
}

fn main() {
    dotenv().ok();

    let api_key = std::env::vars().find_map(|(k, v)| if k=="API_KEY" { Some(v) } else { None }).unwrap();
    let query = r#"{
                     "REQUEST": {
                       "LOGIN": {"authenticationkey":"{}" },
                       "QUERY": [{
                         "objecttype": "WeatherObservation",
                         "schemaversion": 2,
                         "limit": 10,
                         "FILTER": {"EQ": [{"name": "Id", "value": "SE_STA_VVIS251"}]},
                         "INCLUDE": ["Air.Temperature"]
                       }]
                     }
                   }"#;
    let query = query.replace("{}", &api_key);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://api.trafikinfo.trafikverket.se/v2/data.json")
        .body(query)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send().unwrap();
    if resp.status().is_success() {
        println!("Success!");
    } else {
        println!("failed...");
    }
    println!("{}", resp.text().unwrap());
}
