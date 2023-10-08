variable "digitalocean_token" {}

variable "spaces_access_key_id" {}

variable "spaces_secret_access_key" {}

variable "bucket_name" {
  type    = string
  default = "my-zero-ops"
}

variable "region" {
  type    = string
  default = "sgp1"
}

variable "force_destroy" {
  type    = bool
  default = true
}
