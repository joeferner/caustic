use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use serde::{Deserialize, Serialize};

pub async fn write_json_to_s3<T: Serialize>(
    client: &S3Client,
    bucket: &str,
    key: &str,
    data: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string(data)?;

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(json_string.into_bytes()))
        .content_type("application/json") // Set content type
        .send()
        .await?;

    println!("Uploaded JSON to s3://{}/{}", bucket, key);
    Ok(())
}

pub async fn read_json_from_s3<T: for<'de> Deserialize<'de>>(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let resp = client.get_object().bucket(bucket).key(key).send().await;

    // Check if the error is NoSuchKey (file doesn't exist)
    match resp {
        Ok(output) => {
            // File exists, read and parse it
            let data = output.body.collect().await?;
            let bytes = data.into_bytes();
            let parsed: T = serde_json::from_slice(&bytes)?;
            Ok(Some(parsed))
        }
        Err(err) => {
            // Check if it's a NoSuchKey error
            if let Some(service_err) = err.as_service_error()
                && service_err.is_no_such_key()
            {
                return Ok(None);
            }
            // If it's a different error, propagate it
            Err(Box::new(err))
        }
    }
}

pub async fn write_to_s3(
    client: &S3Client,
    bucket: &str,
    key: &str,
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data.to_vec()))
        .send()
        .await?;

    println!("Uploaded to s3://{}/{}", bucket, key);
    Ok(())
}

pub async fn read_from_s3(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = client.get_object().bucket(bucket).key(key).send().await?;

    let data = resp.body.collect().await?;
    Ok(data.into_bytes().to_vec())
}

/// Converts an email address into a valid S3 key by replacing or removing
/// characters that are problematic in S3 keys.
///
/// S3 keys can technically contain most characters, but some should be avoided:
/// - Special characters like &, $, @, =, ;, :, +, space, comma
/// - Characters that need URL encoding
///
/// This function:
/// - Replaces '@' with '_at_'
/// - Replaces '.' with '_'
/// - Replaces other special characters with '-'
/// - Converts to lowercase for consistency
pub fn email_to_s3_key(email: &str) -> String {
    email
        .to_lowercase()
        .chars()
        .map(|c| match c {
            '@' => "_at_".to_string(),
            '.' => "_".to_string(),
            'a'..='z' | '0'..='9' | '-' | '_' => c.to_string(),
            _ => "-".to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_email() {
        assert_eq!(email_to_s3_key("user@example.com"), "user_at_example_com");
    }

    #[test]
    fn test_email_with_dots() {
        assert_eq!(
            email_to_s3_key("first.last@example.com"),
            "first_last_at_example_com"
        );
    }

    #[test]
    fn test_email_with_special_chars() {
        assert_eq!(
            email_to_s3_key("user+tag@example.com"),
            "user-tag_at_example_com"
        );
    }

    #[test]
    fn test_uppercase_email() {
        assert_eq!(email_to_s3_key("User@Example.COM"), "user_at_example_com");
    }
}
