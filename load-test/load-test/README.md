Run load test (in sbt console):
```
eval System.setProperty("users", "3"); gatling-it:testOnly io.vaan.asare.LoadSimulation
```

Show report:
```shell
$ sbt '; project load-test; gatling-it:lastReport'
```
