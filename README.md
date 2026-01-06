outsider
===========================

[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/outsider-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/outsider)
[<img alt="crates.io" src="https://img.shields.io/crates/v/outsider.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/outsider)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-outsider-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/outsider)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/outsider/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/outsider/actions?query=branch%3Amaster)

Test artifact changes locally from multiple projects that already use [gitlab-art](https://github.com/kosma/gitlab-art).

## Example
Project-a depends on artifacts from `kosma/foobar-firmware`. You've made changes to the firmware and want to test them locally in project-a before pushing to GitLab.

### project-a/artifacts.yml
```yml
- project: kosma/foobar-firmware
  ref: 1.4.0
  job: firmware-8051
  install:
    build/8051/release/firmware.bin: artifacts/blobs/firmware-8051.blob
```

From the firmware repo, just run:

```console
# build the firmware
$ make

# copy to project-a
$ outsider ../project-a/artifacts.yml
Processing project: kosma/foobar-firmware
Copying file from "./build/8051/release/firmware.bin" to "../project-a/artifacts/blobs/firmware-8051.blob"
```

## Usage
```
Copy files based on artifacts.yml configuration

Usage: outsider [OPTIONS] <YAML_FILE>

Arguments:
  <YAML_FILE>  Path to the artifacts.yml file

Options:
  -p, --project <PROJECT>        Only process projects whose name contains this string
  -s, --source-dir <SOURCE_DIR>  Source directory [default: .]
  -h, --help                     Print help
  -V, --version                  Print version
```
