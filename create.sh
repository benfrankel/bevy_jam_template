#!/bin/bash

set -u
shopt -s globstar
shopt -s dotglob

declare -ar TEMPLATE_ONLY=(
    '.git'
    'build'
    'create.sh'
    'Cargo.lock'
    'LICENSE-0BSD'
    'LICENSE-CC0-1.0'
    'README.md'
)

function usage() {
    echo "Usage: $0 <NEW_PROJECT_PATH>"
}

function main() {
    [ "$#" -eq 1 ] || usage

    local -r template_path="$(dirname "$0")"
    local -r template_name="$(basename "${template_path}")"
    local -r template_itch="$(echo "${template_name}" | tr '_' '-')"
    local -r package_path="$1"
    local -r package_name="$(basename "${package_path}")"
    local -r package_itch="$(echo "${package_name}" | tr '_' '-')"

    # Let `cargo init` validate the package name for us :)
    cargo --quiet init "${package_path}" || exit 1

    # Copy template contents
    cp -r "${template_path}" "${package_path}"
    cd "${package_path}" || exit 1
    for item in "${TEMPLATE_ONLY[@]}"; do
        rm -rf "${template_path}/${item:?}"
    done

    # Substitute template name for package name
    find "${template_path}" -type f -print0 |\
        xargs -0 sed -i "s/${template_name}/${package_name}/g"
    find "${template_path}" -type f -print0 |\
        xargs -0 sed -i "s/${template_itch}/${package_itch}/g"

    # Move template contents into package
    rm -rf src
    mv "${template_path}"/* .
    rmdir "${template_path}"
    mkdir build

    # Make initial commit
    git add .
    git commit --message 'Initial commit' >/dev/null
}

main "$@"

exit 0
