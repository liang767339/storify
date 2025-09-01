use crate::*;
use storify::error::Result;
use storify::storage::StorageClient;

pub fn tests(client: &StorageClient, tests: &mut Vec<Trial>) {
    tests.extend(async_trials!(client, test_cat_small_file));
}

async fn test_cat_small_file(_client: StorageClient) -> Result<()> {
    let (path, content, _) = TEST_FIXTURE.new_file(_client.operator());
    _client.operator().write(&path, content.clone()).await?;

    let output = storify_cmd()
        .arg("cat")
        .arg(&path)
        .output()
        .expect("Failed to execute command");
    assert_eq!(
        output.stdout.len(),
        content.len(),
        "Output size mismatch: expected {} bytes, got {} bytes",
        content.len(),
        output.stdout.len()
    );
    Ok(())
}
