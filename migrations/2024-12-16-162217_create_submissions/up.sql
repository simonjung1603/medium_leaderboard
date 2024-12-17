-- Your SQL goes here
CREATE TABLE `submissions`(
	`guid` TEXT NOT NULL PRIMARY KEY,
	`realname` TEXT NOT NULL,
	`username` TEXT NOT NULL,
	`latest_published_version` TEXT NOT NULL,
	`latest_published_at` BigInt NOT NULL,
	`clap_count` INTEGER NOT NULL,
	`title` TEXT NOT NULL,
	`img_id` TEXT NOT NULL,
	`word_count` INTEGER NOT NULL
);

