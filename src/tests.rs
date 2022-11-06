use sparkle_cache::tests::Tester;

use crate::Cache;

/// Tests all testing methods in Sparkle Cache
///
/// It combines all of them to avoid creating and deleting the testing guild for
/// each test
#[tokio::test]
async fn test_all() -> Result<(), anyhow::Error> {
    let cache = Cache::new(env!("DATABASE_URL")).await?;
    let mut tester = Tester::new(cache, env!("TEST_BOT_TOKEN")).await?;

    tester.current_user().await?;
    tester.channels().await?;
    tester.permission_overwrites().await?;
    tester.messages().await?;
    tester.members().await?;
    tester.guilds().await?;
    tester.roles().await?;
    tester.emojis().await?;
    // tester.stickers().await?;

    Ok(())
}
