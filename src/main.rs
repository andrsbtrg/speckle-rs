use speckle_rs::ObjectLoader;

fn main() {
    let stream_id = "a41ecf35bc";
    let object_id = "5ac251da6ca7e2be1c90ef3a33ba4655";
    let token = "420872d4db7bbeb7c6c543f18435b4ad7ae96d1917";

    let object_loader = ObjectLoader::new(stream_id, object_id, token);

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
