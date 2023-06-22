use crate::interpreter::utils::LabelError;

use super::{dobjects::StyledDObject, CM};

impl StyledDObject<'_> {
    /// **This method _assumes that config `label` is present.**
    pub fn label(&self) -> Result<String, LabelError> {
        let label = self.get_unchecked("label");
        let size = self.get_unchecked("labelsize");
        let dist = self
            .get_unchecked("dist")
            .try_into_f64()
            .map_err(|_| LabelError::WrongConfigType)?
            / CM;
        let angle = self
            .get_unchecked("angle")
            .try_into_f64()
            .map_err(|_| LabelError::WrongConfigType)?;
        let loc = self
            .get_unchecked("loc")
            .try_into_f64()
            .map_err(|_| LabelError::WrongConfigType)?;
        let font = self.get_unchecked("font");
        let pos = self.get_position(loc).ok_or(LabelError::ObjNotSupported)?;
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
