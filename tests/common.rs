use cloudrun_server::CloudrunServer;

pub fn get_server() {
    CloudrunServer::new("0.0.0.0:8088")
}
