#!/bin/sh
set -eux

# This script is written to be as POSIX as possible
# so it works fine for all Unix-like operating systems

test_cmd() {
  command -v "$1" >/dev/null
}

# proxy version
velocirust_new_ver="${1}"
# proxy directory
# eval to resolve '~' into proper user dir
eval velocirust_dir="'${2}'"

case "${velocirust_new_ver}" in
  v*)
    velocirust_new_version=$(echo "${velocirust_new_ver}" | cut -d'v' -f2)
    velocirust_new_ver_tag="${velocirust_new_ver}"
  ;;
  nightly*)
    velocirust_new_version="${velocirust_new_ver}"
    velocirust_new_ver_tag=$(echo ${velocirust_new_ver} | cut -d '-' -f1)
  ;;
  *)
    printf 'Unknown version\n'
    exit 1
  ;;
esac

if [ -e "${velocirust_dir}/velocirust" ]; then
  velocirust_installed_ver=$("${velocirust_dir}/velocirust" --version | cut -d' ' -f2)

  printf '[DEBUG]: Current proxy version: %s\n' "${velocirust_installed_ver}"
  printf '[DEBUG]: New proxy version: %s\n' "${velocirust_new_version}"
  if [ "${velocirust_installed_ver}" = "${velocirust_new_version}" ]; then
    printf 'Proxy already exists\n'
    exit 0
  else
    printf 'Proxy outdated. Replacing proxy\n'
    rm "${velocirust_dir}/velocirust"
  fi
fi

for _cmd in tar gzip uname; do
  if ! test_cmd "${_cmd}"; then
    printf 'Missing required command: %s\n' "${_cmd}"
    exit 1
  fi
done

# Currently only linux/darwin are supported
case $(uname -s) in
  Linux) os_name=linux ;;
  Darwin) os_name=darwin ;;
  *)
    printf '[ERROR] unsupported os\n'
    exit 1
  ;;
esac

# Currently only amd64/arm64 are supported
case $(uname -m) in
  x86_64|amd64|x64) arch_name=x86_64 ;;
  arm64|aarch64) arch_name=aarch64 ;;
  # riscv64) arch_name=riscv64 ;;
  *)
    printf '[ERROR] unsupported arch\n'
    exit 1
  ;;
esac

velocirust_download_url="https://github.com/velocirust/velocirust/releases/download/${velocirust_new_ver_tag}/velocirust-proxy-${os_name}-${arch_name}.gz"

printf 'Creating "%s"\n' "${velocirust_dir}"
mkdir -p "${velocirust_dir}"
cd "${velocirust_dir}"

if test_cmd 'curl'; then
  # How old curl has these options? we'll find out
  printf 'Downloading using curl\n'
  curl --proto '=https' --tlsv1.2 -LfS -O "${velocirust_download_url}"
  # curl --proto '=https' --tlsv1.2 -LZfS -o "${tmp_dir}/velocirust-proxy-${os_name}-${arch_name}.gz" "${velocirust_download_url}"
elif test_cmd 'wget'; then
  printf 'Downloading using wget\n'
  wget "${velocirust_download_url}"
else
  printf 'curl/wget not found, failed to download proxy\n'
  exit 1
fi

printf 'Decompressing gzip\n'
gzip -df "${velocirust_dir}/velocirust-proxy-${os_name}-${arch_name}.gz"

printf 'Renaming proxy \n'
mv -v "${velocirust_dir}/velocirust-proxy-${os_name}-${arch_name}" "${velocirust_dir}/velocirust"

printf 'Making it executable\n'
chmod +x "${velocirust_dir}/velocirust"

printf 'velocirust-proxy installed\n'

exit 0
