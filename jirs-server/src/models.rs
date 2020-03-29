use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub user_id: i32,
    pub issue_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<jirs_data::Comment> for Comment {
    fn into(self) -> jirs_data::Comment {
        jirs_data::Comment {
            id: self.id,
            body: self.body,
            user_id: self.user_id,
            issue_id: self.issue_id,
            created_at: self.created_at,
            updated_at: self.updated_at,

            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "comments"]
pub struct CommentForm {
    pub body: String,
    pub user_id: i32,
    pub issue_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub issue_type: String,
    pub status: String,
    pub priority: String,
    pub list_position: f64,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub reporter_id: i32,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<jirs_data::Issue> for Issue {
    fn into(self) -> jirs_data::Issue {
        jirs_data::Issue {
            id: self.id,
            title: self.title,
            issue_type: self.issue_type,
            status: self.status,
            priority: self.priority,
            list_position: self.list_position,
            description: self.description,
            description_text: self.description_text,
            estimate: self.estimate,
            time_spent: self.time_spent,
            time_remaining: self.time_remaining,
            reporter_id: self.reporter_id,
            project_id: self.project_id,
            created_at: self.created_at,
            updated_at: self.updated_at,

            user_ids: vec![],
        }
    }
}

impl Into<jirs_data::FullIssue> for Issue {
    fn into(self) -> jirs_data::FullIssue {
        jirs_data::FullIssue {
            id: self.id,
            title: self.title,
            issue_type: self.issue_type,
            status: self.status,
            priority: self.priority,
            list_position: self.list_position,
            description: self.description,
            description_text: self.description_text,
            estimate: self.estimate,
            time_spent: self.time_spent,
            time_remaining: self.time_remaining,
            reporter_id: self.reporter_id,
            project_id: self.project_id,
            created_at: self.created_at,
            updated_at: self.updated_at,

            user_ids: vec![],
            comments: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "issues"]
pub struct CreateIssueForm {
    pub title: String,
    #[serde(rename = "type")]
    pub issue_type: String,
    pub status: String,
    pub priority: String,
    pub list_position: f64,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub reporter_id: i32,
    pub project_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct IssueAssignee {
    pub id: i32,
    pub issue_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "issue_assignees"]
pub struct CreateIssueAssigneeForm {
    pub issue_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<jirs_data::Project> for Project {
    fn into(self) -> jirs_data::Project {
        jirs_data::Project {
            id: self.id,
            name: self.name,
            url: self.url,
            description: self.description,
            category: self.category,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "projects"]
pub struct UpdateProjectForm {
    pub name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub project_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<jirs_data::User> for User {
    fn into(self) -> jirs_data::User {
        jirs_data::User {
            id: self.id,
            name: self.name,
            email: self.email,
            avatar_url: self.avatar_url,
            project_id: self.project_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Into<jirs_data::User> for &User {
    fn into(self) -> jirs_data::User {
        jirs_data::User {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            avatar_url: self.avatar_url.clone(),
            project_id: self.project_id,
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct UserForm {
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub project_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub access_token: Uuid,
    pub refresh_token: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<jirs_data::Token> for Token {
    fn into(self) -> jirs_data::Token {
        jirs_data::Token {
            id: self.id,
            user_id: self.user_id,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "tokens"]
pub struct TokenForm {
    pub user_id: i32,
    pub access_token: Uuid,
    pub refresh_token: Uuid,
}
