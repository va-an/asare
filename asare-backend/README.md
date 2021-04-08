Build jar-file:

```
$ sbt assembly
```

Run load test:
```
sbt
project loadtest
gatling-it:testOnly io.vaan.asare.LoadSimulation
```