use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;

#[test]
fn test_models() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954");
    client.connect()?;

    let activity = activity::Activity::new()
        .state("A test".to_string())
        .details("A placeholder".to_string())
        .assets(
            activity::Assets::new()
                .large_image("large-image".to_string())
                .large_text("Large text".to_string()),
        )
        .buttons(vec![activity::Button::new(
            "A button".to_string(),
            "https://github.com".to_string(),
        )]);
    client.set_activity(activity)?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    client.close()?;
    Ok(())
}
