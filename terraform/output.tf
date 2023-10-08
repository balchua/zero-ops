output "bucket_url" {
  value = "https://${digitalocean_spaces_bucket.zero_ops.name}.${digitalocean_spaces_bucket.zero_ops.region}.digitaloceanspaces.com"
}
