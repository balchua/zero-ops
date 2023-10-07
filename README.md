# Zero ops

A simple low-ops service.  It uses sqlite as its database.
With a simple backup mechanism using [LiteStream]()

Based on 

### Test with parallel curl request

``` bash
cat <<EOF > websites.txt
url = http://localhost:3000/platform/1
url = http://localhost:3000/platform/2
url = http://localhost:3000/platform/1
url = http://localhost:3000/platform/2
url = http://localhost:3000/platform/1
url = http://localhost:3000/platform/2
url = http://localhost:3000/platform/1
url = http://localhost:3000/platform/10
EOF

curl --parallel --parallel-immediate --parallel-max 10 --config websites.txt
```

### Add event 

``` bash
curl --request PUT \
  --url http://localhost:3000/event/input/ \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data name=test2_123 \
  --data active=true \
  --data platform_id=2
```

## Sqld

### Enable bottomless

Automatically backs up to S3 compatible storage.  The following environment variables are required.

``` bash
LIBSQL_BOTTOMLESS_BUCKET=zero-ops # bucket name
LIBSQL_BOTTOMLESS_ENDPOINT='https://sgp1.digitaloceanspaces.com' # address can be overridden for local testing, e.g. with Minio
LIBSQL_BOTTOMLESS_AWS_SECRET_ACCESS_KEY=$ZEROOPS_SECRET_KEY
LIBSQL_BOTTOMLESS_AWS_ACCESS_KEY_ID=$ZEROOPS_ACCESS_KEY
LIBSQL_BOTTOMLESS_AWS_DEFAULT_REGION=sgp1
```

### Start sqld

``` bash
docker run --rm --name zero-ops --env-file s3.env -p 8080:8080 -v ./sqld-data:/var/lib/sqld -it ghcr.io/libsql/sqld:main /bin/sqld --enable-bottomless-replication -disable-default-namespace --checkpoint-interval-s 15 --http-listen-addr 0.0.0.0:8080
```
or

``` bash
docker run --rm --name zero-ops --env-file s3.env -p 8080:8080 -it ghcr.io/libsql/sqld:main /bin/sqld --enable-bottomless-replication --disable-default-namespace --checkpoint-interval-s 15
```
### Restore

``` bash
export AWS_ACCESS_KEY_ID=$LIBSQL_BOTTOMLESS_AWS_ACCESS_KEY_ID 
export AWS_SECRET_ACCESS_KEY=$LIBSQL_BOTTOMLESS_AWS_SECRET_ACCESS_KEY 
export AWS_DEFAULT_REGION=$LIBSQL_BOTTOMLESS_AWS_DEFAULT_REGION 
# returns an sqlite file
bottomless-cli --endpoint $LIBSQL_BOTTOMLESS_ENDPOINT -b $LIBSQL_BOTTOMLESS_BUCKET restore

```

### http based sql statements

``` bash
curl -d '{"statements": ["CREATE TABLE IF NOT EXISTS test4(rname)"]}' 127.0.0.1:8080
curl -d '{"statements": ["INSERT INTO test4 VALUES (\"test4\")"]}' 127.0.0.1:8080
curl -d '{"statements": ["SELECT count(*) FROM test4"]}' 127.0.0.1:8080
```
