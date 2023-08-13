# Zero ops

A simple service that is zero-ops.  It uses sqlite as its database.  
In order to maintain high availability, we will use [Litefs](https://fly.io/docs/litefs/) to replicate the data to read replicas.

[Reference implementation](https://github.com/biluohc/actixweb-sqlx-jwt/tree/actix-web-4.0)