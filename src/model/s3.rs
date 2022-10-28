use rusoto_core::ByteStream;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, StreamingBody, S3};

use crate::error::ServerError;

pub struct SharkS3Client {
    pub bucket_name: String,
    pub client: rusoto_s3::S3Client,
}

impl SharkS3Client {
    pub fn new(name: String, s3: S3Client) -> SharkS3Client {
        SharkS3Client {
            bucket_name: name,
            client: s3,
        }
    }

    pub async fn put_object(
        &self,
        ct: String,
        length: i64,
        obj_key: String,
        contents: StreamingBody,
    ) -> crate::Result<()> {
        self.client
            .put_object(PutObjectRequest {
                content_type: Some(ct),
                content_length: Some(length),
                bucket: self.bucket_name.clone(),
                key: obj_key,
                body: Some(contents),
                ..Default::default()
            })
            .await
            .map_err(|_| ServerError::OtherWithMessage("Failed to put object.".to_string()))?;
        Ok(())
    }

    pub async fn get_object(&self, obj_key: String) -> crate::Result<ByteStream> {
        let mut obj = self
            .client
            .get_object(GetObjectRequest {
                bucket: self.bucket_name.clone(),
                key: obj_key,
                ..Default::default()
            })
            .await
            .map_err(|_| ServerError::OtherWithMessage("Failed to get object.".to_string()))?;
        let stream = obj.body.take().unwrap();
        // let body = StreamBody::new(stream);
        Ok(stream)
    }
}
