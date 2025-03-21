use std::path::PathBuf;

use crate::EmailResult;
use handlebars::Handlebars;

pub static BASE_TEMPLATE: &str = include_str!("../templates/base.hbs");
pub static ATTACHMENT_TEMPLATE: &str = include_str!("../templates/attachment.hbs");

pub static TEMPLATES: &[(&str, &str)] =
	&[("base", BASE_TEMPLATE), ("attachment", ATTACHMENT_TEMPLATE)];

// TODO: expose this enumeration to the public API somehow, so that users can define their own template overrides

pub enum EmailTemplate {
	/// A template for an email which includes attachment(s), e.g. a book on the server
	Attachment,
}

impl AsRef<str> for EmailTemplate {
	fn as_ref(&self) -> &str {
		match self {
			Self::Attachment => "attachment",
		}
	}
}

/// Render a template to a string using the given data and templates directory.
/// If the template does not exist on disk, the default template will be used.
///
/// # Example
/// ```no_run
/// use email::{render_template, EmailTemplate};
/// use serde_json::json;
/// use std::path::PathBuf;
///
/// let data = json!({
///     "title": "Stump Attachment",
/// });
///
/// let rendered = render_template(EmailTemplate::Attachment, &data, PathBuf::from("templates")).unwrap();
/// assert!(rendered.contains("Stump Attachment"));
/// ```
pub fn render_template(
	template: EmailTemplate,
	data: &serde_json::Value,
	templates_dir: PathBuf,
) -> EmailResult<String> {
	let mut handlebars = Handlebars::new();
	handlebars.register_partial("base_partial", "{{> base}}")?;

	for (name, template) in TEMPLATES {
		let override_template = templates_dir.join(format!("{}.hbs", name));
		if override_template.exists() {
			handlebars.register_template_file(name, override_template)?;
		} else {
			handlebars.register_template_string(name, template)?;
		}
	}

	Ok(handlebars.render(template.as_ref(), data)?)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn default_templates_dir() -> PathBuf {
		PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates")
	}

	#[test]
	fn render_template_attachment_from_disk() {
		let data = serde_json::json!({
			"title": "Stump Attachment",
		});

		let rendered =
			render_template(EmailTemplate::Attachment, &data, default_templates_dir())
				.unwrap();

		assert!(rendered.contains("Stump Attachment"));
	}

	#[test]
	fn render_default_template_attachment() {
		let data = serde_json::json!({
			"title": "Stump Attachment",
		});

		let rendered =
			render_template(EmailTemplate::Attachment, &data, PathBuf::new()).unwrap();

		assert!(rendered.contains("Stump Attachment"));
	}
}
