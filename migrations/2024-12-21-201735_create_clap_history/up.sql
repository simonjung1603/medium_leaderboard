CREATE TABLE "clap_history"(
    "id" SERIAL PRIMARY KEY,
	"guid" TEXT NOT NULL REFERENCES submissions(guid),
	"clap_count" INTEGER NOT NULL,
	"timestamp" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

