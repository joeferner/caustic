
resource "random_id" "bucket_suffix" {
  byte_length = 8
}

resource "aws_s3_bucket" "app_bucket" {
  bucket = "caustic-rust-raytracer-${random_id.bucket_suffix.hex}"
}

resource "aws_iam_policy" "s3_read_write_policy" {
  name        = "s3-bucket-read-write-policy"
  description = "Policy for read/write access to specific S3 bucket"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "s3:GetObject",
          "s3:PutObject",
          "s3:DeleteObject",
          "s3:ListBucket",
          "s3:GetObjectVersion"
        ]
        Resource = [
          aws_s3_bucket.app_bucket.arn,
          "${aws_s3_bucket.app_bucket.arn}/*"
        ]
      }
    ]
  })
}

resource "aws_iam_user_policy_attachment" "attach_s3_policy" {
  user       = aws_iam_user.webapp.name
  policy_arn = aws_iam_policy.s3_read_write_policy.arn
}

output "bucket_name" {
  value = aws_s3_bucket.app_bucket.id
}
