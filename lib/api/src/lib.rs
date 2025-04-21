pub const PUSH_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("pushes_descriptor");

tonic::include_proto!("flux.pushes");

tonic::include_proto!("flux.events");
