======================================================== Database: RustyServer

-- DROP DATABASE IF EXISTS "RustyServer";

CREATE DATABASE "RustyServer"
    WITH
    OWNER = postgres
    ENCODING = 'UTF8'
    LC_COLLATE = 'French_France.1252'
    LC_CTYPE = 'French_France.1252'
    LOCALE_PROVIDER = 'libc'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;


-------------------------------------------------------- Table: public.users

-- DROP TABLE IF EXISTS public.users;

CREATE TABLE IF NOT EXISTS public.users
(
    id bigint NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    hwid bit varying NOT NULL,
    username character varying(64) COLLATE pg_catalog."default" NOT NULL,
    password character varying(512) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.users
    OWNER to postgres;


-------------------------------------------------------- Table: public.history

-- DROP TABLE IF EXISTS public.history;

CREATE TABLE IF NOT EXISTS public.history
(
    "idData" bigint NOT NULL DEFAULT nextval('"history_idData_seq"'::regclass),
    "timestamp" bigint NOT NULL,
    author character varying(64) COLLATE pg_catalog."default" NOT NULL,
    receiver character varying(64) COLLATE pg_catalog."default" NOT NULL,
    "encryptData" character varying COLLATE pg_catalog."default" NOT NULL,
    "encryptKey" character varying(128) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT history_pkey PRIMARY KEY ("idData")
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.history
    OWNER to postgres;