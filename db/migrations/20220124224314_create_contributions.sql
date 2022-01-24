-- migrate:up
CREATE TABLE public.contributions (
    id integer GENERATED ALWAYS AS IDENTITY,
    pull_request varchar NOT NULL,
    type varchar NOT NULL,
    merged boolean DEFAULT false,
    balance integer,
    vote_balance integer,
    story_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT (now()),
    PRIMARY KEY (id),
    FOREIGN KEY (story_id) REFERENCES public.companies(id)
);

-- migrate:down
DROP TABLE IF EXISTS contributions
