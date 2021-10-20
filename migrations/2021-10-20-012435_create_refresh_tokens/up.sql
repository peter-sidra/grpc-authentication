-- Your SQL goes here
CREATE TABLE "refresh_tokens" (
	"id"	TEXT NOT NULL,
	"token"	TEXT NOT NULL,
	"user_id"	TEXT NOT NULL,
	FOREIGN KEY("user_id") REFERENCES "users"("id"),
	PRIMARY KEY("id")
);