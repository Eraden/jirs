use {crate::shared::drag::DragState, jirs_data::*, std::collections::HashMap};

#[derive(Default, Debug)]
pub struct StatusIssueIds {
    pub status_id: IssueStatusId,
    pub status_name: IssueStatusName,
    pub issue_ids: Vec<IssueId>,
}

#[derive(Default, Debug)]
pub struct EpicIssuePerStatus {
    pub epic_name: EpicName,
    pub per_status_issues: Vec<StatusIssueIds>,
}

#[derive(Debug, Default)]
pub struct ProjectPage {
    pub text_filter: String,
    pub active_avatar_filters: Vec<UserId>,
    pub only_my_filter: bool,
    pub recently_updated_filter: bool,
    pub issue_drag: DragState,
    pub visible_issues: Vec<EpicIssuePerStatus>,
}

impl ProjectPage {
    pub fn rebuild_visible(
        &mut self,
        epics: &[Epic],
        statuses: &[IssueStatus],
        issues: &[Issue],
        user: &Option<User>,
    ) {
        let mut map = vec![];
        let epics = vec![None]
            .into_iter()
            .chain(epics.iter().map(|s| Some((s.id, s.name.as_str()))));

        let statuses = statuses.iter().map(|s| (s.id, s.name.as_str()));

        let mut issues: Vec<&Issue> = {
            let mut m = HashMap::new();
            let mut sorted = vec![];
            for issue in issues.iter() {
                sorted.push((issue.id, issue.updated_at));
                m.insert(issue.id, issue);
            }
            sorted.sort_by(|(_, a_time), (_, b_time)| a_time.cmp(b_time));
            sorted
                .into_iter()
                .flat_map(|(id, _)| m.remove(&id))
                .collect()
        };
        if self.recently_updated_filter {
            issues = issues[0..10].to_vec()
        }

        for epic in epics {
            let mut per_epic_map = EpicIssuePerStatus::default();
            per_epic_map.epic_name = epic.map(|(_, name)| name).unwrap_or_default().to_string();

            for (current_status_id, issue_status_name) in statuses.to_owned() {
                let mut per_status_map = StatusIssueIds::default();
                per_status_map.status_id = current_status_id;
                per_status_map.status_name = issue_status_name.to_string();
                for issue in issues.iter() {
                    if issue.epic_id == epic.map(|(id, _)| id)
                        && issue_filter_status(issue, current_status_id)
                        && issue_filter_with_avatars(issue, &self.active_avatar_filters)
                        && issue_filter_with_text(issue, self.text_filter.as_str())
                        && issue_filter_with_only_my(issue, self.only_my_filter, user)
                    {
                        per_status_map.issue_ids.push(issue.id);
                    }
                }
                per_epic_map.per_status_issues.push(per_status_map);
            }
            map.push(per_epic_map);
        }
        self.visible_issues = map;
    }
}

#[inline]
fn issue_filter_with_avatars(issue: &Issue, user_ids: &[UserId]) -> bool {
    if user_ids.is_empty() {
        return true;
    }
    user_ids.contains(&issue.reporter_id) || issue.user_ids.iter().any(|id| user_ids.contains(id))
}

#[inline]
fn issue_filter_status(issue: &Issue, current_status_id: IssueStatusId) -> bool {
    issue.issue_status_id == current_status_id
}

#[inline]
fn issue_filter_with_text(issue: &Issue, text: &str) -> bool {
    text.is_empty() || issue.title.contains(text)
}

#[inline]
fn issue_filter_with_only_my(issue: &Issue, only_my: bool, user: &Option<User>) -> bool {
    let my_id = user.as_ref().map(|u| u.id).unwrap_or_default();
    !only_my || issue.user_ids.contains(&my_id)
}