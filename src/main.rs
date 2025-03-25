use dotenv::dotenv;
use speckle_rs::graphql;
use speckle_rs::ObjectLoader;

fn main() {
    dotenv().ok();

    let project_id = std::env::args().nth(1).expect("no project_id given");
    let object_id = std::env::args().nth(2).expect("no object_id given");

    let token = std::env::var("TOKEN").unwrap_or("".to_string());
    // graphql(stream_id, object_id);
    if token.is_empty() {
        println!("WARN: no token given.");
    }

    let object_loader = ObjectLoader::new(&project_id, &object_id, &token);

    let object_iterator = object_loader
        .get_raw_object_iterator()
        .expect("Object was not found");

    let object_response = object_loader
        .fetch_objects(object_iterator)
        .expect("Failed to fetch objects");

    object_loader
        .store_response(object_response)
        .expect("Unable to store object");
}

fn graphql(stream_id: &str, object_id: &str) {
    let client = graphql::GQLClient::new("420872d4db7bbeb7c6c543f18435b4ad7ae96d1917");
    let request = graphql::QueryBuilder::new(stream_id, object_id)
        .where_equals("level.name", "5FL")
        .select("type")
        .build()
        .json();

    println!("{}", request);

    match client.send_query(request) {
        Some(res) => println!("{}", res),
        None => println!("Error"),
    }
}
