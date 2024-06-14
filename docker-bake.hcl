group "default" {
  targets = ["kanatrans"]
}

variable "VERSION" {
  default = "latest"
}

target "kanatrans" {
  target = "kanatrans"
  tags = [
    "ghcr.io/hexium310/kanatrans:latest",
    "ghcr.io/hexium310/kanatrans:${VERSION}",
  ]
}
