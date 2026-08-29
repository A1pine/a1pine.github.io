#!/usr/bin/env bash
set -euo pipefail

project_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
base_path=${1:-${PAGES_BASE_PATH:-}}
profile=${2:-release}
dioxus_cli=${DIOXUS_CLI:-${CARGO_HOME:-${HOME}/.cargo}/bin/dx}

if [[ ! -x "${dioxus_cli}" ]]; then
  dioxus_cli=$(command -v dx)
fi

case "${profile}" in
  release)
    profile_flag=(--release)
    profile_dir=release
    ;;
  debug)
    profile_flag=()
    profile_dir=debug
    ;;
  *)
    echo "profile must be 'release' or 'debug'" >&2
    exit 2
    ;;
esac

build_args=(
  build
  --web
  --ssg
  --fullstack true
  --force-sequential true
  --debug-symbols false
  --locked
  "${profile_flag[@]}"
)

if [[ -n "${base_path}" ]]; then
  build_args+=(--base-path "${base_path}")
fi

cd "${project_root}"
"${dioxus_cli}" "${build_args[@]}"

source_dir="${project_root}/target/dx/arcademic-rust/${profile_dir}/web/public"
output_dir="${project_root}/dist/public"

test -s "${source_dir}/index.html"

rm -rf -- "${output_dir}"
mkdir -p "${output_dir}"
cp -R "${source_dir}/." "${output_dir}/"
cargo run --locked --quiet --bin finalize-pages -- \
  "${output_dir}/index.html" "${base_path}"
touch "${output_dir}/.nojekyll"

echo "Pages artifact: ${output_dir}"
