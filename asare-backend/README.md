Build backend jar-file:

```shell
$ sbt 'project backend; assembly'
```

---
Run app:

```shell
$ java -jar -Xms256m -Xmx256m -DapiPort="8092" backend/target/scala-2.13/backend-assembly-1.1.0.jar
```

---
Run load test (in sbt console):
```
eval System.setProperty("users", "3"); gatling-it:testOnly io.vaan.asare.LoadSimulation
```

Show report:
```shell
$ sbt '; project load-test; gatling-it:lastReport'
```