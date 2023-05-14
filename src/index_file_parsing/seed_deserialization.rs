use core::fmt;
use std::sync::mpsc::SyncSender;

use serde::{
    de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::index_file_parsing::index_file::ProcessingStats;

use super::index_file::AsyncIndexFile;

pub struct ItemSeed<T> {
    pub sender: SyncSender<T>,
}

impl<'de, T> DeserializeSeed<'de> for ItemSeed<T>
where
    T: Deserialize<'de>,
{
    type Value = usize;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemsVisitor<T> {
            sender: SyncSender<T>,
        }

        impl<'de, T> Visitor<'de> for ItemsVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = usize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of objects of type T")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut num_items: usize = 0;
                while let Some(n) = seq.next_element()? {
                    // Might want some better error handling here.
                    if self.sender.send(n).is_err() {
                        break;
                    }
                    num_items += 1;
                }
                Ok(num_items)
            }
        }

        deserializer.deserialize_seq(ItemsVisitor {
            sender: self.sender,
        })
    }
}

pub struct FileSeed<T> {
    pub sender: SyncSender<T>,
}

impl<'de, T> DeserializeSeed<'de> for FileSeed<T>
where
    T: Deserialize<'de>,
{
    type Value = AsyncIndexFile;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            ReportingEntityName,
            ReportingEntityType,
            ReportingStructure,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`reporting_entity_name`, `reporting_entity_type`, or `reporting_structure`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "reporting_entity_name" => Ok(Field::ReportingEntityName),
                            "reporting_entity_type" => Ok(Field::ReportingEntityType),
                            "reporting_structure" => Ok(Field::ReportingStructure),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct FileVisitor<T> {
            sender: SyncSender<T>,
        }

        impl<'de, T> Visitor<'de> for FileVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = AsyncIndexFile;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AsyncIndexFile")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut reporting_entity_name = None;
                let mut reporting_entity_type = None;
                let mut reporting_structure = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ReportingEntityName => {
                            if reporting_entity_name.is_some() {
                                return Err(de::Error::duplicate_field("reporting_entity_name"));
                            }
                            reporting_entity_name = Some(map.next_value()?);
                        }
                        Field::ReportingEntityType => {
                            if reporting_entity_type.is_some() {
                                return Err(de::Error::duplicate_field("reporting_entity_type"));
                            }
                            reporting_entity_type = Some(map.next_value()?);
                        }
                        Field::ReportingStructure => {
                            if reporting_structure.is_some() {
                                return Err(de::Error::duplicate_field("reporting_structure"));
                            }
                            reporting_structure = Some(map.next_value_seed(ItemSeed {
                                sender: self.sender.clone(),
                            })?);
                        }
                    }
                }

                if reporting_structure.is_none() {
                    return Err(de::Error::missing_field("reporting_structure"));
                }

                Ok(AsyncIndexFile {
                    reporting_entity_name: reporting_entity_name
                        .ok_or_else(|| de::Error::missing_field("reporting_entity_name"))?,
                    reporting_entity_type: reporting_entity_type
                        .ok_or_else(|| de::Error::missing_field("reporting_entity_type"))?,
                    reporting_structure_processing_stats: ProcessingStats {
                        num_reporting_structures: reporting_structure.unwrap(),
                    },
                })
            }
        }

        const FIELDS: &[&str] = &[
            "reporting_entity_name",
            "reporting_entity_type",
            "reporting_structure",
        ];
        deserializer.deserialize_struct(
            "File",
            FIELDS,
            FileVisitor {
                sender: self.sender,
            },
        )
    }
}
