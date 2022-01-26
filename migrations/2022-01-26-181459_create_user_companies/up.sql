-- Your SQL goes here
CREATE TABLE public.user_companies (
    id integer GENERATED ALWAYS AS IDENTITY,
    company_id integer NOT NULL,
    user_id integer NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (company_id) REFERENCES public.companies(id),
    FOREIGN KEY (user_id) REFERENCES public.users(id)
);
