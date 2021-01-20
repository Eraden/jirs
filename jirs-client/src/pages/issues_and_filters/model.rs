use {
    crate::model,
    jirs_data::{Issue, IssueId},
};

#[derive(Debug)]
pub struct Model {
    pub visible_issues: Vec<IssueId>,
    pub active_id: Option<IssueId>,
}

impl Model {
    pub fn new(model: &model::Model) -> Self {
        let visible_issues = Self::visible_issues(model.issues());
        let active_id = model.issues().first().as_ref().map(|issue| issue.id);

        Self {
            visible_issues,
            active_id,
        }
    }

    pub fn visible_issues(issues: &[Issue]) -> Vec<IssueId> {
        issues.iter().map(|issue| issue.id).collect()
    }
}
