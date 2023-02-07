variable "region" {
  type    = string
}

variable "production" {
  type        = bool
  default     = false
  description = "Whether or not this job should run in production mode. Default: false."
}

variable "dns_domain" {
  type        = string
  default     = "fermyon.dev"
  description = "The root DNS domain for the Bartholomew docs website, e.g. fermyon.dev, fermyon.link"
}

variable "hostname" {
  type        = string
  default     = null
  description = "An alternative hostname to use (defaults are <canary>.bartholomew.<dns_domain>})"
}

variable "letsencrypt_env" {
  type    = string
  default = "prod"
  description = <<EOF
The Let's Encrypt cert resolver to use. Options are 'staging' and 'prod'. (Default: prod)

With the letsencrypt-prod cert resolver, we're limited to *5 requests per week* for a cert with matching domain and SANs.
For testing/staging, it is recommended to use letsencrypt-staging, which has vastly increased limits.
EOF

  validation {
    condition     = var.letsencrypt_env == "staging" || var.letsencrypt_env == "prod"
    error_message = "The Let's Encrypt env must be either 'staging' or 'prod'."
  }
}

variable "bindle_id" {
  type        = string
  default     = "bartholomew-docs/0.1.0"
  description = "A bindle id, such as foo/bar/1.2.3"
}

locals {
  hostname = "${var.hostname == null ? "${var.production == true ? "bartholomew.${var.dns_domain}" : "canary.bartholomew.${var.dns_domain}"}" : var.hostname}"
}

job "bartholomew-docs" {
  type = "service"
  datacenters = [
    "${var.region}a",
    "${var.region}b",
    "${var.region}c",
    "${var.region}d",
    "${var.region}e",
    "${var.region}f"
  ]

  group "bartholomew-docs" {
    count = 3

    update {
      max_parallel      = 1
      canary            = 3
      min_healthy_time  = "10s"
      healthy_deadline  = "10m"
      progress_deadline = "15m"
      auto_revert       = true
      auto_promote      = true
    }

    network {
      port "http" {}
    }

    service {
      name = "bartholomew-docs-${NOMAD_NAMESPACE}"
      port = "http"

      tags = [
        "traefik.enable=true",
        "traefik.http.routers.bartholomew-docs-${NOMAD_NAMESPACE}.rule=Host(`${local.hostname}`)",
        "traefik.http.routers.bartholomew-docs-${NOMAD_NAMESPACE}.entryPoints=websecure",
        "traefik.http.routers.bartholomew-docs-${NOMAD_NAMESPACE}.tls=true",
        "traefik.http.routers.bartholomew-docs-${NOMAD_NAMESPACE}.tls.certresolver=letsencrypt-cf-${var.letsencrypt_env}",
        "traefik.http.routers.bartholomew-docs-${NOMAD_NAMESPACE}.tls.domains[0].main=${local.hostname}"
      ]

      check {
        type     = "http"
        path     = "/.well-known/spin/health"
        interval = "10s"
        timeout  = "2s"
      }
    }

    task "server" {
      driver = "exec"

      artifact {
        source = "https://github.com/fermyon/spin/releases/download/v0.8.0/spin-v0.8.0-linux-amd64.tar.gz"
        options {
          checksum = "sha256:0ef31fe6e2b4d34ddd089b01a1f88820f88c456276bfe4e1477836a6087654c1"
        }
      }

      env {
        RUST_LOG   = "spin=trace"
        BINDLE_URL = "http://bindle.service.consul:3030/v1"
        BASE_URL   = "https://${local.hostname}"
      }

      config {
        command = "spin"
        args = [
          "up",
          "--listen", "${NOMAD_IP_http}:${NOMAD_PORT_http}",
          "--bindle", var.bindle_id,
          "--log-dir", "${NOMAD_ALLOC_DIR}/logs",
          "--temp", "${NOMAD_ALLOC_DIR}/tmp",

          # Set BASE_URL for Bartholomew to override default (localhost:3000)
          "-e", "BASE_URL=${BASE_URL}",
        ]
      }
    }
  }
}
