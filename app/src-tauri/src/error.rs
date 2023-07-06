use code_generator::CodeGeneratorError;

#[derive(Debug, thiserror::Error)]
pub enum TauriError {
    #[error("lock error: {0:?}")]
    SeaDbErrpr(#[from] sea_orm::DbErr),
    #[error(transparent)]
    CodeGeneratorError(#[from] CodeGeneratorError),
}
impl serde::Serialize for TauriError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
