//! This module provides an easy and safe way to interact with attributes of [Elements](../struct.Element.html)
//!
//! # Note
//! In the [crate::prelude](../prelude/index.html) the name for
//! [Attribute](enum.Attribute.html) is [Attr](../prelude/index.html) and for
//! [AttributeValue](enum.AttributeValue.html) is [AttrValue](../prelude/index.html)
//!
//! # Examples
//! ## 1) Setting attributes of a circle
//! ```
//! use svg_definitions::prelude::*;
//!
//! let circle = SVGElem::new(Tag::Circle)
//!     .set(Attr::Radius, 10.0)
//!     .set(Attr::Cx, 15)
//!     .set(Attr::Cy, 15)
//!     .set(Attr::StrokeWidth, 1)
//!     .set(Attr::Stroke, "#000")
//!     .set(Attr::Fill, "transparent");
//! ```
//!
//! ## 2) Setting attributes of a triangle
//! ```
//! use svg_definitions::prelude::*;
//!
//! let triangle = SVGElem::new(Tag::Path)
//!     .set(Attr::StrokeWidth, 1)
//!     .set(Attr::Stroke, "#000")
//!     .set(Attr::Fill, "transparent")
//!     .set(Attr::D, PathData::new()
//!         .move_to((0.0, 0.0))
//!         .line_to((10.0, 0.0))
//!         .line_to((0.0, 10.0))
//!         .line_to((0.0, 0.0))
//!         .close_path()
//!     );
//! ```

use std::clone::Clone;
use std::convert::From;
use std::hash::Hash;

