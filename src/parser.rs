//! Parser module, enabled with "parsing" feature
//!
//! This module provides the possibility to parse svg files.
//!
//! # Examples
//! ## Getting a svg from a file
//! *The feature "parsing" needs to be enabled for this*
//! ```
//! use svg_definitions::prelude::*;
//!
//! let shape = SVGParseFile("/path/to/file.svg");
//!
//! // ...
//! ```
//!
//! ## Getting a svg from text
//! *The feature "parsing" needs to be enabled for this*
//! ```
//! use svg_definitions::prelude::*;
//!
//! let rect = SVGParseText("<rect width=\"50px\" height=\"50\" fill=\"black\" />");
//!
//! // ...
//! ```

/// The error enum used when parsing
#[derive(Debug)]
pub enum ParseError {
    RoxmltreeError(roxmltree::Error),
    TagNotFound(String),
    NoElement,
    FileError(std::io::Error),
}

fn string_to_tag(string: &str) -> Option<crate::tag_name::TagName> {
    use crate::tag_name::TagName::*;

    let string = string.to_lowercase();

    match &string[..] {
        "a" => Some(A),
        "animate" => Some(Animate),
        "animateMotion" => Some(AnimateMotion),
        "animateTransform" => Some(AnimateTransform),
        "circle" => Some(Circle),
        "clipPath" => Some(ClipPath),
        "color-profile" => Some(ColorProfile),
        "defs" => Some(Defs),
        "desc" => Some(Desc),
        "discard" => Some(Discard),
        "ellipse" => Some(Ellipse),
        "feBlend" => Some(FeBlend),
        "feColorMatrix" => Some(FeColorMatrix),
        "feComponentTransfer" => Some(FeComponentTransfer),
        "feComposite" => Some(FeComposite),
        "feConvolveMatrix" => Some(FeConvolveMatrix),
        "feDiffuseLighting" => Some(FeDiffuseLighting),
        "feDisplacementMap" => Some(FeDisplacementMap),
        "feDistantLight" => Some(FeDistantLight),
        "feDropShadow" => Some(FeDropShadow),
        "feFlood" => Some(FeFlood),
        "feFuncA" => Some(FeFuncA),
        "feFuncB" => Some(FeFuncB),
        "feFuncG" => Some(FeFuncG),
        "feFuncR" => Some(FeFuncR),
        "feGaussianBlur" => Some(FeGaussianBlur),
        "feImage" => Some(FeImage),
        "feMerge" => Some(FeMerge),
        "feMergeNode" => Some(FeMergeNode),
        "feMorphology" => Some(FeMorphology),
        "feOffset" => Some(FeOffset),
        "fePointLight" => Some(FePointLight),
        "feSpecularLighting" => Some(FeSpecularLighting),
        "feSpotLight" => Some(FeSpotLight),
        "feTile" => Some(FeTile),
        "feTurbulence" => Some(FeTurbulence),
        "filter" => Some(Filter),
        "foreignObject" => Some(ForeignObject),
        "g" => Some(G),
        "hatch" => Some(Hatch),
        "hatchpath" => Some(Hatchpath),
        "image" => Some(Image),
        "line" => Some(Line),
        "linearGradient" => Some(LinearGradient),
        "marker" => Some(Marker),
        "mask" => Some(Mask),
        "mesh" => Some(Mesh),
        "meshgradient" => Some(Meshgradient),
        "meshpatch" => Some(Meshpatch),
        "meshrow" => Some(Meshrow),
        "metadata" => Some(Metadata),
        "mpath" => Some(Mpath),
        "path" => Some(Path),
        "pattern" => Some(Pattern),
        "polygon" => Some(Polygon),
        "polyline" => Some(Polyline),
        "radialGradient" => Some(RadialGradient),
        "rect" => Some(Rect),
        "script" => Some(Script),
        "set" => Some(Set),
        "solidcolor" => Some(Solidcolor),
        "stop" => Some(Stop),
        "style" => Some(Style),
        "svg" => Some(Svg),
        "switch" => Some(Switch),
        "symbol" => Some(Symbol),
        "text" => Some(Text),
        "textPath" => Some(TextPath),
        "title" => Some(Title),
        "tspan" => Some(Tspan),
        "unknown" => Some(Unknown),
        "use" => Some(Use),
        "view" => Some(View),
        _ => None,
    }
}
fn string_to_attribute(string: &str) -> crate::attributes::Attribute {
    use crate::attributes::Attribute::*;

    match &string[..] {
        "accent-height" => AccentHeight,
        "accumulate" => Accumulate,
        "additive" => Additive,
        "alignment-baseline" => AlignmentBaseline,
        "allowReorder" => AllowReorder,
        "alphabetic" => Alphabetic,
        "amplitude" => Amplitude,
        "arabic-form" => ArabicForm,
        "ascent" => Ascent,
        "attributeName" => AttributeName,
        "attributeType" => AttributeType,
        "autoReverse" => AutoReverse,
        "azimuth" => Azimuth,
        "baseFrequency" => BaseFrequency,
        "baseline-shift" => BaselineShift,
        "baseProfile" => BaseProfile,
        "bbox" => Bbox,
        "begin" => Begin,
        "bias" => Bias,
        "by" => By,
        "calcMode" => CalcMode,
        "cap-height" => CapHeight,
        "class" => Class,
        "clip" => Clip,
        "clipPathUnits" => ClipPathUnits,
        "clip-path" => ClipPath,
        "clip-rule" => ClipRule,
        "color" => Color,
        "color-interpolation" => ColorInterpolation,
        "color-interpolation-filters" => ColorInterpolationfilters,
        "color-profile" => ColorProfile,
        "color-rendering" => ColorRendering,
        "contentScriptType" => ContentScriptType,
        "contentStyleType" => ContentStyleType,
        "cursor" => Cursor,
        "cx" => Cx,
        "cy" => Cy,
        "d" => D,
        "decelerate" => Decelerate,
        "descent" => Descent,
        "diffuseConstant" => DiffuseConstant,
        "direction" => Direction,
        "display" => Display,
        "divisor" => Divisor,
        "dominant-baseline" => DominantBaseline,
        "dur" => Dur,
        "dx" => Dx,
        "dy" => Dy,
        "edgeMode" => EdgeMode,
        "elevation" => Elevation,
        "enable-background" => EnableBackground,
        "end" => End,
        "exponent" => Exponent,
        "externalResourcesRequired" => ExternalResourcesRequired,
        "fill" => Fill,
        "fill-opacity" => FillOpacity,
        "fill-rule" => FillRule,
        "filter" => Filter,
        "filterRes" => FilterRes,
        "filterUnits" => FilterUnits,
        "flood-color" => FloodColor,
        "flood-opacity" => FloodOpacity,
        "font-family" => FontFamily,
        "font-size" => FontSize,
        "font-size-adjust" => FontSizeadjust,
        "font-stretch" => FontStretch,
        "font-style" => FontStyle,
        "font-variant" => FontVariant,
        "font-weight" => FontWeight,
        "format" => Format,
        "from" => From,
        "fr" => Fr,
        "fx" => Fx,
        "fy" => Fy,
        "g1" => G1,
        "g2" => G2,
        "glyph-name" => GlyphName,
        "glyph-orientation-horizontal" => GlyphOrientationhorizontal,
        "glyph-orientation-vertical" => GlyphOrientationvertical,
        "glyphRef" => GlyphRef,
        "gradientTransform" => GradientTransform,
        "gradientUnits" => GradientUnits,
        "hanging" => Hanging,
        "height" => Height,
        "href" => Href,
        "hreflang" => Hreflang,
        "horiz-adv-x" => HorizAdvx,
        "horiz-origin-x" => HorizOriginx,
        "id" => Id,
        "ideographic" => Ideographic,
        "image-rendering" => ImageRendering,
        "in" => In,
        "in2" => In2,
        "intercept" => Intercept,
        "k" => K,
        "k1" => K1,
        "k2" => K2,
        "k3" => K3,
        "k4" => K4,
        "kernelMatrix" => KernelMatrix,
        "kernelUnitLength" => KernelUnitLength,
        "kerning" => Kerning,
        "keyPoints" => KeyPoints,
        "keySplines" => KeySplines,
        "keyTimes" => KeyTimes,
        "lang" => Lang,
        "lengthAdjust" => LengthAdjust,
        "letter-spacing" => LetterSpacing,
        "lighting-color" => LightingColor,
        "limitingConeAngle" => LimitingConeAngle,
        "local" => Local,
        "marker-end" => MarkerEnd,
        "marker-mid" => MarkerMid,
        "marker-start" => MarkerStart,
        "markerHeight" => MarkerHeight,
        "markerUnits" => MarkerUnits,
        "markerWidth" => MarkerWidth,
        "mask" => Mask,
        "maskContentUnits" => MaskContentUnits,
        "maskUnits" => MaskUnits,
        "mathematical" => Mathematical,
        "max" => Max,
        "media" => Media,
        "method" => Method,
        "min" => Min,
        "mode" => Mode,
        "name" => Name,
        "numOctaves" => NumOctaves,
        "offset" => Offset,
        "opacity" => Opacity,
        "operator" => Operator,
        "order" => Order,
        "orient" => Orient,
        "orientation" => Orientation,
        "origin" => Origin,
        "overflow" => Overflow,
        "overline-position" => OverlinePosition,
        "overline-thickness" => OverlineThickness,
        "panose-1" => Panose1,
        "paint-order" => PaintOrder,
        "path" => Path,
        "pathLength" => PathLength,
        "patternContentUnits" => PatternContentUnits,
        "patternTransform" => PatternTransform,
        "patternUnits" => PatternUnits,
        "ping" => Ping,
        "pointer-events" => PointerEvents,
        "points" => Points,
        "pointsAtX" => PointsAtX,
        "pointsAtY" => PointsAtY,
        "pointsAtZ" => PointsAtZ,
        "preserveAlpha" => PreserveAlpha,
        "preserveAspectRatio" => PreserveAspectRatio,
        "primitiveUnits" => PrimitiveUnits,
        "r" => R,
        "radius" => Radius,
        "referrerPolicy" => ReferrerPolicy,
        "refX" => RefX,
        "refY" => RefY,
        "rel" => Rel,
        "rendering-intent" => RenderingIntent,
        "repeatCount" => RepeatCount,
        "repeatDur" => RepeatDur,
        "requiredExtensions" => RequiredExtensions,
        "requiredFeatures" => RequiredFeatures,
        "restart" => Restart,
        "result" => Result,
        "rotate" => Rotate,
        "rx" => Rx,
        "ry" => Ry,
        "slope" => Slope,
        "spacing" => Spacing,
        "specularConstant" => SpecularConstant,
        "specularExponent" => SpecularExponent,
        "speed" => Speed,
        "spreadMethod" => SpreadMethod,
        "startOffset" => StartOffset,
        "stdDeviation" => StdDeviation,
        "stemh" => Stemh,
        "stemv" => Stemv,
        "stitchTiles" => StitchTiles,
        "stop-color" => StopColor,
        "stop-opacity" => StopOpacity,
        "strikethrough-position" => StrikethroughPosition,
        "strikethrough-thickness" => StrikethroughThickness,
        "string" => String,
        "stroke" => Stroke,
        "stroke-dasharray" => StrokeDasharray,
        "stroke-dashoffset" => StrokeDashoffset,
        "stroke-linecap" => StrokeLinecap,
        "stroke-linejoin" => StrokeLinejoin,
        "stroke-miterlimit" => StrokeMiterlimit,
        "stroke-opacity" => StrokeOpacity,
        "stroke-width" => StrokeWidth,
        "style" => Style,
        "surfaceScale" => SurfaceScale,
        "systemLanguage" => SystemLanguage,
        "tabindex" => Tabindex,
        "tableValues" => TableValues,
        "target" => Target,
        "targetX" => TargetX,
        "targetY" => TargetY,
        "text-anchor" => TextAnchor,
        "text-decoration" => TextDecoration,
        "text-rendering" => TextRendering,
        "textLength" => TextLength,
        "to" => To,
        "transform" => Transform,
        "type" => Type,
        "u1" => U1,
        "u2" => U2,
        "underline-position" => UnderlinePosition,
        "underline-thickness" => UnderlineThickness,
        "unicode" => Unicode,
        "unicode-bidi" => UnicodeBidi,
        "unicode-range" => UnicodeRange,
        "units-per-em" => UnitsPerem,
        "v-alphabetic" => VAlphabetic,
        "v-hanging" => VHanging,
        "v-ideographic" => VIdeographic,
        "v-mathematical" => VMathematical,
        "values" => Values,
        "vector-effect" => VectorEffect,
        "version" => Version,
        "vert-adv-y" => VertAdvy,
        "vert-origin-x" => VertOriginx,
        "vert-origin-y" => VertOriginy,
        "viewBox" => ViewBox,
        "viewTarget" => ViewTarget,
        "visibility" => Visibility,
        "width" => Width,
        "widths" => Widths,
        "word-spacing" => WordSpacing,
        "writing-mode" => WritingMode,
        "x" => X,
        "x-height" => XHeight,
        "x1" => X1,
        "x2" => X2,
        "xChannelSelector" => XChannelSelector,
        "xlink:actuate" => XlinkActuate,
        "xlink:arcrole" => XlinkArcrole,
        "xlink:href" => XlinkHref,
        "xlink:role" => XlinkRole,
        "xlink:show" => XlinkShow,
        "xlink:title" => XlinkTitle,
        "xlink:type" => XlinkType,
        "xml:base" => XmlBase,
        "xml:lang" => XmlLang,
        "xml:space" => XmlSpace,
        "y" => Y,
        "y1" => Y1,
        "y2" => Y2,
        "yChannelSelector" => YChannelSelector,
        "z" => Z,
        "zoomAndPan" => ZoomAndPan,
        attr => UnmappedAttribute(std::string::String::from(attr)),
    }
}

