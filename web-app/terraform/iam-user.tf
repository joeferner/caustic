resource "aws_iam_user" "webapp" {
  name = "caustic-rust-raytracer-webapp"
}

resource "aws_iam_access_key" "webapp" {
  user = aws_iam_user.webapp.name
}

resource "aws_iam_role" "s3_access_role" {
  name = "s3-read-write-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      }
    ]
  })
}

output "webapp_access_key_id" {
  value = aws_iam_access_key.webapp.id
  description = "Access Key ID"
}

output "webapp_secret_access_key" {
  value     = aws_iam_access_key.webapp.secret
  sensitive = true
  description = "Secret Access Key - run 'terraform output webapp_secret_access_key' to view"
}
