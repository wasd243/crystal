use super::{CrystalPackage, CrystalParseErr};

/// implementation for CrystalPackage
impl CrystalPackage {
    /// Check if the package name is valid
    /// `\n` `\` `/` `\*` are all invalid.
    pub(super) fn check(&self) -> Result<(), CrystalParseErr> {
        if self.name.contains("/")
            || self.name.contains("\\")
            || self.name.chars().any(|c| c.is_whitespace() || c.is_control())
        {
            return Err(CrystalParseErr::InvalidPackageName);
        }
        Ok(())
    }
}