fn node_to_element(root: roxmltree::Node) -> Result<Option<crate::Element>, ParseError> {
    if !root.is_element() {
        return Ok(None);
    }

    let mut inner = String::from("");

    let tag = root.tag_name().name();
    let mut element: crate::Element =
        crate::Element::new(string_to_tag(tag).ok_or(ParseError::TagNotFound(String::from(tag)))?);
    for attribute in root.attributes().iter() {
        element = element.set(string_to_attribute(attribute.name()), attribute.value());
    }

    for child in root.children() {
        if child.is_text() {
            inner = format!("{}{}", inner, child.text().unwrap());
        }

        let child_element = node_to_element(child)?;

        match child_element {
            Some(child_element) => {
                element = element.append(child_element);
            }
            None => (),
        };
    }

    if inner != "" {
        element = element.set_inner(&inner[..]);
    }

    Ok(Some(element))
}

/// Parsing from a pure string
///
/// ## Getting a svg from text
/// *The feature "parsing" needs to be enabled for this*
/// ```
/// use svg_definitions::prelude::*;
///
/// let rect = SVGParseText("<rect width=\"50px\" height=\"50\" fill=\"black\" />");
///
/// // ...
/// ```
pub fn parse_text(xml: &str) -> Result<crate::Element, ParseError> {
    let doc = roxmltree::Document::parse(xml).map_err(|err| ParseError::RoxmltreeError(err))?;
    return node_to_element(doc.root_element())?.ok_or(ParseError::NoElement);
}

/// Parsing from a svg file
///
/// ## Getting a svg from a file
/// *The feature "parsing" needs to be enabled for this*
/// ```
/// use svg_definitions::prelude::*;
///
/// let shape = SVGParseFile("/path/to/file.svg");
///
/// // ...
/// ```
pub fn parse_file(path: &str) -> Result<crate::Element, ParseError> {
    let string = std::fs::read_to_string(path).map_err(|err| ParseError::FileError(err))?;
    return parse_text(&string[..]);
}