/// An attribute to an Element
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Attribute {
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/accent-height)
    AccentHeight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/accumulate)
    Accumulate,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/additive)
    Additive,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alignment-baseline)
    AlignmentBaseline,

    /// No MDN Documentation available for this attribute
    AllowReorder,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alphabetic)
    Alphabetic,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/amplitude)
    Amplitude,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/arabic-form)
    ArabicForm,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/ascent)
    Ascent,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/attributeName)
    AttributeName,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/attributeType)
    AttributeType,

    /// No MDN Documentation available for this attribute
    AutoReverse,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/azimuth)
    Azimuth,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseFrequency)
    BaseFrequency,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseline-shift)
    BaselineShift,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseProfile)
    BaseProfile,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/bbox)
    Bbox,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/begin)
    Begin,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/bias)
    Bias,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/by)
    By,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/calcMode)
    CalcMode,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cap-height)
    CapHeight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/class)
    Class,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip)
    Clip,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clipPathUnits)
    ClipPathUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip-path)
    ClipPath,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip-rule)
    ClipRule,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color)
    Color,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation)
    ColorInterpolation,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation-filters)
    ColorInterpolationfilters,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-profile)
    ColorProfile,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-rendering)
    ColorRendering,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/contentScriptType)
    ContentScriptType,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/contentStyleType)
    ContentStyleType,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cursor)
    Cursor,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cx)
    Cx,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cy)
    Cy,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d)
    D,

    /// No MDN Documentation available for this attribute
    Decelerate,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/descent)
    Descent,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/diffuseConstant)
    DiffuseConstant,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/direction)
    Direction,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/display)
    Display,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/divisor)
    Divisor,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dominant-baseline)
    DominantBaseline,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dur)
    Dur,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dx)
    Dx,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dy)
    Dy,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/edgeMode)
    EdgeMode,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/elevation)
    Elevation,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/enable-background)
    EnableBackground,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/end)
    End,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/exponent)
    Exponent,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/externalResourcesRequired)
    ExternalResourcesRequired,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill)
    Fill,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-opacity)
    FillOpacity,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule)
    FillRule,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filter)
    Filter,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filterRes)
    FilterRes,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filterUnits)
    FilterUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/flood-color)
    FloodColor,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/flood-opacity)
    FloodOpacity,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-family)
    FontFamily,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-size)
    FontSize,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-size-adjust)
    FontSizeadjust,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-stretch)
    FontStretch,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-style)
    FontStyle,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-variant)
    FontVariant,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-weight)
    FontWeight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/format)
    Format,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/from)
    From,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fr)
    Fr,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fx)
    Fx,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fy)
    Fy,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/g1)
    G1,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/g2)
    G2,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/glyph-name)
    GlyphName,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/glyph-orientation-horizontal)
    GlyphOrientationhorizontal,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/glyph-orientation-vertical)
    GlyphOrientationvertical,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/glyphRef)
    GlyphRef,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/gradientTransform)
    GradientTransform,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/gradientUnits)
    GradientUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/hanging)
    Hanging,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height)
    Height,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/href)
    Href,

    /// No MDN Documentation available for this attribute
    Hreflang,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/horiz-adv-x)
    HorizAdvx,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/horiz-origin-x)
    HorizOriginx,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/id)
    Id,

    /// No MDN Documentation available for this attribute
    Ideographic,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/image-rendering)
    ImageRendering,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/in)
    In,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/in2)
    In2,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/intercept)
    Intercept,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k)
    K,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k1)
    K1,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k2)
    K2,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k3)
    K3,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k4)
    K4,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/kernelMatrix)
    KernelMatrix,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/kernelUnitLength)
    KernelUnitLength,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/kerning)
    Kerning,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/keyPoints)
    KeyPoints,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/keySplines)
    KeySplines,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/keyTimes)
    KeyTimes,

    /// No MDN Documentation available for this attribute
    Lang,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lengthAdjust)
    LengthAdjust,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/letter-spacing)
    LetterSpacing,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lighting-color)
    LightingColor,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/limitingConeAngle)
    LimitingConeAngle,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/local)
    Local,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/marker-end)
    MarkerEnd,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/marker-mid)
    MarkerMid,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/marker-start)
    MarkerStart,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/markerHeight)
    MarkerHeight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/markerUnits)
    MarkerUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/markerWidth)
    MarkerWidth,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/mask)
    Mask,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/maskContentUnits)
    MaskContentUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/maskUnits)
    MaskUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/mathematical)
    Mathematical,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/max)
    Max,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/media)
    Media,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/method)
    Method,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/min)
    Min,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/mode)
    Mode,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/name)
    Name,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/numOctaves)
    NumOctaves,

    /// No MDN Documentation available for this attribute
    Offset,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/opacity)
    Opacity,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/operator)
    Operator,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/order)
    Order,

    /// No MDN Documentation available for this attribute
    Orient,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/orientation)
    Orientation,

    /// No MDN Documentation available for this attribute
    Origin,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/overflow)
    Overflow,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/overline-position)
    OverlinePosition,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/overline-thickness)
    OverlineThickness,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/panose-1)
    Panose1,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/paint-order)
    PaintOrder,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/path)
    Path,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pathLength)
    PathLength,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/patternContentUnits)
    PatternContentUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/patternTransform)
    PatternTransform,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/patternUnits)
    PatternUnits,

    /// No MDN Documentation available for this attribute
    Ping,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointer-events)
    PointerEvents,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/points)
    Points,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointsAtX)
    PointsAtX,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointsAtY)
    PointsAtY,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointsAtZ)
    PointsAtZ,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/preserveAlpha)
    PreserveAlpha,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/preserveAspectRatio)
    PreserveAspectRatio,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/primitiveUnits)
    PrimitiveUnits,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/r)
    R,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/radius)
    Radius,

    /// No MDN Documentation available for this attribute
    ReferrerPolicy,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/refX)
    RefX,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/refY)
    RefY,

    /// No MDN Documentation available for this attribute
    Rel,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/rendering-intent)
    RenderingIntent,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/repeatCount)
    RepeatCount,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/repeatDur)
    RepeatDur,

    /// No MDN Documentation available for this attribute
    RequiredExtensions,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/requiredFeatures)
    RequiredFeatures,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/restart)
    Restart,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/result)
    Result,

    /// No MDN Documentation available for this attribute
    Rotate,

    /// No MDN Documentation available for this attribute
    Slope,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/spacing)
    Spacing,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/specularConstant)
    SpecularConstant,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/specularExponent)
    SpecularExponent,

    /// No MDN Documentation available for this attribute
    Speed,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/spreadMethod)
    SpreadMethod,

    /// No MDN Documentation available for this attribute
    StartOffset,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stdDeviation)
    StdDeviation,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stemh)
    Stemh,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stemv)
    Stemv,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stitchTiles)
    StitchTiles,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stop-color)
    StopColor,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stop-opacity)
    StopOpacity,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/strikethrough-position)
    StrikethroughPosition,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/strikethrough-thickness)
    StrikethroughThickness,

    /// No MDN Documentation available for this attribute
    String,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke)
    Stroke,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-dasharray)
    StrokeDasharray,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-dashoffset)
    StrokeDashoffset,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linecap)
    StrokeLinecap,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linejoin)
    StrokeLinejoin,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-miterlimit)
    StrokeMiterlimit,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-opacity)
    StrokeOpacity,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-width)
    StrokeWidth,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/style)
    Style,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/surfaceScale)
    SurfaceScale,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/systemLanguage)
    SystemLanguage,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/tabindex)
    Tabindex,

    /// No MDN Documentation available for this attribute
    TableValues,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/target)
    Target,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/targetX)
    TargetX,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/targetY)
    TargetY,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-anchor)
    TextAnchor,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-decoration)
    TextDecoration,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-rendering)
    TextRendering,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/textLength)
    TextLength,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/to)
    To,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform)
    Transform,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/type)
    Type,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/u1)
    U1,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/u2)
    U2,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/underline-position)
    UnderlinePosition,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/underline-thickness)
    UnderlineThickness,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/unicode)
    Unicode,

    /// No MDN Documentation available for this attribute
    UnicodeBidi,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/unicode-range)
    UnicodeRange,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/units-per-em)
    UnitsPerem,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/v-alphabetic)
    VAlphabetic,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/v-hanging)
    VHanging,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/v-ideographic)
    VIdeographic,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/v-mathematical)
    VMathematical,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/values)
    Values,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/vector-effect)
    VectorEffect,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/version)
    Version,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/vert-adv-y)
    VertAdvy,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/vert-origin-x)
    VertOriginx,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/vert-origin-y)
    VertOriginy,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox)
    ViewBox,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewTarget)
    ViewTarget,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/visibility)
    Visibility,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/width)
    Width,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/widths)
    Widths,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/word-spacing)
    WordSpacing,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/writing-mode)
    WritingMode,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x)
    X,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x-height)
    XHeight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x1)
    X1,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/x2)
    X2,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xChannelSelector)
    XChannelSelector,

    /// No MDN Documentation available for this attribute
    XlinkActuate,

    /// No MDN Documentation available for this attribute
    XlinkArcrole,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xlink:href)
    XlinkHref,

    /// No MDN Documentation available for this attribute
    XlinkRole,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xlink:show)
    XlinkShow,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xlink:title)
    XlinkTitle,

    /// No MDN Documentation available for this attribute
    XlinkType,

    /// No MDN Documentation available for this attribute
    XmlBase,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xml:lang)
    XmlLang,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xml:space)
    XmlSpace,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/y)
    Y,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/y1)
    Y1,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/y2)
    Y2,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/yChannelSelector)
    YChannelSelector,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/z)
    Z,

    /// No MDN Documentation available for this attribute
    ZoomAndPan,
}

