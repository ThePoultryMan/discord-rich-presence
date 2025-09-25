use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_reconnect() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    loop {
        if client.connect().is_ok() {
            break;
        }
    }

    loop {
        let payload = activity::Activity::new()
            .state("part 1 (test)".to_string())
            .details("a placeholder".to_string())
            .assets(
                activity::Assets::new()
                    .large_image("large-image".to_string())
                    .large_text("a thing".to_string()),
            );

        if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
            continue;
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}
