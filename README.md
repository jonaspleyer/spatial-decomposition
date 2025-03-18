[![Apache License](https://img.shields.io/github/license/jonaspleyer/spatial-decomposition?style=flat-square)](https://opensource.org/license/apache-2-0)
[![Test](https://img.shields.io/github/actions/workflow/status/jonaspleyer/spatial-decomposition/test.yml?label=Test&style=flat-square)](https://github.com/jonaspleyer/spatial-decomposition/actions)
[![Crate](https://img.shields.io/crates/v/spatial-decomposition.svg?style=flat-square)](https://crates.io/crates/spatial-decomposition)
![Crates.io Total Downloads](https://img.shields.io/crates/d/spatial-decomposition?style=flat-square)
![docs.rs](https://img.shields.io/docsrs/spatial-decomposition?style=flat-square)

# spatial_decomposition

This crate implements algorithms to decompose spaces into subdomains.
The created partitions can be reused within numerical solvers such as FETI (finite element tearing
and interconnect) or other methods.

# Kong-Mount-Roscoe (KMR) Decomposition
This algorithms divides a given rectangle into multiple smaller rectangles and minimizes the
maximum rectangle perimeter.
## Square

<table style="width: 60%">
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square2.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square3.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square4.png"></td>
    </tr>
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square5.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square6.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square7.png"></td>
    </tr>
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square8.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square9.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/square10.png"></td>
    </tr>
</table>

## Wide Rectangle

<table style="width: 60%">
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/wide_rect2.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/wide_rect3.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/wide_rect4.png"></td>
    </tr>
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/wide_rect5.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/wide_rect6.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/wide_rect7.png"></td>
    </tr>
</table>

## Rectangle

<table style="width: 60%">
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/other_rect2.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/other_rect3.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/other_rect4.png"></td>
    </tr>
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/other_rect5.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/other_rect6.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/other_rect7.png"></td>
    </tr>
</table>

# Kong-Mount-Roscoe (KMR) Digitization

## Wide Rectangle

<table style="width: 60%">
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/rectangle_1_in_4.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/rectangle_2_in_4.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/rectangle_3_in_4.png"></td>
    </tr>
</table>


## Rectangle

<table style="width: 60%">
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/rectangle_4_in_4.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/rectangle_5_in_5.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/rectangle_6_in_6.png"></td>
    </tr>
</table>

## Square
<table style="width: 60%">
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_3x3_in_2.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_3x3_in_3.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_4x4_in_3.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_4x4_in_4.png"></td>
    </tr>
    <tr>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_9x9_in_3.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_9x9_in_4.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_9x9_in_5.png"></td>
        <td><img style="width: 100%;" src="https://raw.githubusercontent.com/jonaspleyer/spatial-decomposition/refs/heads/main/plots/digitize_kmr_1/square_9x9_in_6.png"></td>
    </tr>
</table>

## KMR Benchmarks

Measurements were performed one a AMD Ryzen 3700X with fixed CPU frequency at 2000MHz.

### Decomposition Square
| `N` Subdomains | Time | lower | upper |
| --- | --- | --- | --- |
| 10 |  1.0574 µs | 1.0509 µs | 1.0663 µs |
| 20 |  2.8923 µs | 2.8911 µs | 2.8930 µs |
| 30 |  6.1582 µs | 6.1524 µs | 6.1612 µs |
| 40 |  10.798 µs | 10.764 µs | 10.819 µs |
| 50 |  16.888 µs | 16.832 µs | 16.950 µs |
| 60 |  23.779 µs | 23.763 µs | 23.788 µs |
| 70 |  32.187 µs | 32.135 µs | 32.311 µs |
| 80 |  41.466 µs | 41.361 µs | 41.553 µs |
| 90 |  52.375 µs | 52.292 µs | 52.486 µs |
| 100 | 64.349 µs | 63.931 µs | 64.866 µs |
| 110 | 78.309 µs | 77.629 µs | 79.082 µs |
| 120 | 92.232 µs | 91.824 µs | 92.709 µs |
| 130 | 107.96 µs | 107.69 µs | 108.19 µs |
| 140 | 123.95 µs | 123.81 µs | 124.19 µs |
| 150 | 142.52 µs | 141.99 µs | 143.23 µs |
| 160 | 161.40 µs | 161.27 µs | 161.62 µs |
| 170 | 182.68 µs | 182.04 µs | 182.99 µs |
| 180 | 207.25 µs | 204.32 µs | 209.21 µs |
| 190 | 229.12 µs | 228.86 µs | 229.41 µs |

### Digitization Square
We fix the number of subdomains to `n_subdomains=20`.

| Method | `N` Digits | Time | lower | upper |
| --- | --- | --- | --- | --- |
| Serial | 1000 | 6.3998 ms | 6.3817 ms | 6.4392 ms |
| | 2000 | 25.073 ms | 25.032 ms | 25.106 ms |
| | 3000 | 56.227 ms | 56.015 ms | 56.503 ms |
| | 4000 | 99.222 ms | 99.125 ms | 99.416 ms |
| Parallel | 1000 | 6.2554 ms | 6.2311 ms | 6.2744 ms |
| | 2000 | 24.951 ms | 24.843 ms | 25.067 ms |
| | 3000 | 55.823 ms | 55.620 ms | 56.107 ms |
| | 3000 | 99.835 ms | 99.150 ms | 100.73 ms |
