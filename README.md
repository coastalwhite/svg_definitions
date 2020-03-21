Hello fellow Rustacians! Here is a crate with SVG Definitions.
This was mostly created to serve as a backend crate for [wasm_svg_graphics](https://crates.io/crates/wasm_svg_graphics),
but feel free to use it!

I am open to pull requests so please contribute!

# Example

## Creating a group with a triangle

```
use svg_definitions::prelude::*;

let triangle = SVGElem::new(Tag::Path)
    .set(Attr::StrokeWidth, 1)
    .set(Attr::Stroke, "#000")
    .set(Attr::Fill, "transparent")
    .set(Attr::D, PathData::new()
        .move_to((0.0, 0.0))
        .line_to((10.0, 0.0))
        .line_to((0.0, 10.0))
        .line_to((0.0, 0.0))
        .close_path()
    );

let group = SVGElem::new(Tag::G)
    .append(triangle);
```
