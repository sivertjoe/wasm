mod ws;

type SError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
pub async fn main() -> Result<(), SError>
{
    ws::spawn_web_socket_server().await;
    Ok(())
}
