use std::fs::OpenOptions;

use csv;

use super::meta_repository_trait::{
    DbLink, DbLinkInput, FileRow, FileRowInput, FromInput, MetaRepository, Plan, PlanInput,
};

pub struct CsvMetaRepository {
    pub files_csv_path: &'static str,
    pub links_csv_path: &'static str,
    pub plans_csv_path: &'static str,
}

impl CsvMetaRepository {
    fn _get_length_of_file_db(&self, db_path: &str) -> usize {
        // get id of last row in csv file
        // or just length of csv file?
        let reader = csv::Reader::from_path(db_path).expect("failed to open {db_path}");
        // add one to get new id
        return reader.into_records().count() as usize;
    }

    fn _write_row_to_file_db<
        'a,
        InputType,
        RowType: FromInput<'a, InputType, RowType> + IntoIterator,
    >(
        &self,
        db_path: &str,
        row: &'a InputType,
    ) -> usize
    where
        <RowType as IntoIterator>::Item: AsRef<[u8]>,
    {
        let id = self._get_length_of_file_db(db_path) + 1;

        let file_db = OpenOptions::new()
            .write(true)
            .append(true)
            .open(db_path)
            .unwrap();

        // todo ensure headers and newline are there
        let mut csv_writer = csv::Writer::from_writer(file_db);
        csv_writer
            .write_record(RowType::from_input(id, &row))
            .unwrap();
        csv_writer.flush().unwrap();
        // return id
        return id;
    }
}

impl MetaRepository for CsvMetaRepository {
    fn add_file(&self, file: FileRowInput) -> usize {
        return self._write_row_to_file_db::<FileRowInput, FileRow>(self.files_csv_path, &file);
    }

    fn add_link(&self, link: DbLinkInput) -> usize {
        return self._write_row_to_file_db::<DbLinkInput, DbLink>(self.links_csv_path, &link);
    }

    fn add_plan(&self, plan: PlanInput) -> usize {
        return self._write_row_to_file_db::<PlanInput, Plan>(self.plans_csv_path, &plan);
    }
}

pub struct FileRowIterator<'a> {
    file_row: FileRow<'a>,
    index: usize,
}

impl<'a> Iterator for FileRowIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item> = match self.index {
            0 => Some(self.file_row.id.to_string().into_bytes()),
            1 => Some(self.file_row.url.into()),
            2 => Some(self.file_row.filename.into()),
            3 => Some(self.file_row.reporting_entity_name.into()),
            4 => Some(self.file_row.reporting_entity_type.into()),
            _ => None,
        };
        self.index += 1;
        result
    }
}

impl<'a> IntoIterator for FileRow<'a> {
    type Item = Vec<u8>;

    type IntoIter = FileRowIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return FileRowIterator {
            file_row: self,
            index: 0,
        };
    }
}

pub struct DbLinkIterator<'a> {
    db_link: DbLink<'a>,
    index: usize,
}

impl<'a> Iterator for DbLinkIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item> = match self.index {
            0 => Some(self.db_link.id.to_string().into_bytes()),
            1 => Some(self.db_link.from_id.to_string().into_bytes()),
            2 => Some(self.db_link.from_type.to_string().into_bytes()),
            3 => Some(self.db_link.to_id.to_string().into_bytes()),
            4 => Some(self.db_link.to_type.to_string().into_bytes()),
            _ => None,
        };
        self.index += 1;
        result
    }
}

impl<'a> IntoIterator for DbLink<'a> {
    type Item = Vec<u8>;

    type IntoIter = DbLinkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return DbLinkIterator {
            db_link: self.clone(),
            index: 0,
        };
    }
}

pub struct PlanIterator<'a> {
    plan: Plan<'a>,
    index: usize,
}

impl<'a> Iterator for PlanIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item> = match self.index {
            0 => Some(self.plan.id.to_string().into_bytes()),
            1 => Some(self.plan.plan_name.to_string().into_bytes()),
            2 => Some(self.plan.plan_id_type.into()),
            3 => Some(self.plan.plan_id.into()),
            4 => Some(self.plan.plan_market_type.into()),
            _ => None,
        };
        self.index += 1;
        result
    }
}

impl<'a> IntoIterator for Plan<'a> {
    type Item = Vec<u8>;

    type IntoIter = PlanIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return PlanIterator {
            plan: self.clone(),
            index: 0,
        };
    }
}
