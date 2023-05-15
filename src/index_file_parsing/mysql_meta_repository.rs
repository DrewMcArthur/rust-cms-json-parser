use mysql::{params, prelude::Queryable, Pool};

use super::meta_repository_trait::{
    BatchedMetaRepository, DbLinkInput, FileRowInput, MetaRepository, PlanInput,
};

pub struct MysqlMetaRepository {
    connection_pool: Pool,
}

impl MysqlMetaRepository {
    pub fn new() -> Self {
        let mysql_meta_repo_connection_string = "mysql://root@localhost:3306/metadb";
        MysqlMetaRepository {
            connection_pool: Pool::new(mysql_meta_repo_connection_string).unwrap(),
        }
    }
}

impl BatchedMetaRepository for MysqlMetaRepository {
    fn add_files(&self, files: Vec<FileRowInput>) -> Vec<usize> {
        let mut conn = self.connection_pool.get_conn().unwrap();
        conn.exec_batch(
            r"INSERT INTO file (url, filename, reporting_entity_name, reporting_entity_type)
              VALUES (:url, :filename, :re_name, :re_type)",
            files.iter().map(|file| {
                params! {
                    "url" => file.url,
                    "filename" => file.filename,
                    "re_name" => file.reporting_entity_name,
                    "re_type" => file.reporting_entity_type,
                }
            }),
        )
        .unwrap();
        // TODO get ids
        vec![]
    }

    fn add_links(&self, links: Vec<DbLinkInput>) -> Vec<usize> {
        let mut conn = self.connection_pool.get_conn().unwrap();
        conn.exec_batch(
            r"INSERT INTO link (index_file_id,data_file_id,plan_id)
              VALUES (:index_file_id, :data_file_id, :plan_id)",
            links.iter().map(|link| match link.to_type {
                "plan" => {
                    params! {
                        "index_file_id" => mysql::Value::NULL,
                        "data_file_id" => link.from_id,
                        "plan_id" => link.to_id,
                    }
                }
                "rate_file" => {
                    params! {
                        "index_file_id" => link.from_id,
                        "data_file_id" => link.to_id,
                        "plan_id" => mysql::Value::NULL,
                    }
                }
                _ => {
                    panic!("Unknown link type: {}", link.to_type);
                }
            }),
        )
        .unwrap();
        // todo
        vec![]
    }

    fn add_plans(&self, plans: Vec<PlanInput>) -> Vec<usize> {
        let mut conn = self.connection_pool.get_conn().unwrap();
        conn.exec_batch(
            r"INSERT INTO plan (plan_name, plan_id_type, plan_id, plan_market_type)
              VALUES (:plan_name, :plan_id_type, :plan_id, :plan_market_type)",
            plans.iter().map(|plan| {
                params! {
                    "plan_name" => plan.plan_name,
                    "plan_id_type" => plan.plan_id_type,
                    "plan_id" => plan.plan_id,
                    "plan_market_type" => plan.plan_market_type,
                }
            }),
        )
        .unwrap();
        // TODO get ids
        vec![]
    }
}

impl MetaRepository for MysqlMetaRepository {
    fn add_file(&self, file: FileRowInput) -> Option<usize> {
        self.add_files(vec![file]).first().copied()
    }

    fn add_link(&self, link: DbLinkInput) -> Option<usize> {
        self.add_links(vec![link]).first().copied()
    }

    fn add_plan(&self, plan: PlanInput) -> Option<usize> {
        self.add_plans(vec![plan]).first().copied()
    }
}
