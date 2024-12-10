<div align="center">
  <img src="https://emoji2svg.deno.dev/api/🩻" height="160px">
  <h1>DCMfx Playground</h1>
  <p>
    <strong>
      Browser app that exposes some of DCMfx's DICOM and
      <br>
      DICOM JSON functionality
    </strong>
    <br />
  </p>

  [<img src="https://img.shields.io/badge/License-AGPLv3-blue.svg">](https://www.gnu.org/licenses/agpl-3.0.en.html)
  [<img src="https://img.shields.io/badge/MSRV-1.80-CE422B">](https://www.rust-lang.org)
</div>

## Try It Out 🛝

To open DCMfx Playground [click here](https://dcmfx.github.io/dcmfx-playground). Opened DICOM files
never leave your device.

## Features ✨

1. View the content of DICOM and DICOM JSON files.

2. Convert between DICOM files and DICOM JSON files.

3. Rewrite DICOM files to ensure they have correct headers, change their string encoding to UTF-8,
   and correct errors that may prevent use in other software.

## Development

DCMfx Playground is written in [Dioxus](https://dioxuslabs.com). Install the Dioxus CLI with
`cargo install dioxus-cli`, then run `dx serve` to start the dev server.

## License 📋

DCMfx is published under the GNU Affero General Public License Version 3 (AGPLv3). This license
permits commercial use; however, any software that incorporates DCMfx, either directly or
indirectly, must also be released under the AGPLv3 or a compatible license. This includes making the
source code of the combined work available under the same terms, and ensuring that users who
interact with the software over a network can access the source code.

Copyright © Dr Richard Viney, 2024-2025.
