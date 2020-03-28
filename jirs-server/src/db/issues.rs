use crate::db::DbExecutor;
use crate::errors::ServiceErrors;
use crate::models::Issue;
use actix::{Handler, Message};
use diesel::expression::dsl::not;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoadIssue {
    pub issue_id: i32,
}

impl Message for LoadIssue {
    type Result = Result<Issue, ServiceErrors>;
}

impl Handler<LoadIssue> for DbExecutor {
    type Result = Result<Issue, ServiceErrors>;

    fn handle(&mut self, msg: LoadIssue, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issues::dsl::{id, issues};
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let record = issues
            .filter(id.eq(msg.issue_id))
            .distinct()
            .first::<Issue>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project issues".to_string()))?;
        Ok(record)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoadProjectIssues {
    pub project_id: i32,
}

impl Message for LoadProjectIssues {
    type Result = Result<Vec<Issue>, ServiceErrors>;
}

impl Handler<LoadProjectIssues> for DbExecutor {
    type Result = Result<Vec<Issue>, ServiceErrors>;

    fn handle(&mut self, msg: LoadProjectIssues, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issues::dsl::{issues, project_id};
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let vec = issues
            .filter(project_id.eq(msg.project_id))
            .distinct()
            .load::<Issue>(conn)
            .map_err(|_| ServiceErrors::RecordNotFound("project issues".to_string()))?;
        Ok(vec)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssue {
    pub issue_id: i32,
    pub title: Option<String>,
    pub issue_type: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub list_position: Option<f64>,
    pub description: Option<Option<String>>,
    pub description_text: Option<Option<String>>,
    pub estimate: Option<Option<i32>>,
    pub time_spent: Option<Option<i32>>,
    pub time_remaining: Option<Option<i32>>,
    pub project_id: Option<i32>,

    pub users: Option<Vec<jirs_data::User>>,
    pub user_ids: Option<Vec<i32>>,
}

impl Message for UpdateIssue {
    type Result = Result<Issue, ServiceErrors>;
}

impl Handler<UpdateIssue> for DbExecutor {
    type Result = Result<Issue, ServiceErrors>;

    fn handle(&mut self, msg: UpdateIssue, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::issues::dsl::{self, issues};
        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let current_issue_id = msg.issue_id;

        let chain = diesel::update(issues.find(current_issue_id)).set((
            msg.title.map(|title| dsl::title.eq(title)),
            msg.issue_type
                .map(|issue_type| dsl::issue_type.eq(issue_type)),
            msg.status.map(|status| dsl::status.eq(status)),
            msg.priority.map(|priority| dsl::priority.eq(priority)),
            msg.list_position
                .map(|list_position| dsl::list_position.eq(list_position)),
            msg.description
                .map(|description| dsl::description.eq(description)),
            msg.description_text
                .map(|description_text| dsl::description_text.eq(description_text)),
            msg.estimate.map(|estimate| dsl::estimate.eq(estimate)),
            msg.time_spent
                .map(|time_spent| dsl::time_spent.eq(time_spent)),
            msg.time_remaining
                .map(|time_remaining| dsl::time_remaining.eq(time_remaining)),
            msg.project_id
                .map(|project_id| dsl::project_id.eq(project_id)),
            dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
        ));
        diesel::debug_query::<diesel::pg::Pg, _>(&chain);
        chain
            .get_result::<Issue>(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        if let Some(user_ids) = msg.user_ids.as_ref() {
            use crate::schema::issue_assignees::dsl;
            diesel::delete(dsl::issue_assignees)
                .filter(not(dsl::user_id.eq_any(user_ids)).and(dsl::issue_id.eq(current_issue_id)))
                .execute(conn)
                .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
            let existing: Vec<i32> = dsl::issue_assignees
                .select(dsl::user_id)
                .filter(dsl::issue_id.eq(current_issue_id))
                .get_results::<i32>(conn)
                .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
            let mut values = vec![];
            for user_id in user_ids.iter() {
                if !existing.contains(user_id) {
                    values.push(crate::models::CreateIssueAssigneeForm {
                        issue_id: current_issue_id,
                        user_id: *user_id,
                    })
                }
            }
            diesel::insert_into(dsl::issue_assignees)
                .values(values)
                .execute(conn)
                .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        }

        let row = issues
            .find(msg.issue_id)
            .first::<Issue>(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        Ok(row)
    }
}
