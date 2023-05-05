<h3 align="center">
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
	Catppuccin Lookup Tables (Hald-CLUT)
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

> Generated using `noise_2` variant

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
<details>
<summary>⚫ Oled</summary>
<img src="examples/oled.png"/>
</details>

## Usage

Images:

```bash
magick input.png src/noise_2/mocha.png -hald-clut output.png
```

Videos:

```bash
ffmpeg -i input.mkv -i src/noise_2/mocha.png -filter_complex '[0][1] haldclut' output.mp4
```

### Apply script

The script is an easy way to use the LUTs on an image. It accepts any number of images and some flavors to use.

```bash
./apply.sh -i image1.png -i image2.png -f "oled mocha macchiato frappe latte" 
```

The script can also be easily "installed" by symlinking it to `.local/bin`

```bash
ln -s `realpath apply.sh` ~/.local/bin/ctpify
```

### (Re)generating LUTs

Requirements: [imagemagick](https://imagemagick.org)

```bash
./generate.sh lut.png 2 color1 color2 ...
```

## 💝 Thanks to

- Gingeh for pioneering the process :)

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
