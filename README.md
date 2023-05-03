<h3 align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
	Catppuccin for ImageMagick (Lookup Tables)
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
	<a href="https://github.com/ozwaldorf/catppuccin-luts/stargazers"><img src="https://img.shields.io/github/stars/ozwaldorf/catppuccin-luts?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
	<a href="https://github.com/ozwaldorf/catppuccin-luts/issues"><img src="https://img.shields.io/github/issues/ozwaldorf/catppuccin-luts?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
	<a href="https://github.com/ozwaldorf/catppuccin-luts/contributors"><img src="https://img.shields.io/github/contributors/ozwaldorf/catppuccin-luts?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
	<img src="examples/preview.png"/>
</p>

## Previews

<details>
<summary>🌻 Latte</summary>
<img src="examples/latte.png"/>
</details>
<details>
<summary>🪴 Frappé</summary>
<img src="examples/frappe.png"/>
</details>
<details>
<summary>🌺 Macchiato</summary>
<img src="examples/macchiato.png"/>
</details>
<details>
<summary>🌿 Mocha</summary>
<img src="examples/mocha.png"/>
</details>

## Generating LUTs

Requirements: `cargo-play`, `imagemagick`

```bash
cargo play generate.rs
```

## Usage

Images:

```bash
magick input.png luts/mocha.png -hald-clut output.png
```

Videos:

```bash
ffmpeg -i input.mkv -i luts/mocha.png -filter_complex '[0][1] haldclut' output.mp4
```

&nbsp;

<p align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" />
</p>

<p align="center">
	Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
</p>

<p align="center">
	<a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>
