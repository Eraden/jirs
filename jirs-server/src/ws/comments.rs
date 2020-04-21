use actix::Addr;
use actix_web::web::Data;
use futures::executor::block_on;

use jirs_data::{CommentId, CreateCommentPayload, IssueId, UpdateCommentPayload, WsMsg};

use crate::db::DbExecutor;
use crate::ws::{current_user, WebSocketActor, WsHandler, WsResult};

pub struct LoadIssueComments {
    pub issue_id: IssueId,
}

impl WsHandler<LoadIssueComments> for WebSocketActor {
    fn handle_msg(&mut self, msg: LoadIssueComments, _ctx: Self::Context) -> WsResult {
        self.require_user()?;

        let comments = match block_on(self.db.send(crate::db::comments::LoadIssueComments {
            issue_id: msg.issue_id,
        })) {
            Ok(Ok(comments)) => comments.into_iter().map(|c| c.into()).collect(),
            _ => return Ok(None),
        };

        Ok(Some(WsMsg::IssueCommentsLoaded(comments)))
    }
}

impl WsHandler<CreateCommentPayload> for WebSocketActor {
    fn handle_msg(&mut self, mut msg: CreateCommentPayload, _ctx: Self::Context) -> WsResult {
        use crate::db::comments::CreateComment;

        let user_id = self.require_user()?.id;
        if msg.user_id.is_none() {
            msg.user_id = Some(user_id);
        }
        let issue_id = msg.issue_id;
        match block_on(self.db.send(CreateComment {
            user_id,
            issue_id,
            body: msg.body,
        })) {
            Ok(Ok(_)) => (),
            _ => return Ok(None),
        };
        self.handle_msg(LoadIssueComments { issue_id })
    }
}

impl WsHandler<UpdateCommentPayload> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateCommentPayload, _ctx: Self::Context) -> WsResult {
        use crate::db::comments::UpdateComment;

        info!("{:?}", msg);
        let user_id = self.require_user()?.id;

        let UpdateCommentPayload {
            id: comment_id,
            body,
        } = msg;

        let issue_id = match block_on(self.db.send(UpdateComment {
            comment_id,
            user_id,
            body,
        })) {
            Ok(Ok(comment)) => comment.issue_id,
            _ => return Ok(None),
        };
        if let Some(v) = self.handle_msg(LoadIssueComments { issue_id })? {
            self.broadcast(&v);
        }
        Ok(None)
    }
}

pub struct DeleteComment {
    pub comment_id: CommentId,
}

impl WsHandler<DeleteComment> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteComment, _ctx: Self::Context) -> WsResult {
        use crate::db::comments::DeleteComment;

        let user_id = self.require_user()?.id;

        let m = DeleteComment {
            comment_id: msg.comment_id,
            user_id,
        };
        match block_on(self.db.send(m)) {
            Ok(Ok(_)) => (),
            _ => return Ok(None),
        };

        Ok(Some(WsMsg::CommentDeleted(msg.comment_id)))
    }
}
