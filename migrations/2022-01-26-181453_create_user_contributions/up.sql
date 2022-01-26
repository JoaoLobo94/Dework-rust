-- Your SQL goes here
CREATE TABLE public.user_contributions (
    id integer GENERATED ALWAYS AS IDENTITY,
    company_id integer NOT NULL,
    contribution_id integer NOT NULL,
    balance integer,
    vote_balance integer,
    PRIMARY KEY (id),
    FOREIGN KEY (company_id) REFERENCES public.companies(id),
    FOREIGN KEY (contribution_id) REFERENCES public.contributions(id)
);