-- Table des Guilds
CREATE TABLE IF NOT EXISTS guilds (
    discord_guild_id BIGINT PRIMARY KEY,    -- ID de la guild (Discord)
    channel_id BIGINT,          -- ID de la guild (Discord)
    name TEXT,                              -- Nom de la guild
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Date de création
);

-- Table des utilisateurs
CREATE TABLE IF NOT EXISTS users (
    discord_user_id BIGINT PRIMARY KEY,  -- ID de l'utilisateur (Discord)
    username TEXT NOT NULL               -- Nom d'utilisateur
);

-- Table de liaison entre Guild et Users pour savoir quel utilisateur appartient à quelle guild
CREATE TABLE IF NOT EXISTS guild_users (
    discord_guild_id BIGINT,  -- ID de la guild
    discord_user_id BIGINT,   -- ID de l'utilisateur
    PRIMARY KEY (discord_guild_id, discord_user_id),  -- Clé primaire composée
    FOREIGN KEY (discord_guild_id) REFERENCES guilds(discord_guild_id) ON DELETE CASCADE,
    FOREIGN KEY (discord_user_id) REFERENCES users(discord_user_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS questions (
    question_id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    discord_guild_id BIGINT,  -- NULL si question globale
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (discord_guild_id) REFERENCES guilds(discord_guild_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS answers (
    answer_id SERIAL PRIMARY KEY,
    question_id INT NOT NULL,
    answered_by BIGINT NOT NULL,        -- L'utilisateur qui a répondu
    answer_user_id BIGINT NOT NULL,     -- L'utilisateur mentionné comme "réponse"
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (question_id) REFERENCES questions(question_id) ON DELETE CASCADE,
    FOREIGN KEY (answered_by) REFERENCES users(discord_user_id) ON DELETE CASCADE,
    FOREIGN KEY (answer_user_id) REFERENCES users(discord_user_id) ON DELETE CASCADE
);

CREATE INDEX idx_questions_guild ON questions(discord_guild_id);
CREATE INDEX idx_answers_question ON answers(question_id);
CREATE INDEX idx_answers_answered_by ON answers(answered_by);
