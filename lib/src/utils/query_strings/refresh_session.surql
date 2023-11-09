-- BEGIN TRANSACTION;

LET $tok = UPDATE ONLY system_session_token MERGE {
    expires_at: time::from::unix(0),
} WHERE token = $refresh_token;

CREATE system_session_token CONTENT {
    session: $tok.session.id,
    token: rand::ulid(),
    expires_at: time::from::secs(time::unix() + 60 * 60 * 24 * (365 / 2)),
} RETURN token, expires_at, session.id, session.user.*;

-- COMMIT TRANSACTION;