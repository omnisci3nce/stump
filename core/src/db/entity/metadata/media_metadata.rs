use std::collections::HashMap;

use pdf::{
	object::InfoDict,
	primitive::{Dictionary, PdfString},
};
use prisma_client_rust::chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use specta::Type;
use utoipa::ToSchema;

use crate::{
	db::entity::{
		metadata::common::{
			age_rating_deserializer, comma_separated_list_to_vec, parse_age_restriction,
			string_list_deserializer,
		},
		page_dimension::PageDimensionsEntity,
	},
	prisma::media_metadata,
};

const NAIVE_DATE_FORMATS: [&str; 2] = ["%Y-%m-%d", "%m-%d-%Y"];

// TODO: use skip_serializing_none after upgrade specta: https://github.com/oscartbeaumont/specta/issues/235
// TODO: author field?
// NOTE: alias is used primarily to support ComicInfo.xml files, as that metadata
// is formatted in PascalCase
/// Struct representing the metadata for a processed file.
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default, ToSchema)]
pub struct MediaMetadata {
	/// The metadata of the media.
	#[serde(skip_deserializing, skip_serializing)]
	pub id: String,
	/// The title of the media.
	#[serde(alias = "Title", skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	/// The series name which the media belongs to. This isn't necessarily the same as the
	/// series name as it was interpreted by Stump.
	#[serde(alias = "Series", skip_serializing_if = "Option::is_none")]
	pub series: Option<String>,
	/// The number this media is in the series. This can be a float, e.g. 20.1,
	/// which typically represents a one-shot or special issue.
	#[serde(alias = "Number", skip_serializing_if = "Option::is_none")]
	pub number: Option<f64>,
	#[serde(alias = "Volume", skip_serializing_if = "Option::is_none")]
	pub volume: Option<i32>,
	/// The summary of the media.
	#[serde(alias = "Summary", skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	/// Optional notes about the media.
	#[serde(alias = "Notes", skip_serializing_if = "Option::is_none")]
	pub notes: Option<String>,
	/// The age rating of the media. This varies a lot between media, but Stump will try
	/// to normalize it to a number between 0 and 18.
	#[serde(
		default,
		alias = "AgeRating",
		deserialize_with = "age_rating_deserializer",
		skip_serializing_if = "Option::is_none"
	)]
	pub age_rating: Option<i32>,
	/// The genre(s) the media belongs to.
	#[serde(
		alias = "Genre",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub genre: Option<Vec<String>>,

	/// The year the media was published.
	#[serde(alias = "Year", skip_serializing_if = "Option::is_none")]
	pub year: Option<i32>,
	/// The month the media was published (1-12)
	#[serde(alias = "Month", skip_serializing_if = "Option::is_none")]
	pub month: Option<i32>,
	/// The day the media was published (1-31). The day is not validated against the month.
	#[serde(alias = "Day", skip_serializing_if = "Option::is_none")]
	pub day: Option<i32>,

	/// The writer(s) of the associated media
	#[serde(
		alias = "Writer",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub writers: Option<Vec<String>>,
	/// The penciller(s) of the associated media
	#[serde(
		alias = "Penciller",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub pencillers: Option<Vec<String>>,
	/// The inker(s) of the associated media
	#[serde(
		alias = "Inker",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub inkers: Option<Vec<String>>,
	/// The colorist(s) of the associated media
	#[serde(
		alias = "Colorist",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub colorists: Option<Vec<String>>,
	/// The letterer(s) of the associated media
	#[serde(
		alias = "Letterer",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub letterers: Option<Vec<String>>,
	/// The cover artist(s) of the associated media
	#[serde(
		alias = "CoverArtist",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub cover_artists: Option<Vec<String>>,
	/// The editor(s) of the associated media
	#[serde(
		alias = "Editor",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub editors: Option<Vec<String>>,
	/// The publisher of the associated media
	#[serde(alias = "Publisher", skip_serializing_if = "Option::is_none")]
	pub publisher: Option<String>,

	/// Link(s) to the associated media, e.g. a comixology link
	#[serde(
		alias = "Web",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub links: Option<Vec<String>>,
	/// A list of characters that appear in the associated media
	#[serde(
		alias = "Characters",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub characters: Option<Vec<String>>,
	/// A list of teams that appear in the associated media
	#[serde(
		alias = "Teams",
		deserialize_with = "string_list_deserializer",
		default = "Option::default",
		skip_serializing_if = "Option::is_none"
	)]
	pub teams: Option<Vec<String>>,

	/// The number of pages in the associated media. This does *not* take priority over
	/// the number of pages detected by the file processor.
	#[serde(alias = "PageCount", skip_serializing_if = "Option::is_none")]
	pub page_count: Option<i32>,
	/// The (height, width) dimensions of each page in the associated media. This is
	/// generated by the image analysis job and is [None] before being generated.
	#[serde(
		default = "Option::default",
		skip_deserializing,
		skip_serializing_if = "Option::is_none"
	)]
	pub page_dimensions: Option<PageDimensionsEntity>,
	//#[serde(alias = "Resolutions", default = "Option::default")]
	//pub resolutions: Option<Vec<Resolution>>,
	// TODO: pages, e.g. <Pages><Page Image="0" Type="FrontCover" ImageSize="741291" /></Pages>
}

