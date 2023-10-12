import Database from "libsql";
import reader from "readline-sync";

const url = "http://localhost:8080";
// if (!url) {
//     throw new Error("Environment variable LIBSQL_URL is not set.");
// }
const authToken = process.env.LIBSQL_AUTH_TOKEN;

const options = { syncUrl: url, authToken: "" };
const db = new Database("../data/users.db", options);

// Using Date objects 
let start = Date.now();
db.sync();
console.log("User entries:");

try {
    const row_count = db.prepare("SELECT count(*) as count FROM address").get(1);
    console.log(row_count.count);
} catch (error) {
    console.log(error);
}


let end = Date.now();

console.log("Time taken: " + (end - start) + "ms");
// for (const row of rows) {
//     console.log(row.name + "(" + row.email + ")");
// }