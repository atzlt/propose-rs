use crate::interpreter::utils::LabelError;

use super::{render::StyledDObject, CM};

macro_rules! get_or_wrong_type {
    ($config:ident, $key:expr) => {
        $config
            .get_unchecked($key)
            .try_into_f64()
            .map_err(|_| LabelError::WrongConfigType)
    };
}

impl StyledDObject<'_> {
    /// **This method _assumes that config `label` is present.**
    pub fn label(&self) -> Result<String, LabelError> {
        let mut label = self.get_unchecked("label").to_string();

        // Get label styles.
        let size = get_or_wrong_type!(self, "labelsize")?;
        let dist = get_or_wrong_type!(self, "dist")? / CM;
        let angle = get_or_wrong_type!(self, "angle")?;
        let loc = get_or_wrong_type!(self, "loc")?;
        let font = self.get_unchecked("font");
        let pos = self.get_position(loc);

        // Process the label.

        if label.contains("_") {
            label = label.replacen(
                "_",
                format!(
                    "<tspan dy=\"{}\" font-size=\"{}\">",
                    size * 0.3,
                    size * 0.5,
                )
                .as_str(),
                1,
            );
            label += "</tspan>";
        }

        Ok(format!(
            "<text font-size=\"{}\" font-family=\"{}\" font-style=\"italic\" text-anchor=\"middle\" dominant-baseline=\"middle\" x=\"{}cm\" y=\"{}cm\">{}</text>",
            size,
            font,
            pos.x + dist * angle.cos(),
            -(pos.y + dist * angle.sin()),
            label,
        ))
    }
}
