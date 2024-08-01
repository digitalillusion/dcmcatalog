use dicom::{
    core::Tag,
    object::{open_file, FileDicomObject, InMemDicomObject},
};
use log::debug;

use super::types::{DicomError, DicomInfo};

pub struct FieldNameAndTag(String, Tag);

pub enum DicomFileField {
    PersonName,
    PatientId,
}

impl DicomFileField {
    fn value(&self) -> FieldNameAndTag {
        match *self {
            DicomFileField::PersonName => FieldNameAndTag("PersonName".to_string(), Tag(0x0010, 0x0010)),
            DicomFileField::PatientId => FieldNameAndTag("PatientID".to_string(), Tag(0x0010, 0x0020)),
        }
    }
}

fn read_element_string_value(
    obj: &FileDicomObject<InMemDicomObject>,
    field: DicomFileField,
) -> Result<String, Box<DicomError>> {
    std::str::from_utf8(
        &obj.element(field.value().1)
            .map_err(|e| Box::new(DicomError::GetElement(field.value().0, e)))?
            .to_bytes()
            .map_err(|e| Box::new(DicomError::GetElementBytes(field.value().0, e)))?,
    )
    .map_err(|e| Box::new(DicomError::ToString(field.value().0, e)))
    .map(|utf8| utf8.to_string())
}

pub fn read_dicom_file(file: &String) -> Result<DicomInfo, Box<DicomError>> {
    let obj = open_file(file).map_err(|e| DicomError::OpenFile(file.to_string(), e))?;

    let patient_id = read_element_string_value(&obj, DicomFileField::PatientId)?;
    let person_name = read_element_string_value(&obj, DicomFileField::PersonName)?;

    debug!("File: {file} - PatientID: {patient_id} - PersonName: {person_name}");

    Ok(DicomInfo { person_name, patient_id })
}
