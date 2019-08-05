mod protos;

use protobuf::*;
use protos::search::SearchRequest;

fn main() {
    println!("Hello, world!");
    let mut search_request = SearchRequest::new();
    search_request.set_query("query".to_string());
    search_request.set_page_number(25);
    search_request.set_result_per_page(100);

    // This might help as well
    // https://github.com/stepancheg/rust-protobuf/blob/980c04d65a2670168026ffd57448aead1b764c72/protobuf-test-common/src/serialize_deserialize_tests.rs
    let serialized_bytes = search_request.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<SearchRequest>(&serialized_bytes).unwrap();
    assert_eq!(search_request, parsed);

    println!("{:#?}", parsed);
}
