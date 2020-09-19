enablePlugins(ScalaJSPlugin)

scalaVersion := "2.13.1"

scalaJSUseMainModuleInitializer := true
jsEnv := new org.scalajs.jsenv.jsdomnodejs.JSDOMNodeJSEnv()

libraryDependencies += "org.scala-js" %%% "scalajs-dom" % "1.1.0"
