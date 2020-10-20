CREATE TABLE public.users
(
  id varchar(36) DEFAULT uuid_generate_v4() NOT NULL ,
  email varchar(255) NOT NULL ,
  "password" varchar(255) NOT NULL ,
  CONSTRAINT pk_users_id PRIMARY KEY ( id )
);