impl MediaMetadata {
	pub fn into_prisma(self) -> Vec<media_metadata::SetParam> {
		vec![
			media_metadata::title::set(self.title),
			media_metadata::series::set(self.series),
			media_metadata::number::set(self.number),
			media_metadata::volume::set(self.volume),
			media_metadata::summary::set(self.summary),
			media_metadata::notes::set(self.notes),
			media_metadata::age_rating::set(self.age_rating),
			media_metadata::genre::set(self.genre.map(|v| v.join(", "))),
			media_metadata::year::set(self.year),
			media_metadata::month::set(self.month),
			media_metadata::day::set(self.day),
			media_metadata::writers::set(self.writers.map(|v| v.join(", "))),
			media_metadata::pencillers::set(self.pencillers.map(|v| v.join(", "))),
			media_metadata::inkers::set(self.inkers.map(|v| v.join(", "))),
			media_metadata::colorists::set(self.colorists.map(|v| v.join(", "))),
			media_metadata::letterers::set(self.letterers.map(|v| v.join(", "))),
			media_metadata::cover_artists::set(self.cover_artists.map(|v| v.join(", "))),
			media_metadata::editors::set(self.editors.map(|v| v.join(", "))),
			media_metadata::publisher::set(self.publisher),
			media_metadata::links::set(self.links.map(|v| v.join(", "))),
			media_metadata::characters::set(self.characters.map(|v| v.join(", "))),
			media_metadata::teams::set(self.teams.map(|v| v.join(", "))),
			media_metadata::page_count::set(self.page_count),
		]
	}
}

impl From<media_metadata::Data> for MediaMetadata {
	fn from(metadata: media_metadata::Data) -> Self {
		let page_dimensions = match metadata.page_dimensions() {
			Ok(opt) => opt.map(|data| PageDimensionsEntity::from(data.to_owned())),
			Err(_e) => None,
		};

		MediaMetadata {
			id: metadata.id,
			title: metadata.title,
			series: metadata.series,
			number: metadata.number,
			volume: metadata.volume,
			summary: metadata.summary,
			notes: metadata.notes,
			age_rating: metadata.age_rating,
			genre: metadata.genre.map(comma_separated_list_to_vec),
			year: metadata.year,
			month: metadata.month,
			day: metadata.day,
			writers: metadata.writers.map(comma_separated_list_to_vec),
			pencillers: metadata.pencillers.map(comma_separated_list_to_vec),
			inkers: metadata.inkers.map(comma_separated_list_to_vec),
			colorists: metadata.colorists.map(comma_separated_list_to_vec),
			letterers: metadata.letterers.map(comma_separated_list_to_vec),
			cover_artists: metadata.cover_artists.map(comma_separated_list_to_vec),
			editors: metadata.editors.map(comma_separated_list_to_vec),
			publisher: metadata.publisher,
			links: metadata.links.map(comma_separated_list_to_vec),
			characters: metadata.characters.map(comma_separated_list_to_vec),
			teams: metadata.teams.map(comma_separated_list_to_vec),
			page_count: metadata.page_count,
			page_dimensions,
		}
	}
}

