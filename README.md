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

curl --parallel --parallel-immediate --parallel-max 3 --config websites.txt
```