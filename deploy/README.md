# Deployments

The [Bartholomew](https://bartholomew.fermyon.dev) website is deployed via the [deploy.yaml](../.github/workflows/deploy.yml) GitHub workflow.

This website is a simple redirect app to redirect requests from `bartholomew.fermyon.dev/*` to `developer.fermyon.com/bartholomew/*`.

## Auto Deploys

The production version of the website is deployed whenever commits are pushed to the `main` branch.

## Manual Deploys

Deployments may also be [triggered manually](https://github.com/fermyon/bartholomew/actions/workflows/deploy.yml), providing a choice of `ref` and `commit`.

## Nomad jobs

We currently deploy the website via its Nomad job directly. (In the future, we envision running the website as a Fermyon Cloud app.)

The [bartholomew-docs](./bartholomew-docs.nomad) Nomad job contains configuration for the running website, including the OCI reference to run from.
