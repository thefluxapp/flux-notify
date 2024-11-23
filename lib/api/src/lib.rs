pub const NOTIFY_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("notify_descriptor");

tonic::include_proto!("flux.notify");