// Implementation of Attribute
impl ToString for Attribute {
    fn to_string(&self) -> String {
        use Attribute::*;

        std::string::String::from(match self {
            AccentHeight => "accent-height",
            Accumulate => "accumulate",
            Additive => "additive",
            AlignmentBaseline => "alignment-baseline",
            AllowReorder => "allowReorder",
            Alphabetic => "alphabetic",
            Amplitude => "amplitude",
            ArabicForm => "arabic-form",
            Ascent => "ascent",
            AttributeName => "attributeName",
            AttributeType => "attributeType",
            AutoReverse => "autoReverse",
            Azimuth => "azimuth",
            BaseFrequency => "baseFrequency",
            BaselineShift => "baseline-shift",
            BaseProfile => "baseProfile",
            Bbox => "bbox",
            Begin => "begin",
            Bias => "bias",
            By => "by",
            CalcMode => "calcMode",
            CapHeight => "cap-height",
            Class => "class",
            Clip => "clip",
            ClipPathUnits => "clipPathUnits",
            ClipPath => "clip-path",
            ClipRule => "clip-rule",
            Color => "color",
            ColorInterpolation => "color-interpolation",
            ColorInterpolationfilters => "color-interpolation-filters",
            ColorProfile => "color-profile",
            ColorRendering => "color-rendering",
            ContentScriptType => "contentScriptType",
            ContentStyleType => "contentStyleType",
            Cursor => "cursor",
            Cx => "cx",
            Cy => "cy",
            D => "d",
            Decelerate => "decelerate",
            Descent => "descent",
            DiffuseConstant => "diffuseConstant",
            Direction => "direction",
            Display => "display",
            Divisor => "divisor",
            DominantBaseline => "dominant-baseline",
            Dur => "dur",
            Dx => "dx",
            Dy => "dy",
            EdgeMode => "edgeMode",
            Elevation => "elevation",
            EnableBackground => "enable-background",
            End => "end",
            Exponent => "exponent",
            ExternalResourcesRequired => "externalResourcesRequired",
            Fill => "fill",
            FillOpacity => "fill-opacity",
            FillRule => "fill-rule",
            Filter => "filter",
            FilterRes => "filterRes",
            FilterUnits => "filterUnits",
            FloodColor => "flood-color",
            FloodOpacity => "flood-opacity",
            FontFamily => "font-family",
            FontSize => "font-size",
            FontSizeadjust => "font-size-adjust",
            FontStretch => "font-stretch",
            FontStyle => "font-style",
            FontVariant => "font-variant",
            FontWeight => "font-weight",
            Format => "format",
            From => "from",
            Fr => "fr",
            Fx => "fx",
            Fy => "fy",
            G1 => "g1",
            G2 => "g2",
            GlyphName => "glyph-name",
            GlyphOrientationhorizontal => "glyph-orientation-horizontal",
            GlyphOrientationvertical => "glyph-orientation-vertical",
            GlyphRef => "glyphRef",
            GradientTransform => "gradientTransform",
            GradientUnits => "gradientUnits",
            Hanging => "hanging",
            Height => "height",
            Href => "href",
            Hreflang => "hreflang",
            HorizAdvx => "horiz-adv-x",
            HorizOriginx => "horiz-origin-x",
            Id => "id",
            Ideographic => "ideographic",
            ImageRendering => "image-rendering",
            In => "in",
            In2 => "in2",
            Intercept => "intercept",
            K => "k",
            K1 => "k1",
            K2 => "k2",
            K3 => "k3",
            K4 => "k4",
            KernelMatrix => "kernelMatrix",
            KernelUnitLength => "kernelUnitLength",
            Kerning => "kerning",
            KeyPoints => "keyPoints",
            KeySplines => "keySplines",
            KeyTimes => "keyTimes",
            Lang => "lang",
            LengthAdjust => "lengthAdjust",
            LetterSpacing => "letter-spacing",
            LightingColor => "lighting-color",
            LimitingConeAngle => "limitingConeAngle",
            Local => "local",
            MarkerEnd => "marker-end",
            MarkerMid => "marker-mid",
            MarkerStart => "marker-start",
            MarkerHeight => "markerHeight",
            MarkerUnits => "markerUnits",
            MarkerWidth => "markerWidth",
            Mask => "mask",
            MaskContentUnits => "maskContentUnits",
            MaskUnits => "maskUnits",
            Mathematical => "mathematical",
            Max => "max",
            Media => "media",
            Method => "method",
            Min => "min",
            Mode => "mode",
            Name => "name",
            NumOctaves => "numOctaves",
            Offset => "offset",
            Opacity => "opacity",
            Operator => "operator",
            Order => "order",
            Orient => "orient",
            Orientation => "orientation",
            Origin => "origin",
            Overflow => "overflow",
            OverlinePosition => "overline-position",
            OverlineThickness => "overline-thickness",
            Panose1 => "panose-1",
            PaintOrder => "paint-order",
            Path => "path",
            PathLength => "pathLength",
            PatternContentUnits => "patternContentUnits",
            PatternTransform => "patternTransform",
            PatternUnits => "patternUnits",
            Ping => "ping",
            PointerEvents => "pointer-events",
            Points => "points",
            PointsAtX => "pointsAtX",
            PointsAtY => "pointsAtY",
            PointsAtZ => "pointsAtZ",
            PreserveAlpha => "preserveAlpha",
            PreserveAspectRatio => "preserveAspectRatio",
            PrimitiveUnits => "primitiveUnits",
            R => "r",
            Radius => "radius",
            ReferrerPolicy => "referrerPolicy",
            RefX => "refX",
            RefY => "refY",
            Rel => "rel",
            RenderingIntent => "rendering-intent",
            RepeatCount => "repeatCount",
            RepeatDur => "repeatDur",
            RequiredExtensions => "requiredExtensions",
            RequiredFeatures => "requiredFeatures",
            Restart => "restart",
            Result => "result",
            Rotate => "rotate",
            Slope => "slope",
            Spacing => "spacing",
            SpecularConstant => "specularConstant",
            SpecularExponent => "specularExponent",
            Speed => "speed",
            SpreadMethod => "spreadMethod",
            StartOffset => "startOffset",
            StdDeviation => "stdDeviation",
            Stemh => "stemh",
            Stemv => "stemv",
            StitchTiles => "stitchTiles",
            StopColor => "stop-color",
            StopOpacity => "stop-opacity",
            StrikethroughPosition => "strikethrough-position",
            StrikethroughThickness => "strikethrough-thickness",
            String => "string",
            Stroke => "stroke",
            StrokeDasharray => "stroke-dasharray",
            StrokeDashoffset => "stroke-dashoffset",
            StrokeLinecap => "stroke-linecap",
            StrokeLinejoin => "stroke-linejoin",
            StrokeMiterlimit => "stroke-miterlimit",
            StrokeOpacity => "stroke-opacity",
            StrokeWidth => "stroke-width",
            Style => "style",
            SurfaceScale => "surfaceScale",
            SystemLanguage => "systemLanguage",
            Tabindex => "tabindex",
            TableValues => "tableValues",
            Target => "target",
            TargetX => "targetX",
            TargetY => "targetY",
            TextAnchor => "text-anchor",
            TextDecoration => "text-decoration",
            TextRendering => "text-rendering",
            TextLength => "textLength",
            To => "to",
            Transform => "transform",
            Type => "type",
            U1 => "u1",
            U2 => "u2",
            UnderlinePosition => "underline-position",
            UnderlineThickness => "underline-thickness",
            Unicode => "unicode",
            UnicodeBidi => "unicode-bidi",
            UnicodeRange => "unicode-range",
            UnitsPerem => "units-per-em",
            VAlphabetic => "v-alphabetic",
            VHanging => "v-hanging",
            VIdeographic => "v-ideographic",
            VMathematical => "v-mathematical",
            Values => "values",
            VectorEffect => "vector-effect",
            Version => "version",
            VertAdvy => "vert-adv-y",
            VertOriginx => "vert-origin-x",
            VertOriginy => "vert-origin-y",
            ViewBox => "viewBox",
            ViewTarget => "viewTarget",
            Visibility => "visibility",
            Width => "width",
            Widths => "widths",
            WordSpacing => "word-spacing",
            WritingMode => "writing-mode",
            X => "x",
            XHeight => "x-height",
            X1 => "x1",
            X2 => "x2",
            XChannelSelector => "xChannelSelector",
            XlinkActuate => "xlink:actuate",
            XlinkArcrole => "xlink:arcrole",
            XlinkHref => "xlink:href",
            XlinkRole => "xlink:role",
            XlinkShow => "xlink:show",
            XlinkTitle => "xlink:title",
            XlinkType => "xlink:type",
            XmlBase => "xml:base",
            XmlLang => "xml:lang",
            XmlSpace => "xml:space",
            Y => "y",
            Y1 => "y1",
            Y2 => "y2",
            YChannelSelector => "yChannelSelector",
            Z => "z",
            ZoomAndPan => "zoomAndPan",
        })
    }
}