# probe-api
is a api to probe another applications

# To do

- Adjust in dockerfile
- Create error and providers to send notifications when the requests failed


## FAQ

```bash
curl -X POST http://localhost:8080/health \
     -H "Content-Type: application/json" \
     -d 'google.com'
```
