CREATE  TABLE public.todos ( 
	id 							 		 varchar(36) DEFAULT uuid_generate_v4() NOT NULL ,
	user_id              varchar(36)  NOT NULL ,
	content              text  NOT NULL ,
	"checked"            boolean DEFAULT false NOT NULL ,
	CONSTRAINT pk_todos_id PRIMARY KEY ( id )
 );