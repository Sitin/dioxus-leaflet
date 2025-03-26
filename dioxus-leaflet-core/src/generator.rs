use crate::signals::{ApiEvent, ApiRecv, CtrlRecv, CtrlResp};

#[cfg(feature = "generator")]
pub fn generate(path: impl AsRef<std::path::Path>) -> Result<(), specta_typescript::ExportError> {
    use specta_typescript::BigIntExportBehavior;
    use specta::TypeCollection;

    let mut types = TypeCollection::default();
    types.register::<CtrlRecv>();
    types.register::<CtrlResp>();
    types.register::<ApiRecv>();
    types.register::<ApiEvent>();

    specta_typescript::Typescript::new()
        .bigint(BigIntExportBehavior::BigInt)
        .export_to(path, &types)
}
