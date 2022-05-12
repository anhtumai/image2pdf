# Image2pdf

Inspired from [Img2pdf project](https://github.com/josch/img2pdf).

Features:

- Support file formats: BMP, PNG, JPG
- Support color type: L8, La8, Rgb8, Rgba8, Bgr8, Bgra8
- For image with alpha channel, automatically apply white background

## Usage

### 1. Without pagesize option

```bash
$> image2pdf assets/* --output output.pdf
```

Output: [Pdf file](./assets/outputs/without_pagesize.pdf)

### 2. With pagesize option

```bash
$> image2pdf assets/* --output output.pdf --pagesize A4
```

Output: [Pdf file](./assets/outputs/with_pagesize_A4.pdf)

### 3. With pagesize A4 landscape option

```bash
$> image2pdf assets/* --output output.pdf --pagesize A4^T
```

Output: [Pdf file](./assets/outputs/with_pagesize_A4_landscape.pdf)

### 4. With custom pagesize

```bash
$> image2pdf assets/* --output output.pdf --pagesize 200mmx200mm
```

Output: [Pdf file](./assets/outputs/with_custom_pagesize.pdf)
