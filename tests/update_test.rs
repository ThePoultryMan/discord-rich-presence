use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_updating() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    client.connect()?;

    client.set_activity(
        activity::Activity::new()
            .state("part 1 (test)".to_string())
            .details("a placeholder".to_string())
            .assets(
                activity::Assets::new()
                    .large_image("large-image".to_string())
                    .large_text("a thing".to_string()),
            ),
    )?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.set_activity(
        activity::Activity::new()
            .state("part 2 (test)".to_string())
            .details("a placeholder".to_string())
            .assets(
                activity::Assets::new()
                    .large_image("small-image".to_string())
                    .large_text("a thing".to_string()),
            ),
    )?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    client.close()?;
    Ok(())
}
