Build backend jar-file:

```shell
$ sbt 'project backend; assembly'
```

---
Run load test (in sbt console):
```
eval System.setProperty("users", "3"); gatling-it:testOnly io.vaan.asare.LoadSimulation
```

Show report:
```shell
$ sbt '; project loadtest; gatling-it:lastReport'
```