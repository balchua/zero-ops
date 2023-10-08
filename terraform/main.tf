provider "digitalocean" {
  token             = var.digitalocean_token
  spaces_access_id  = var.spaces_access_key_id
  spaces_secret_key = var.spaces_secret_access_key
}

resource "digitalocean_spaces_bucket" "zero_ops" {
  name          = var.bucket_name
  region        = var.region
  force_destroy = var.force_destroy
}
