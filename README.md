Hello fellow Rustacians! Here is a crate with SVG Definitions.
This was mostly created to serve as a backend crate for [wasm_svg_graphics](https://crates.io/crates/wasm_svg_graphics),
but feel free to use it!

I am open to pull requests so please contribute!

# Example

## Creating a group with a triangle

```
use svg_definitions::prelude::*;

let triangle = SVGElem::new(Tag::SVGPath)
    .set(Attr::StrokeWidth, 1.into())
    .set(Attr::StrokeColor, RGB::new(0,0,0).into())
    .set(Attr::FillColor, RGBT::Transparent.into())
    .set(Attr::PathDefinition, PathString::new()
        .move_to((0.0, 0.0))
        .line_to((10.0, 0.0))
        .line_to((0.0, 10.0))
        .line_to((0.0, 0.0))
        .close_path()
        .into()
    );

let group = SVGElem::new(Tag::Group)
    .append(triangle);
```
