# rcz

A tool to write a commit message based on [Conventional Commits](https://www.conventionalcommits.org/)  
inspired by [git-cz](https://github.com/streamich/git-cz)

![GitHub CI](https://github.com/Cassin01/rcz/actions/workflows/rust_ci.yml/badge.svg)

## Installation

```sh
cargo install rcz
```

## Example

```zsh
# bash
function gitz() {
  local output
  if output=$(rcz); then
    git commit -m "${output}"
  else
    echo "Err: failed to generate a commit message"
  fi
}
```
or

`.gitconfig`
```.gitconfig
[alias]
  cz = "!gitz() { if local output= ...}; gitz"

```

<details>
<summary>gif</summary>

![gif](https://github.com/Cassin01/rcz/blob/34789fd4c59983201745cbe3c760252e23320d69/asset/example-gitz.gif)

</details>

## Configuration

The configuration file will be automatically generated on:

- Linux: `~/.config/rcz`
- Windows: `{FOLDERID_RoamingAppData}\rcz`
- Mac OS: `~/Library/Preferences/rs.rcz`


<details>
<summary>Default configuration</summary>

```toml
format = '''
{type}: {subject}'''

[[types]]
description = 'A bug fix'
value = 'fix'
emoji = '๐'

[[types]]
description = 'A new feature'
value = 'feat'
emoji = 'โจ'

[[types]]
description = 'Changes that introduces a breaking API change'
value = 'BREAKING CHANGE'
emoji = '๐ฅ'

[[types]]
description = 'build system or external dependencies'
value = 'chore'
emoji = '๐ ๏ธ'

[[types]]
description = 'CI related changes'
value = 'ci'
emoji = '๐ซ'

[[types]]
description = 'Documentation only changes'
value = 'docs'
emoji = 'โ๏ธ'

[[types]]
description = 'Changes that do not affect the meaning of the code'
value = 'style'
emoji = '๐'

[[types]]
description = 'A code change that neither fixes a bug nor adds a feature'
value = 'refactor'
emoji = '๐งน'

[[types]]
description = ' A code change that improves performance'
value = 'perf'
emoji = '๐'

[[types]]
description = 'Adding or correcting tests'
value = 'test'
emoji = '๐งช'
```
</details>

### Format

All section etc `{scope}` that you can add on the format are bellow.

- `{type}`
- `{emoji}`
- `{description}`

:warning: The string that enclosed in double brackets (`{{echo 'foo'}}`, `{{date}}` etc) is interpreted as shell script.

Other strings (`{body}`, `{footer}`, `{header}` etc) is interpreted as a custom input.  
The string is used as a prompt message.

A example for your customize is bellow.

```toml
format = '''
{type}{scope}: {emoji}{subject}
{body}
{footer}'''
```
