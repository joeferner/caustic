use anyhow::{Context, Result, anyhow};
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream, types::ObjectIdentifier};
use serde::{Deserialize, Serialize};

pub async fn write_json_to_s3<T: Serialize>(
    client: &S3Client,
    bucket: &str,
    key: &str,
    data: &T,
) -> Result<()> {
    let json_string = serde_json::to_string(data)
        .with_context(|| format!("writing json to s3://{bucket}/{key}"))?;
    write_to_s3(
        client,
        bucket,
        key,
        "application/json",
        ByteStream::from(json_string.into_bytes()),
    )
    .await
}

pub async fn read_json_from_s3<T: for<'de> Deserialize<'de>>(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Option<T>> {
    let resp = read_from_s3(client, bucket, key)
        .await
        .with_context(|| format!("reading json from s3://{bucket}/{key}"))?;
    if let Some(resp) = resp {
        let data = resp.body.collect().await?;
        let bytes = data.into_bytes();
        let parsed: T = serde_json::from_slice(&bytes)?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

pub async fn write_to_s3(
    client: &S3Client,
    bucket: &str,
    key: &str,
    content_type: &str,
    data: ByteStream,
) -> Result<()> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(data)
        .content_type(content_type)
        .send()
        .await
        .with_context(|| format!("writing to s3://{bucket}/{key}"))?;
    Ok(())
}

pub async fn copy_s3_file(
    client: &S3Client,
    bucket: &str,
    from_key: &str,
    to_key: &str,
) -> Result<()> {
    client
        .copy_object()
        .copy_source(format!("{bucket}/{from_key}"))
        .bucket(bucket)
        .key(to_key)
        .send()
        .await
        .with_context(|| format!("copying s3://{bucket}/{from_key} to s3://{bucket}/{to_key}"))?;
    Ok(())
}

pub async fn delete_s3_objects_with_prefix(
    client: &S3Client,
    bucket: &str,
    prefix: &str,
) -> Result<()> {
    let mut continuation_token: Option<String> = None;

    loop {
        // List objects with the prefix
        let mut list_request = client.list_objects_v2().bucket(bucket).prefix(prefix);
        if let Some(token) = continuation_token {
            list_request = list_request.continuation_token(token);
        }

        let response = list_request
            .send()
            .await
            .with_context(|| format!("listing s3://{bucket}/{prefix}"))?;

        // Get the objects from the response
        if response.contents().is_empty() {
            break;
        }

        // Prepare objects for deletion
        let to_delete = {
            let mut to_delete: Vec<ObjectIdentifier> = vec![];
            for obj in response.contents() {
                if let Some(key) = &obj.key {
                    to_delete.push(ObjectIdentifier::builder().key(key).build()?);
                }
            }
            to_delete
        };

        if !to_delete.is_empty() {
            // Delete the objects
            let delete_response = client
                .delete_objects()
                .bucket(bucket)
                .delete(
                    aws_sdk_s3::types::Delete::builder()
                        .set_objects(Some(to_delete.clone()))
                        .build()?,
                )
                .send()
                .await?;

            // Check for errors
            if !delete_response.errors().is_empty() {
                let mut message = String::new();
                for error in delete_response.errors() {
                    message += &format!(
                        "Error deleting {}: {:?}",
                        error.key().unwrap_or("unknown"),
                        error.message()
                    );
                }
                return Err(anyhow!(message));
            }
        }

        // Check if there are more objects to list
        continuation_token = response.next_continuation_token().map(|s| s.to_string());
        if continuation_token.is_none() {
            break;
        }
    }

    Ok(())
}

pub struct ReadFromS3Data {
    pub content_type: Option<String>,
    pub body: ByteStream,
}

pub async fn read_from_s3(
    client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<Option<ReadFromS3Data>> {
    let resp = client.get_object().bucket(bucket).key(key).send().await;
    match resp {
        Ok(resp) => {
            let content_type = resp.content_type().map(|s| s.to_string());
            let body = resp.body;
            Ok(Some(ReadFromS3Data { content_type, body }))
        }
        Err(err) => {
            // Check if it's a NoSuchKey error
            if let Some(service_err) = err.as_service_error()
                && service_err.is_no_such_key()
            {
                Ok(None)
            } else {
                Err(anyhow!("reading s3://{bucket}/{key}: {err}"))
            }
        }
    }
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
