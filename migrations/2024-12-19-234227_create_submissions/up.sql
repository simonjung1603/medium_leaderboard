CREATE TABLE "submissions"(
	"guid" TEXT NOT NULL PRIMARY KEY,
	"realname" TEXT NOT NULL,
	"username" TEXT NOT NULL,
	"latest_published_version" TEXT NOT NULL,
	"latest_published_at" BIGINT NOT NULL,
	"clap_count" INTEGER NOT NULL,
	"title" TEXT NOT NULL,
	"img_id" TEXT NOT NULL,
	"word_count" INTEGER NOT NULL,
	"clap_count_last_updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"details_last_updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

