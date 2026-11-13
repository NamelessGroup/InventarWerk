# Builds all images with one command:
#   docker buildx bake                 # all images
#   docker buildx bake backend         # only the three API images
#   docker buildx bake web --load      # build one image and load it into the local docker
#
# In CI the meta-* targets are overridden by docker/metadata-action (tags + labels).

variable "REGISTRY" {
  default = "ghcr.io/namelessgroup/inventarwerk"
}

variable "TAG" {
  default = "latest"
}

# Version shown in the frontend (short commit hash)
variable "VERSION" {
  default = "dev"
}

group "default" {
  targets = ["inventarwerk-api", "zauberwerk-api", "last-changes-api", "web"]
}

group "backend" {
  targets = ["inventarwerk-api", "zauberwerk-api", "last-changes-api"]
}

# ---------- tags (overridden in CI) ----------
target "meta-inventarwerk-api" {
  tags = ["${REGISTRY}/inventarwerk-api:${TAG}"]
}

target "meta-zauberwerk-api" {
  tags = ["${REGISTRY}/zauberwerk-api:${TAG}"]
}

target "meta-last-changes-api" {
  tags = ["${REGISTRY}/last-changes-api:${TAG}"]
}

target "meta-web" {
  tags = ["${REGISTRY}/web:${TAG}"]
}

# ---------- backend ----------
# All three targets share the builder stage, so Rust is only compiled once.
target "_backend" {
  context    = "."
  dockerfile = "backend/Dockerfile"
}

target "inventarwerk-api" {
  inherits = ["_backend", "meta-inventarwerk-api"]
  target   = "inventarwerk_api"
}

target "zauberwerk-api" {
  inherits = ["_backend", "meta-zauberwerk-api"]
  target   = "zauberwerk_api"
}

target "last-changes-api" {
  inherits = ["_backend", "meta-last-changes-api"]
  target   = "last_changes_api"
}

# ---------- frontend + reverse proxy ----------
target "web" {
  inherits   = ["meta-web"]
  context    = "."
  dockerfile = "frontend/Dockerfile"
  args = {
    VERSION = VERSION
  }
}
