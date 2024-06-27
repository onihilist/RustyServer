-- Database: RustyDB

-- DROP DATABASE IF EXISTS "RustyDB";

CREATE DATABASE "RustyDB"
    WITH
    OWNER = postgres
    ENCODING = 'UTF8'
    LC_COLLATE = 'French_France.1252'
    LC_CTYPE = 'French_France.1252'
    LOCALE_PROVIDER = 'libc'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;



    -- SCHEMA: public

    -- DROP SCHEMA IF EXISTS public ;

    CREATE SCHEMA IF NOT EXISTS public
        AUTHORIZATION pg_database_owner;

    COMMENT ON SCHEMA public
        IS 'standard public schema';

    GRANT USAGE ON SCHEMA public TO PUBLIC;

    GRANT ALL ON SCHEMA public TO pg_database_owner;


    
  -- Table: public.users

  -- DROP TABLE IF EXISTS public.users;

  CREATE TABLE IF NOT EXISTS public.users
  (
      "userID" integer NOT NULL DEFAULT nextval('"users_userID_seq"'::regclass),
      hwid character varying(256) COLLATE pg_catalog."default" NOT NULL,
      username character varying(64) COLLATE pg_catalog."default" NOT NULL,
      pass character varying(512) COLLATE pg_catalog."default" NOT NULL,
      CONSTRAINT users_pkey PRIMARY KEY ("userID")
  )

  TABLESPACE pg_default;

  ALTER TABLE IF EXISTS public.users
      OWNER to postgres;