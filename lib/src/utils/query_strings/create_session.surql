BEGIN TRANSACTION;

LET $ses = CREATE ONLY system_session CONTENT {
    user: type::thing("system_user", $user_id),
};

CREATE ONLY system_session_token CONTENT {
    session: $ses.id,
    token: rand::ulid(),
    expires_at: time::from::secs(time::unix() + 60 * 60 * 24 * (365 / 2)),
} RETURN token, expires_at, session.id;

COMMIT TRANSACTION;