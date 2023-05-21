use speckle_rs::ObjectLoader;

fn main() {
    let stream_id = "0bacfc3aa6";
    let object_id = "67bf1bb1b8dbe4d6eb875abf79520096";
    let token = "420872d4db7bbeb7c6c543f18435b4ad7ae96d1917";

    let object_loader = ObjectLoader::new(stream_id, object_id, token);

    let object_iterator = object_loader.get_raw_object_iterator();
    object_loader.fetch_objects(object_iterator);
}
