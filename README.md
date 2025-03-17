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
