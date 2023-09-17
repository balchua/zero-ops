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