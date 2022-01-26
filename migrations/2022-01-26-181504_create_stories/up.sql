-- Your SQL goes here
CREATE TABLE public.stories (
    id integer GENERATED ALWAYS AS IDENTITY,
    pull_request varchar NOT NULL,
    type varchar NOT NULL,
    merged boolean DEFAULT false,
    balance integer,
    vote_balance integer,
    company_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL DEFAULT (now()),
    PRIMARY KEY (id),
    FOREIGN KEY (company_id) REFERENCES public.companies(id)
);