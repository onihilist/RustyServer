
#[cfg(test)]
mod tests {

    use std::process::Command;
    use crate::server::startServer;

    #[tokio::test]
    async fn test_serverStart() {
        startServer().await;
    }

    #[tokio::test]
    async fn test_dbconnect() {
        startServer().await;
        // curl telnet://127.0.0.1:42000 <<< INIT_CONNECTION::onhlt::server::test
        Command::new("C:/Windows/System32/curl")
            .arg("telnet://127.0.0.1:42000")
            .arg("<<<")
            .arg("INIT_CONNECTION::onhlt::server::test")
            .spawn()
            .expect("ls command failed to start");
    }
}