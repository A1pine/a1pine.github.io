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

build_web_dir="${project_root}/target/dx/arcademic-rust/${profile_dir}/web"
source_dir="${build_web_dir}/public"
output_dir="${project_root}/dist/public"

cd "${project_root}"
rm -rf -- "${build_web_dir}"
"${dioxus_cli}" "${build_args[@]}"

test -s "${source_dir}/index.html"

rm -rf -- "${output_dir}"
mkdir -p "${output_dir}"
cp -R "${source_dir}/." "${output_dir}/"
cargo run --locked --quiet --bin finalize-pages -- \
  "${output_dir}/index.html" "${base_path}" "${output_dir}/404.html"
touch "${output_dir}/.nojekyll"
if [[ -n "${PAGES_CNAME:-}" ]]; then
  printf '%s\n' "${PAGES_CNAME}" > "${output_dir}/CNAME"
fi

echo "Pages artifact: ${output_dir}"
