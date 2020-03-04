/// TagName provides tags for SVG creation
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TagName {
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/a)
    A,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animate)
    Animate,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateMotion)
    AnimateMotion,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateTransform)
    AnimateTransform,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/circle)
    Circle,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/clipPath)
    ClipPath,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/color-profile)
    ColorProfile,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/defs)
    Defs,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/desc)
    Desc,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/discard)
    Discard,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/ellipse)
    Ellipse,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feBlend)
    FeBlend,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feColorMatrix)
    FeColorMatrix,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComponentTransfer)
    FeComponentTransfer,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComposite)
    FeComposite,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feConvolveMatrix)
    FeConvolveMatrix,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDiffuseLighting)
    FeDiffuseLighting,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDisplacementMap)
    FeDisplacementMap,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDistantLight)
    FeDistantLight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDropShadow)
    FeDropShadow,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFlood)
    FeFlood,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncA)
    FeFuncA,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncB)
    FeFuncB,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncG)
    FeFuncG,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncR)
    FeFuncR,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feGaussianBlur)
    FeGaussianBlur,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feImage)
    FeImage,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMerge)
    FeMerge,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMergeNode)
    FeMergeNode,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMorphology)
    FeMorphology,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feOffset)
    FeOffset,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/fePointLight)
    FePointLight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpecularLighting)
    FeSpecularLighting,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpotLight)
    FeSpotLight,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTile)
    FeTile,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTurbulence)
    FeTurbulence,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/filter)
    Filter,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/foreignObject)
    ForeignObject,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g)
    G,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatch)
    Hatch,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatchpath)
    Hatchpath,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/image)
    Image,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/line)
    Line,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/linearGradient)
    LinearGradient,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/marker)
    Marker,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/mask)
    Mask,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element)
    Mesh,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element)
    Meshgradient,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element)
    Meshpatch,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element)
    Meshrow,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/metadata)
    Metadata,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/mpath)
    Mpath,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/path)
    Path,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/pattern)
    Pattern,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polygon)
    Polygon,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polyline)
    Polyline,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/radialGradient)
    RadialGradient,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/rect)
    Rect,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/script)
    Script,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/set)
    Set,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/solidcolor)
    Solidcolor,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/stop)
    Stop,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/style)
    Style,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg)
    Svg,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/switch)
    Switch,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/symbol)
    Symbol,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text)
    Text,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/textPath)
    TextPath,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/title)
    Title,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/tspan)
    Tspan,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element)
    Unknown,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/use)
    Use,

    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/view)
    View,
}

// Implementation of Tagname
impl ToString for TagName {
    fn to_string(&self) -> String {
        use TagName::*;

        String::from(match self {
            A => "a",
            Animate => "animate",
            AnimateMotion => "animateMotion",
            AnimateTransform => "animateTransform",
            Circle => "circle",
            ClipPath => "clipPath",
            ColorProfile => "color-profile",
            Defs => "defs",
            Desc => "desc",
            Discard => "discard",
            Ellipse => "ellipse",
            FeBlend => "feBlend",
            FeColorMatrix => "feColorMatrix",
            FeComponentTransfer => "feComponentTransfer",
            FeComposite => "feComposite",
            FeConvolveMatrix => "feConvolveMatrix",
            FeDiffuseLighting => "feDiffuseLighting",
            FeDisplacementMap => "feDisplacementMap",
            FeDistantLight => "feDistantLight",
            FeDropShadow => "feDropShadow",
            FeFlood => "feFlood",
            FeFuncA => "feFuncA",
            FeFuncB => "feFuncB",
            FeFuncG => "feFuncG",
            FeFuncR => "feFuncR",
            FeGaussianBlur => "feGaussianBlur",
            FeImage => "feImage",
            FeMerge => "feMerge",
            FeMergeNode => "feMergeNode",
            FeMorphology => "feMorphology",
            FeOffset => "feOffset",
            FePointLight => "fePointLight",
            FeSpecularLighting => "feSpecularLighting",
            FeSpotLight => "feSpotLight",
            FeTile => "feTile",
            FeTurbulence => "feTurbulence",
            Filter => "filter",
            ForeignObject => "foreignObject",
            G => "g",
            Hatch => "hatch",
            Hatchpath => "hatchpath",
            Image => "image",
            Line => "line",
            LinearGradient => "linearGradient",
            Marker => "marker",
            Mask => "mask",
            Mesh => "mesh",
            Meshgradient => "meshgradient",
            Meshpatch => "meshpatch",
            Meshrow => "meshrow",
            Metadata => "metadata",
            Mpath => "mpath",
            Path => "path",
            Pattern => "pattern",
            Polygon => "polygon",
            Polyline => "polyline",
            RadialGradient => "radialGradient",
            Rect => "rect",
            Script => "script",
            Set => "set",
            Solidcolor => "solidcolor",
            Stop => "stop",
            Style => "style",
            Svg => "svg",
            Switch => "switch",
            Symbol => "symbol",
            Text => "text",
            TextPath => "textPath",
            Title => "title",
            Tspan => "tspan",
            Unknown => "unknown",
            Use => "use",
            View => "view",
        })
    }
}

