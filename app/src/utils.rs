use web_sys::js_sys::{Array, Uint8Array};
use web_sys::{Blob, BlobPropertyBag, Url};

pub fn create_blob_url(mime: String, content: &Vec<u8>) -> String {
    let property = BlobPropertyBag::new();
    property.set_type(&mime);

    let blob = Blob::new_with_u8_slice_sequence_and_options(
        &Array::of1(&Uint8Array::from(content.as_slice())),
        &property,
    )
    .expect("Failed to create blob");

    Url::create_object_url_with_blob(&blob).expect("Failed to create blob URL")
}
