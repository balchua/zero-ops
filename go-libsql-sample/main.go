package main

import (
	"context"
	"database/sql"
	"fmt"
	"os"

	_ "github.com/libsql/libsql-client-go/libsql"
	"github.com/oklog/ulid/v2"
)

var dbUrl = "http://127.0.0.1:8080"

func exec(ctx context.Context, db *sql.DB, stmt string, args ...any) sql.Result {
	res, err := db.ExecContext(ctx, stmt, args...)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to execute statement %s: %s", stmt, err)
		os.Exit(1)
	}
	return res
}

func main() {
	db, err := sql.Open("libsql", dbUrl)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to open db %s: %s", dbUrl, err)
		os.Exit(1)
	}
	ctx := context.Background()
	exec(ctx, db, "CREATE TABLE IF NOT EXISTS counter(country TEXT, city TEXT, value INT, PRIMARY KEY(country, city)) WITHOUT ROWID")
	exec(ctx, db, "CREATE TABLE IF NOT EXISTS test(id TEXT, name TEXT, value TEXT, PRIMARY KEY(id))")
	exec(ctx, db, `CREATE TABLE IF NOT EXISTS Instances (
		[InstanceID] TEXT PRIMARY KEY NOT NULL,
		[ExecutionID] TEXT NOT NULL,
		[Name] TEXT NOT NULL, -- the type name of the orchestration or entity
		[Version] TEXT NULL, -- the version of the orchestration (optional)
		[RuntimeStatus] TEXT NOT NULL,
		[CreatedTime] DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
		[LastUpdatedTime] DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
		[CompletedTime] DATETIME NULL,
		[LockedBy] TEXT NULL,
		[LockExpiration] DATETIME NULL,
		[Input] TEXT NULL,
		[Output] TEXT NULL,
		[CustomStatus] TEXT NULL,
		[FailureDetails] BLOB NULL,
		[ParentInstanceID] TEXT NULL
	);`)
	id := ulid.Make()
	fmt.Println(id)
	exec(ctx, db, "INSERT INTO test(id, name, value) VALUES(?, ?, ?)", id.String(), "test", "test")

}
