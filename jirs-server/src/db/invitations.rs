use actix::{Handler, Message};
use diesel::pg::Pg;
use diesel::prelude::*;

use jirs_data::{
    EmailString, Invitation, InvitationId, InvitationState, ProjectId, User, UserId, UserRole,
    UsernameString,
};

use crate::db::DbExecutor;
use crate::errors::ServiceErrors;

pub struct ListInvitation {
    pub user_id: UserId,
}

impl Message for ListInvitation {
    type Result = Result<Vec<Invitation>, ServiceErrors>;
}

impl Handler<ListInvitation> for DbExecutor {
    type Result = Result<Vec<Invitation>, ServiceErrors>;

    fn handle(&mut self, msg: ListInvitation, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = invitations
            .filter(invited_by_id.eq(msg.user_id))
            .filter(state.ne(InvitationState::Accepted))
            .order_by(state.asc())
            .then_order_by(updated_at.desc());
        debug!("{}", diesel::debug_query::<Pg, _>(&query).to_string());
        query
            .load(conn)
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)
    }
}

pub struct CreateInvitation {
    pub user_id: UserId,
    pub project_id: ProjectId,
    pub email: EmailString,
    pub name: UsernameString,
    pub role: UserRole,
}

impl Message for CreateInvitation {
    type Result = Result<Invitation, ServiceErrors>;
}

impl Handler<CreateInvitation> for DbExecutor {
    type Result = Result<Invitation, ServiceErrors>;

    fn handle(&mut self, msg: CreateInvitation, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;

        let query = diesel::insert_into(invitations).values((
            name.eq(msg.name),
            email.eq(msg.email),
            state.eq(InvitationState::Sent),
            project_id.eq(msg.project_id),
            invited_by_id.eq(msg.user_id),
            role.eq(msg.role),
        ));
        debug!("{}", diesel::debug_query::<Pg, _>(&query).to_string());
        query
            .get_result(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))
    }
}

pub struct DeleteInvitation {
    pub id: InvitationId,
}

impl Message for DeleteInvitation {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<DeleteInvitation> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: DeleteInvitation, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let query = diesel::delete(invitations).filter(id.eq(msg.id));
        debug!("{}", diesel::debug_query::<Pg, _>(&query).to_string());
        query
            .execute(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;
        Ok(())
    }
}

pub struct RevokeInvitation {
    pub id: InvitationId,
}

impl Message for RevokeInvitation {
    type Result = Result<(), ServiceErrors>;
}

impl Handler<RevokeInvitation> for DbExecutor {
    type Result = Result<(), ServiceErrors>;

    fn handle(&mut self, msg: RevokeInvitation, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;
        let query = diesel::update(invitations)
            .set((
                state.eq(InvitationState::Revoked),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .filter(id.eq(msg.id));
        debug!("{}", diesel::debug_query::<Pg, _>(&query).to_string());
        query
            .execute(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;
        Ok(())
    }
}

pub struct AcceptInvitation {
    pub id: InvitationId,
}

impl Message for AcceptInvitation {
    type Result = Result<User, ServiceErrors>;
}

impl Handler<AcceptInvitation> for DbExecutor {
    type Result = Result<User, ServiceErrors>;

    fn handle(&mut self, msg: AcceptInvitation, _ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;

        let conn = &self
            .pool
            .get()
            .map_err(|_| ServiceErrors::DatabaseConnectionLost)?;

        let query = invitations.find(msg.id);
        debug!("{}", diesel::debug_query::<Pg, _>(&query).to_string());
        let invitation: Invitation = query
            .first(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;

        if invitation.state == InvitationState::Revoked {
            return Err(ServiceErrors::DatabaseQueryFailed(
                "This invitation is no longer valid".to_string(),
            ));
        }

        let query = diesel::update(invitations)
            .set((
                state.eq(InvitationState::Accepted),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .filter(id.eq(invitation.id))
            .filter(state.eq(InvitationState::Sent));
        debug!("{}", diesel::debug_query::<Pg, _>(&query).to_string());
        query
            .execute(conn)
            .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;

        let user: User = {
            use crate::schema::users::dsl::*;

            let query = users
                .filter(name.eq(invitation.name).and(email.eq(invitation.email)))
                .limit(1);
            debug!("{}", diesel::debug_query::<Pg, _>(&query));
            query
                .first(conn)
                .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?
        };
        {
            use crate::schema::user_projects::dsl::*;

            let query = diesel::insert_into(user_projects).values((
                user_id.eq(user.id),
                project_id.eq(invitation.project_id),
                role.eq(invitation.role),
            ));
            debug!("{}", diesel::debug_query::<Pg, _>(&query));
            query
                .execute(conn)
                .map_err(|e| ServiceErrors::DatabaseQueryFailed(format!("{}", e)))?;
        };

        Ok(user)
    }
}
