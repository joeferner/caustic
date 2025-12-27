resource "aws_s3_object" "examples" {
  for_each = fileset("${path.module}/store/", "**/*")

  bucket = aws_s3_bucket.app_bucket.id
  key    = "store/${each.value}"
  source = "${path.module}/store/${each.value}"
  content_type = lookup(
    {
      "scad" = "application/x-openscad"
      "json" = "application/json"
      "txt"  = "text/plain"
      "png"  = "image/png"
      "jpg"  = "image/jpeg"
      "jpeg" = "image/jpeg"
      "gif"  = "image/gif"
      "svg"  = "image/svg+xml"
    },
    split(".", each.value)[length(split(".", each.value)) - 1],
    "application/octet-stream"
  )
  etag = filemd5("${path.module}/store/${each.value}")
}
