mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-grid-3/
 * CSS Grid Layout Module Level 3
 */

// // https://drafts.csswg.org/css-grid-3/#grid-template-columns
// #[value(" none | <track-list> | <auto-track-list> | subgrid <line-name-list>? ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the content area")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item in the computed track list (see § 7.2.5 computed value of a track listing and § 7.2.3.3 interpolation/combination of repeat()); discrete otherwise")]
// pub enum GridTemplateColumns {}

// // https://drafts.csswg.org/css-grid-3/#grid-template-rows
// #[value(" none | <track-list> | <auto-track-list> | subgrid <line-name-list>? ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the content area")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item in the computed track list (see § 7.2.5 computed value of a track listing and § 7.2.3.3 interpolation/combination of repeat()); discrete otherwise")]
// pub enum GridTemplateRows {}

// // https://drafts.csswg.org/css-grid-3/#grid-template-areas
// #[value(" none | <string>+ ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum GridTemplateAreas {}

// // https://drafts.csswg.org/css-grid-3/#grid-template
// #[value(" none | [ <'grid-template-rows'> / <'grid-template-columns'> ] | [ <line-names>? <string> <track-size>? <line-names>? ]+ [ / <explicit-track-list> ]? ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum GridTemplate {}

// // https://drafts.csswg.org/css-grid-3/#grid-auto-columns
// #[value(" <track-size>+ ")]
// #[initial("auto")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("see track sizing")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item; discrete otherwise")]
// pub struct GridAutoColumns;

// // https://drafts.csswg.org/css-grid-3/#grid-auto-rows
// #[value(" <track-size>+ ")]
// #[initial("auto")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("see track sizing")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item; discrete otherwise")]
// pub struct GridAutoRows;

// // https://drafts.csswg.org/css-grid-3/#grid-auto-flow
// #[value(" [ row | column | row-reverse | column-reverse ] || dense || wrap-reverse ")]
// #[initial("row")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum GridAutoFlow {}

// // https://drafts.csswg.org/css-grid-3/#grid
// #[value(" <'grid-template'> | <'grid-template-rows'> / [ auto-flow && dense? ] <'grid-auto-columns'>? | [ auto-flow && dense? ] <'grid-auto-rows'>? / <'grid-template-columns'> ")]
// #[initial("none")]
// #[applies_to("grid containers")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub enum Grid {}

// // https://drafts.csswg.org/css-grid-3/#grid-row-start
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridRowStart;

// // https://drafts.csswg.org/css-grid-3/#grid-column-start
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridColumnStart;

// // https://drafts.csswg.org/css-grid-3/#grid-row-end
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridRowEnd;

// // https://drafts.csswg.org/css-grid-3/#grid-column-end
// #[value(" <grid-line> ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridColumnEnd;

// // https://drafts.csswg.org/css-grid-3/#grid-row
// #[value(" <grid-line> [ / <grid-line> ]? ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridRow;

// // https://drafts.csswg.org/css-grid-3/#grid-column
// #[value(" <grid-line> [ / <grid-line> ]? ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridColumn;

// // https://drafts.csswg.org/css-grid-3/#grid-area
// #[value(" <grid-line> [ / <grid-line> ]{0,3} ")]
// #[initial("auto")]
// #[applies_to("grid items and absolutely-positioned boxes whose containing block is a grid container")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub struct GridArea;

// https://drafts.csswg.org/css-grid-3/#masonry-direction
#[value(" row | column | row-reverse | column-reverse ")]
#[initial("column")]
#[applies_to("masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum MasonryDirection {}

// https://drafts.csswg.org/css-grid-3/#masonry-fill
#[value(" normal | reverse ")]
#[initial("normal")]
#[applies_to("masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum MasonryFill {}

// https://drafts.csswg.org/css-grid-3/#masonry-flow
#[value(" <'masonry-direction'> || <'masonry-fill'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct MasonryFlow;

// // https://drafts.csswg.org/css-grid-3/#masonry
// #[value(" <'masonry-template-areas'> || <'masonry-template-tracks'> || <'masonry-direction'> || <'masonry-fill'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct Masonry;

// // https://drafts.csswg.org/css-grid-3/#masonry-template-tracks
// #[value(" none | <track-list> | <masonry-auto-track-list> | subgrid <line-name-list>? ")]
// #[initial("repeat(auto-areas, auto)")]
// #[applies_to("masonry containers")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the content area")]
// #[canonical_order("per grammar")]
// #[animation_type("if list lengths match, by computed value type; otherwise, discrete")]
// pub enum MasonryTemplateTracks {}

// // https://drafts.csswg.org/css-grid-3/#masonry-template-areas
// #[value(" none | <string> ")]
// #[initial("none")]
// #[applies_to("masonry containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum MasonryTemplateAreas {}

// // https://drafts.csswg.org/css-grid-3/#masonry-auto-tracks
// #[value(" <'grid-auto-columns'> ")]
// #[initial("auto")]
// #[applies_to("grid containers")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the content area")]
// #[canonical_order("per grammar")]
// #[animation_type("if the list lengths match, by computed value type per item; discrete otherwise")]
// pub struct MasonryAutoTracks;

// // https://drafts.csswg.org/css-grid-3/#masonry-slack
// #[value(" <length-percentage> | infinite ")]
// #[initial("1em")]
// #[applies_to("masonry containers")]
// #[inherited("no")]
// #[percentages("relative to the grid-axis content box size of the masonry container")]
// #[canonical_order("per grammar")]
// #[animation_type("as length")]
// pub enum MasonrySlack {}
