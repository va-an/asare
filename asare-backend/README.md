Build jar-file:

```
$ sbt assembly
```

---
Run load test:
```
$ sbt -Dusers=10 '; project loadtest; gatling-it:testOnly io.vaan.asare.LoadSimulation'
```

Show report:
```
$ sbt '; project loadtest; gatling-it:lastReport'
```