Build jar-file:

```
$ sbt assembly
```

---
Run load test (in sbt console):
```
eval System.setProperty("users", "3"); gatling-it:testOnly io.vaan.asare.LoadSimulation
```

Show report:
```
$ sbt '; project loadtest; gatling-it:lastReport'
```