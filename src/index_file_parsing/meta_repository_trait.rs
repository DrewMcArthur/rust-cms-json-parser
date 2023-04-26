pub trait MetaRepository<'a> {
    fn add_file(&self, file: &'a mut FileRowInput<'a>) -> usize;
    fn add_link(&self, link: &'a mut DbLinkInput<'a>) -> usize;
    fn add_plan(&self, plan: &'a mut PlanInput<'a>) -> usize;
}

pub trait FromInput<'a, I, O> {
    fn from_input(id: usize, item: &'a I) -> O;
}

#[derive(Clone)]
pub struct FileRowInput<'a> {
    pub url: &'a str,
    pub filename: &'a str,
    pub reporting_entity_name: &'a str,
    pub reporting_entity_type: &'a str,
}

#[derive(Clone, Copy)]
pub struct FileRow<'a> {
    pub id: usize,
    pub url: &'a str,
    pub filename: &'a str,
    pub reporting_entity_name: &'a str,
    pub reporting_entity_type: &'a str,
}

impl<'a> FromInput<'a, FileRowInput<'a>, FileRow<'a>> for FileRow<'a> {
    fn from_input(id: usize, file: &'a FileRowInput<'a>) -> FileRow {
        return FileRow {
            id,
            url: file.url,
            filename: file.filename,
            reporting_entity_name: file.reporting_entity_name,
            reporting_entity_type: file.reporting_entity_type,
        };
    }
}

#[derive(Clone)]
pub struct DbLinkInput<'a> {
    pub from_id: usize,
    pub from_type: &'a str,
    pub to_id: usize,
    pub to_type: &'a str,
}

#[derive(Clone, Copy)]
pub struct DbLink<'a> {
    pub id: usize,
    pub from_id: usize,
    pub from_type: &'a str,
    pub to_id: usize,
    pub to_type: &'a str,
}

impl<'a> FromInput<'a, DbLinkInput<'a>, DbLink<'a>> for DbLink<'a> {
    fn from_input(id: usize, link: &'a DbLinkInput) -> DbLink<'a> {
        return DbLink {
            id,
            from_id: link.from_id,
            from_type: link.from_type,
            to_id: link.to_id,
            to_type: link.to_type,
        };
    }
}

#[derive(Clone)]
pub struct PlanInput<'a> {
    pub plan_name: &'a str,
    pub plan_id_type: &'a str,
    pub plan_market_type: &'a str,
    pub plan_id: &'a str,
}

impl<'a> PlanInput<'a> {
    pub(crate) fn from_reporting_plan(plan: &'a super::index_file::ReportingPlan) -> PlanInput<'a> {
        PlanInput {
            plan_name: &plan.plan_name,
            plan_id_type: &plan.plan_id_type,
            plan_market_type: &plan.plan_market_type,
            plan_id: &plan.plan_id,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Plan<'a> {
    pub id: usize,
    pub plan_name: &'a str,
    pub plan_id_type: &'a str,
    pub plan_market_type: &'a str,
    pub plan_id: &'a str,
}

impl<'a> FromInput<'a, PlanInput<'a>, Plan<'a>> for Plan<'a> {
    fn from_input(id: usize, plan: &'a PlanInput) -> Plan<'a> {
        return Plan {
            id,
            plan_name: plan.plan_name,
            plan_id_type: plan.plan_id_type,
            plan_market_type: plan.plan_market_type,
            plan_id: plan.plan_id,
        };
    }
}
