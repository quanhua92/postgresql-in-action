CREATE TABLE episode (
    id bigint GENERATED BY DEFAULT AS IDENTITY,
    created_date timestamp(6) with time zone NOT NULL,
    modified_date timestamp(6) with time zone NOT NULL,
    episode_number integer NOT NULL,
    original_title varchar(255) NOT NULL,
    release_date date,
    runtime numeric(38, 2) NOT NULL,
    title varchar(255) NOT NULL,
    season_id bigint NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE movie (
    id bigint GENERATED BY DEFAULT AS IDENTITY,
    created_date timestamp(6) with time zone NOT NULL,
    modified_date timestamp(6) with time zone NOT NULL,
    available_globally boolean,
    locale varchar(10),
    original_title varchar(255),
    release_date date,
    runtime bigint,
    title varchar(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE season (
    id bigint GENERATED BY DEFAULT AS IDENTITY,
    created_date timestamp(6) with time zone NOT NULL,
    modified_date timestamp(6) with time zone NOT NULL,
    original_title varchar(255),
    release_date date,
    runtime bigint,
    season_number integer,
    title varchar(255) NOT NULL,
    tv_show_id bigint,
    PRIMARY KEY (id)
);

CREATE TABLE tv_show (
    id bigint GENERATED BY DEFAULT AS IDENTITY,
    created_date timestamp(6) with time zone NOT NULL,
    modified_date timestamp(6) with time zone NOT NULL,
    available_globally boolean,
    locale varchar(10),
    original_title varchar(255),
    release_date date,
    title varchar(255) NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT idx_tv_show_title UNIQUE (title)
);

CREATE TABLE view_summary (
    id bigint GENERATED BY DEFAULT AS IDENTITY,
    created_date timestamp(6) with time zone NOT NULL,
    modified_date timestamp(6) with time zone NOT NULL,
    cumulative_weeks_in_top10 integer,
    duration varchar(20) NOT NULL CHECK (duration IN ('WEEKLY', 'SEMI_ANNUALLY')),
    end_date date NOT NULL,
    hours_viewed integer NOT NULL,
    start_date date NOT NULL,
    view_rank integer,
    views integer,
    movie_id bigint,
    season_id bigint,
    PRIMARY KEY (id)
);

CREATE INDEX fk_episode_season_id ON episode (season_id);

CREATE INDEX idx_movie_title_runtime ON movie (title, runtime);

CREATE INDEX idx_season_title_runtime ON season (title, runtime);

CREATE INDEX fk_season_tv_show_id ON season (tv_show_id);

ALTER TABLE IF EXISTS episode
    ADD CONSTRAINT fk_episode_season_id FOREIGN KEY (season_id) REFERENCES season;

ALTER TABLE IF EXISTS season
    ADD CONSTRAINT fk_season_tv_show_id FOREIGN KEY (tv_show_id) REFERENCES tv_show;

ALTER TABLE IF EXISTS view_summary
    ADD CONSTRAINT fk_view_summary_movie_id FOREIGN KEY (movie_id) REFERENCES movie;

ALTER TABLE IF EXISTS view_summary
    ADD CONSTRAINT fk_view_summary_season_id FOREIGN KEY (season_id) REFERENCES season;

