variable "service_name" {
default = "saint-service"
}

variable "app_version" {
  default = "latest"
}

variable "organisation" {
  default = "HocVienCongGiao"
}

variable "environment" {
  default = "dev-sg"
}

variable "aws_region" {
  type    = string
  default = "ap-southeast-1"
}

variable "aws_access_key_id" {}
variable "aws_secret_access_key" {}
variable "aws_account_id" {}


variable "db_host" {}
variable "db_user" {}
variable "db_password" {}
variable "db_name" {}