#[cfg(feature = "parser")]
pub fn parse(text: &str) -> Result<TagName, ()> {
    match text {
        "a" => Ok(TagName::A),
        "animate" => Ok(TagName::Animate),
        "animateMotion" => Ok(TagName::AnimateMotion),
        "animateTransform" => Ok(TagName::AnimateTransform),
        "circle" => Ok(TagName::Circle),
        "clipPath" => Ok(TagName::ClipPath),
        "color-profile" => Ok(TagName::ColorProfile),
        "defs" => Ok(TagName::Defs),
        "desc" => Ok(TagName::Desc),
        "discard" => Ok(TagName::Discard),
        "ellipse" => Ok(TagName::Ellipse),
        "feBlend" => Ok(TagName::FeBlend),
        "feColorMatrix" => Ok(TagName::FeColorMatrix),
        "feComponentTransfer" => Ok(TagName::FeComponentTransfer),
        "feComposite" => Ok(TagName::FeComposite),
        "feConvolveMatrix" => Ok(TagName::FeConvolveMatrix),
        "feDiffuseLighting" => Ok(TagName::FeDiffuseLighting),
        "feDisplacementMap" => Ok(TagName::FeDisplacementMap),
        "feDistantLight" => Ok(TagName::FeDistantLight),
        "feDropShadow" => Ok(TagName::FeDropShadow),
        "feFlood" => Ok(TagName::FeFlood),
        "feFuncA" => Ok(TagName::FeFuncA),
        "feFuncB" => Ok(TagName::FeFuncB),
        "feFuncG" => Ok(TagName::FeFuncG),
        "feFuncR" => Ok(TagName::FeFuncR),
        "feGaussianBlur" => Ok(TagName::FeGaussianBlur),
        "feImage" => Ok(TagName::FeImage),
        "feMerge" => Ok(TagName::FeMerge),
        "feMergeNode" => Ok(TagName::FeMergeNode),
        "feMorphology" => Ok(TagName::FeMorphology),
        "feOffset" => Ok(TagName::FeOffset),
        "fePointLight" => Ok(TagName::FePointLight),
        "feSpecularLighting" => Ok(TagName::FeSpecularLighting),
        "feSpotLight" => Ok(TagName::FeSpotLight),
        "feTile" => Ok(TagName::FeTile),
        "feTurbulence" => Ok(TagName::FeTurbulence),
        "filter" => Ok(TagName::Filter),
        "foreignObject" => Ok(TagName::ForeignObject),
        "g" => Ok(TagName::G),
        "hatch" => Ok(TagName::Hatch),
        "hatchpath" => Ok(TagName::Hatchpath),
        "image" => Ok(TagName::Image),
        "line" => Ok(TagName::Line),
        "linearGradient" => Ok(TagName::LinearGradient),
        "marker" => Ok(TagName::Marker),
        "mask" => Ok(TagName::Mask),
        "mesh" => Ok(TagName::Mesh),
        "meshgradient" => Ok(TagName::Meshgradient),
        "meshpatch" => Ok(TagName::Meshpatch),
        "meshrow" => Ok(TagName::Meshrow),
        "metadata" => Ok(TagName::Metadata),
        "mpath" => Ok(TagName::Mpath),
        "path" => Ok(TagName::Path),
        "pattern" => Ok(TagName::Pattern),
        "polygon" => Ok(TagName::Polygon),
        "polyline" => Ok(TagName::Polyline),
        "radialGradient" => Ok(TagName::RadialGradient),
        "rect" => Ok(TagName::Rect),
        "script" => Ok(TagName::Script),
        "set" => Ok(TagName::Set),
        "solidcolor" => Ok(TagName::Solidcolor),
        "stop" => Ok(TagName::Stop),
        "style" => Ok(TagName::Style),
        "svg" => Ok(TagName::Svg),
        "switch" => Ok(TagName::Switch),
        "symbol" => Ok(TagName::Symbol),
        "text" => Ok(TagName::Text),
        "textPath" => Ok(TagName::TextPath),
        "title" => Ok(TagName::Title),
        "tspan" => Ok(TagName::Tspan),
        "unknown" => Ok(TagName::Unknown),
        "use" => Ok(TagName::Use),
        "view" => Ok(TagName::View),
        _ => Err(()),
    }
}
