enablePlugins(ScalaJSPlugin)

scalaVersion := "2.13.4"

scalaJSUseMainModuleInitializer := true
jsEnv := new org.scalajs.jsenv.jsdomnodejs.JSDOMNodeJSEnv()

libraryDependencies += "org.scala-js" %%% "scalajs-dom" % "1.1.0"
libraryDependencies += "org.typelevel" %%% "cats-core" % "2.3.1"
