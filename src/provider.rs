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

    async fn get_record(
        &self,
        reqwest: reqwest::Client,
        input: &Self::GetRecordInput,
    ) -> Self::GetRecordOutput;

    async fn get_records(
        &self,
        reqwest: reqwest::Client,
        input: &Self::GetRecordsInput,
    ) -> Self::GetRecordsOutput;

    async fn add_record(
        &self,
        reqwest: reqwest::Client,
        input: &Self::AddRecordInput,
    ) -> Self::AddRecordOutput;

    async fn update_record(
        &self,
        reqwest: reqwest::Client,
        input: &Self::UpdateRecordInput,
    ) -> Self::UpdateRecordOutput;

    async fn delete_record(
        &self,
        reqwest: reqwest::Client,
        input: &Self::DeleteRecordInput,
    ) -> Self::DeleteRecordOutput;
}
