use leptos::html::div;
use leptos::prelude::*;
use leptos::svg::{circle, defs, feGaussianBlur, filter, g, svg};

/// The outline of a circle with a "glow" sitting behind it. This is provided
/// as an SVG and inverts its colours when the browser is in dark mode.
pub fn logo() -> impl IntoView {
    div().attr("class", "w-10 h-10").child(
        svg()
            .attr("class", "stroke-current")
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", "200 100 600 600")
            .attr("opacity", "1")
            .child(
                defs().child((
                    filter()
                        .attr("id", "nnneon-filter")
                        .attr("x", "-100%")
                        .attr("y", "-100%")
                        .attr("width", "400%")
                        .attr("height", "400%")
                        .attr("filterUnits", "objectBoundingBox")
                        .attr("primitiveUnits", "userSpaceOnUse")
                        .attr("color-interpolation-filters", "sRGB")
                        .child(
                            feGaussianBlur()
                                .attr("stdDeviation", "17 8")
                                .attr("x", "0%")
                                .attr("y", "0%")
                                .attr("width", "100%")
                                .attr("height", "100%")
                                .attr("in", "SourceGraphic")
                                .attr("edgeMode", "none")
                                .attr("result", "blur"),
                        ),
                    filter()
                        .attr("id", "nnneon-filter2")
                        .attr("x", "-100%")
                        .attr("y", "-100%")
                        .attr("width", "400%")
                        .attr("height", "400%")
                        .attr("filterUnits", "objectBoundingBox")
                        .attr("primitiveUnits", "userSpaceOnUse")
                        .attr("color-interpolation-filters", "sRGB")
                        .child(
                            feGaussianBlur()
                                .attr("stdDeviation", "10 17")
                                .attr("x", "0%")
                                .attr("y", "0%")
                                .attr("width", "100%")
                                .attr("height", "100%")
                                .attr("in", "SourceGraphic")
                                .attr("edgeMode", "none")
                                .attr("result", "blur"),
                        ),
                )),
            )
            .child(
                g().attr("stroke-width", "16").attr("fill", "none").child((
                    circle()
                        .attr("r", "150")
                        .attr("cx", "400")
                        .attr("cy", "400")
                        .attr("filter", "url(#nnneon-filter)"),
                    circle()
                        .attr("r", "150")
                        .attr("cx", "412")
                        .attr("cy", "400")
                        .attr("filter", "url(#nnneon-filter2)")
                        .attr("opacity", "0.25"),
                    circle()
                        .attr("r", "150")
                        .attr("cx", "388")
                        .attr("cy", "400")
                        .attr("filter", "url(#nnneon-filter2)")
                        .attr("opacity", "0.25"),
                    circle()
                        .attr("r", "150")
                        .attr("cx", "400")
                        .attr("cy", "400"),
                )),
            ),
    )
}
