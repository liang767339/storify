use anyhow::Result;
use opendal::EntryMode;
use opendal::Operator;
use opendal::services::Oss;
use std::path::Path;
use tokio::fs;

// Build Operator
async fn build_operator(
    bucket: &str,
    endpoint: &str,
    access_key_id: &str,
    access_key_secret: &str,
) -> Result<Operator> {
    let builder = Oss::default()
        .bucket(bucket)
        .endpoint(endpoint)
        .access_key_id(access_key_id)
        .access_key_secret(access_key_secret);

    Ok(Operator::new(builder)?.finish())
}

// List Directory
pub async fn list_directory(
    path: String,
    bucket: String,
    endpoint: String,
    access_key_id: String,
    access_key_secret: String,
) -> Result<()> {
    let builder = Oss::default()
        .bucket(&bucket)
        .endpoint(&endpoint)
        .access_key_id(&access_key_id)
        .access_key_secret(&access_key_secret);

    let op = Operator::new(builder)?.finish();

    let entries = op.list(&path).await?;

    for entry in entries {
        let entry_path = entry.path();
        let is_dir = entry.metadata().mode().is_dir();

        if is_dir {
            // Use Box::pin to call the async function recursively
            Box::pin(list_directory(
                entry_path.to_string(),
                bucket.clone(),
                endpoint.clone(),
                access_key_id.clone(),
                access_key_secret.clone(),
            ))
            .await?;
        } else {
            println!("File: {}", entry_path);
        }
    }

    Ok(())
}

// Download Files
pub async fn download_files(
    remote_path: String,
    local_path: String,
    bucket: String,
    endpoint: String,
    access_key_id: String,
    access_key_secret: String,
) -> Result<()> {
    let op = build_operator(&bucket, &endpoint, &access_key_id, &access_key_secret).await?;

    fs::create_dir_all(&local_path).await?;

    let entries = op.list(&remote_path).await?;

    for entry in entries {
        let meta = entry.metadata();
        let remote_path = entry.path();
        let path_obj = Path::new(&remote_path);
        let file_name = path_obj
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let local_full_path = Path::new(&local_path).join(file_name);

        if meta.mode() == EntryMode::DIR {
            let new_remote = format!("{}/", remote_path);
            let new_local = local_full_path.to_str().unwrap().to_string();

            fs::create_dir_all(&new_local).await?;
            Box::pin(download_files(
                new_remote,
                new_local,
                bucket.clone(),
                endpoint.clone(),
                access_key_id.clone(),
                access_key_secret.clone(),
            ))
            .await?;
        } else {
            let data = op.read(&remote_path).await?;
            let bytes = data.to_vec();
            fs::write(&local_full_path, &bytes).await?;
            println!(
                "Downloaded: {} → {}",
                remote_path,
                local_full_path.display()
            );
        }
    }

    Ok(())
}

// Count Files
pub async fn count_files_in_directory(
    path: String,
    bucket: String,
    endpoint: String,
    access_key_id: String,
    access_key_secret: String,
) -> Result<usize> {
    let op = build_operator(&bucket, &endpoint, &access_key_id, &access_key_secret).await?;
    let mut file_count = 0;

    let entries = op.list(&path).await?;

    for entry in entries {
        let meta = entry.metadata();
        let is_dir = meta.mode() == EntryMode::DIR;

        if is_dir {
            let sub_path = entry.path();
            // Use Box::pin to wrap recursive call
            file_count += Box::pin(count_files_in_directory(
                sub_path.to_string(),
                bucket.clone(),
                endpoint.clone(),
                access_key_id.clone(),
                access_key_secret.clone(),
            ))
            .await?;
        } else {
            file_count += 1;
        }
    }

    Ok(file_count)
}