// NOTE: this is primarily used for converting the EPUB metadata into a common Metadata struct
impl From<HashMap<String, Vec<String>>> for MediaMetadata {
	fn from(map: HashMap<String, Vec<String>>) -> Self {
		let mut metadata = MediaMetadata::default();

		for (key, value) in map {
			match key.to_lowercase().as_str() {
				"title" => metadata.title = Some(value.join("\n").to_string()),
				"series" => metadata.series = Some(value.join("\n").to_string()),
				"number" => {
					metadata.number =
						value.into_iter().next().and_then(|n| n.parse().ok());
				},
				"volume" => {
					metadata.volume =
						value.into_iter().next().and_then(|n| n.parse().ok());
				},
				"summary" => metadata.summary = Some(value.join("\n").to_string()),
				"notes" => metadata.notes = Some(value.join("\n").to_string()),
				"genre" => metadata.genre = Some(value),
				"year" => {
					metadata.year = value.into_iter().next().and_then(|n| n.parse().ok());
				},
				"month" => {
					metadata.month =
						value.into_iter().next().and_then(|n| n.parse().ok());
				},
				"day" => {
					metadata.day = value.into_iter().next().and_then(|n| n.parse().ok());
				},
				"pencillers" => metadata.pencillers = Some(value),
				"inkers" => metadata.inkers = Some(value),
				"colorists" => metadata.colorists = Some(value),
				"letterers" => metadata.letterers = Some(value),
				"coverartists" => metadata.cover_artists = Some(value),
				"editors" => metadata.editors = Some(value),
				"publisher" => metadata.publisher = Some(value.join("\n").to_string()),
				"links" => metadata.links = Some(value),
				"characters" => metadata.characters = Some(value),
				"teams" => metadata.teams = Some(value),
				"pagecount" => {
					metadata.page_count =
						value.into_iter().next().and_then(|n| n.parse().ok());
				},
				"date" => {
					// Note: we don't know the format of the date. It could be a year, a full date, etc.
					// We need to _try_ to parse each part of the date, and if it fails, we just ignore it.
					// This is a bit of a hack, but it's the best we can do without knowing the format.
					let raw_date = value.into_iter().next().unwrap_or_default();

					for format in &NAIVE_DATE_FORMATS {
						if let Ok(date) = NaiveDate::parse_from_str(&raw_date, format) {
							metadata.year = Some(date.year());
							metadata.month = Some(date.month() as i32);
							metadata.day = Some(date.day() as i32);
							break;
						}
					}

					if metadata.year.is_none() {
						if let Ok(year) = raw_date.parse() {
							metadata.year = Some(year);
						}
					}
				},
				// TODO: separate out writer vs author?
				"creator" | "author" | "writers" => match metadata.writers {
					Some(ref mut writers) => {
						writers.extend(value);
						// remove duplicates
						writers.sort();
						writers.dedup();
					},
					None => metadata.writers = Some(value),
				},
				"typicalagerange" | "contentrating" => {
					let parsed = value
						.into_iter()
						.next()
						.as_deref()
						.and_then(parse_age_restriction);

					match (metadata.age_rating, parsed) {
						// if metadata.age_rating has been set, we need to take the min of the two
						(Some(existing), Some(new)) => {
							metadata.age_rating = Some(existing.min(new));
						},
						// if metadata.age_rating has not been set, we can just take the new value
						(_, Some(new)) => metadata.age_rating = Some(new),
						_ => (),
					}
				},
				_ => (),
			}
		}

		metadata
	}
}

impl From<Dictionary> for MediaMetadata {
	fn from(dict: Dictionary) -> Self {
		// FIXME: this is pretty hacky! I need to match on the type of the value
		let map = dict
			.into_iter()
			.map(|(k, v)| v.to_string().map(|v| (k, v)))
			.filter_map(Result::ok)
			.map(|(k, v)| (k.to_lowercase(), vec![v]))
			.collect::<HashMap<String, Vec<String>>>();
		Self::from(map)
	}
}

fn pdf_string_to_string(pdf_string: PdfString) -> Option<String> {
	pdf_string.to_string().map_or_else(
		|error| {
			tracing::error!(error = ?error, "Failed to convert PdfString to String");
			None
		},
		|str| Some(str.trim().to_owned()),
	)
}

impl From<InfoDict> for MediaMetadata {
	fn from(dict: InfoDict) -> Self {
		MediaMetadata {
			title: dict.title.and_then(pdf_string_to_string),
			genre: dict.subject.and_then(pdf_string_to_string).map(|v| vec![v]),
			year: dict.creation_date.as_ref().map(|date| date.year as i32),
			month: dict.creation_date.as_ref().map(|date| date.month as i32),
			day: dict.creation_date.as_ref().map(|date| date.day as i32),
			writers: dict.author.and_then(pdf_string_to_string).map(|v| vec![v]),
			..Default::default()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pdf_string_to_string() {
		let pdf_string = PdfString::from("Hello, world!");
		assert_eq!(
			pdf_string_to_string(pdf_string),
			Some("Hello, world!".to_string())
		);
	}

	#[test]
	fn test_from_hashmap() {
		let mut map = HashMap::new();

		map.insert("Title".to_string(), vec![String::from("The Way of Kings")]);
		map.insert(
			"creator".to_string(),
			vec![String::from("Brandon Sanderson")],
		);
		map.insert("date".to_string(), vec![String::from("08-31-2010")]);
		map.insert("Genre".to_string(), vec![String::from("Fantasy")]);
		map.insert(
			"Summary".to_string(),
			vec![String::from("A book, you know?")],
		);

		let metadata = MediaMetadata::from(map);

		assert_eq!(metadata.title, Some("The Way of Kings".to_string()));
		assert_eq!(
			metadata.writers,
			Some(vec!["Brandon Sanderson".to_string()])
		);
		assert_eq!(metadata.year, Some(2010));
		assert_eq!(metadata.month, Some(8));
		assert_eq!(metadata.day, Some(31));
		assert_eq!(metadata.genre, Some(vec!["Fantasy".to_string()]));
		assert_eq!(metadata.summary, Some("A book, you know?".to_string()));
	}

	#[test]
	fn test_resolve_multiple_age_ratings() {
		let mut map = HashMap::new();

		map.insert("Title".to_string(), vec![String::from("The Way of Kings")]);
		map.insert("typicalAgeRange".to_string(), vec![String::from("14-17")]);
		map.insert("ContentRating".to_string(), vec![String::from("13+")]);

		let metadata = MediaMetadata::from(map);

		assert_eq!(metadata.age_rating, Some(13));
	}
}
