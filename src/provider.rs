pub mod nitrado;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Feature {
    GetRecord,
    GetRecords,
    AddRecord,
    UpdateRecord,
    DeleteRecord,
}

pub trait Provider {
    type GetRecordInput;
    type GetRecordOutput;
    type GetRecordsInput;
    type GetRecordsOutput;
    type AddRecordInput;
    type AddRecordOutput;
    type UpdateRecordInput;
    type UpdateRecordOutput;
    type DeleteRecordInput;
    type DeleteRecordOutput;

    fn get_provider_name() -> &'static str;

    fn get_supported_features() -> Vec<Feature>;
    fn is_feature_supported(feature: &Feature) -> bool {
        Self::get_supported_features().contains(feature)
    }

    fn get_record(&self, input: &Self::GetRecordInput) -> &Self::GetRecordOutput;
    fn get_records(&self, input: &Self::GetRecordsInput) -> &Self::GetRecordsOutput;
    fn add_record(&self, input: &Self::AddRecordInput) -> &Self::AddRecordOutput;
    fn update_record(&self, input: &Self::UpdateRecordInput) -> &Self::UpdateRecordOutput;
    fn delete_record(&self, input: &Self::DeleteRecordInput) -> &Self::DeleteRecordOutput;
}
