use std::collections::HashMap;

use chrono::NaiveDateTime;
use jirs_data::*;

use crate::shared::drag::DragState;

#[derive(Default, Debug)]
pub struct StatusIssueIds {
    pub status_id: IssueStatusId,
    pub status_name: IssueStatusName,
    pub issue_ids: Vec<IssueId>,
}

#[derive(Default, Debug)]
pub struct EpicIssuePerStatus {
    pub epic_ref: Option<(EpicId, EpicName, Option<StartsAt>, Option<EndsAt>)>,
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
    pub fn visible_issues(
        page: &ProjectPage,
        epics: &[Epic],
        statuses: &[IssueStatus],
        issues: &[Issue],
        user: &Option<User>,
    ) -> Vec<EpicIssuePerStatus> {
        let epics = vec![None].into_iter().chain(
            epics
                .iter()
                .map(|epic| Some((epic.id, epic.name.as_str(), epic.starts_at, epic.ends_at))),
        );

        let statuses = statuses.iter().map(|s| (s.id, s.name.as_str()));
        let issues = issues.iter().filter(|issue| {
            issue_filter_with_avatars(issue, &page.active_avatar_filters)
                && issue_filter_with_text(issue, page.text_filter.as_str())
                && issue_filter_with_only_my(issue, page.only_my_filter, user)
        });
        let issues = if page.recently_updated_filter {
            let mut m = HashMap::new();
            let mut sorted: Vec<(IssueId, NaiveDateTime)> = issues
                .map(|issue| {
                    m.insert(issue.id, issue);
                    (issue.id, issue.updated_at)
                })
                .collect();
            sorted.sort_by(|(_, a_time), (_, b_time)| a_time.cmp(b_time));
            let mut issues: Vec<&Issue> = sorted
                .into_iter()
                .take(10)
                .flat_map(|(id, _)| m.remove(&id))
                .collect();
            issues.sort_by(|a, b| a.list_position.cmp(&b.list_position));
            issues
        } else {
            issues.collect()
        };

        let issues_per_epic_id = issues.into_iter().fold(HashMap::new(), |mut m, issue| {
            m.entry(issue.epic_id).or_insert_with(Vec::new).push(issue);
            m
        });

        epics
            .map(|epic| {
                let mut per_epic_map = EpicIssuePerStatus {
                    epic_ref: epic.map(|(id, name, starts_at, ends_at)| {
                        (id, name.to_string(), starts_at, ends_at)
                    }),
                    ..Default::default()
                };

                for (current_status_id, issue_status_name) in statuses.to_owned() {
                    let per_status_map = StatusIssueIds {
                        status_id: current_status_id,
                        status_name: issue_status_name.to_string(),
                        issue_ids: issues_per_epic_id
                            .get(&epic.map(|(id, ..)| id))
                            .map(|v| {
                                v.iter()
                                    .filter(|issue| issue_filter_status(issue, current_status_id))
                                    .map(|issue| issue.id)
                                    .collect()
                            })
                            .unwrap_or_default(),
                    };
                    per_epic_map.per_status_issues.push(per_status_map);
                }
                per_epic_map
            })
            .collect()
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
