-- Add up migration script here
CREATE TABLE "user"(
id BIGINT GENERATED BY DEFAULT AS IDENTITY NOT NULL  PRIMARY KEY,
emial varchar(128) UNIQUE,
verified_email boolean DEFAULT false,
name varchar(128),
given_name varchar(128),
family_name varchar(128),
pitcure varchar(128),
locale varchar(128)
);