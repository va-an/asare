import Dependencies._

Global / cancelable := true
Global / onChangedBuildSource := ReloadOnSourceChanges

ThisBuild / organization := "io.vaan"
ThisBuild / scalaVersion := "2.13.6"

ThisBuild / scalacOptions ++= Seq(
  "-deprecation",
  "-feature",
  "-language:_",
  "-unchecked",
  "-Wvalue-discard",
  "-Xfatal-warnings",
  "-Ymacro-annotations"
)

lazy val backend =
  (project in file("backend"))
    .settings(
      version := "1.2.1",
      libraryDependencies ++= Seq(
        Libraries.cats,
        Libraries.catsEffect,
        Libraries.circeGeneric,
        Libraries.cirisCore,
        Libraries.cirisRefined,
        Libraries.scalaTest,
        Libraries.http4sDsl,
        Libraries.http4sServer,
        Libraries.http4sCirce,
        Libraries.log4catsSlf4j,
        Libraries.logback,
        Libraries.refined
      )
    )
    .settings(commonSettings: _*)

lazy val `telegram-bot` = 
  (project in file("telegram-bot"))
    .settings(
      version := "1.0.2",
      libraryDependencies ++= Seq(
        Libraries.cats,
        Libraries.catsEffect withRevision "2.5.0",
        Libraries.canoe,
        Libraries.log4catsSlf4j withRevision "1.2.2",
        Libraries.logback,
        Libraries.cirisCore withRevision "1.2.1",
        Libraries.cirisRefined withRevision "1.2.1",
        Libraries.refined,
        Libraries.http4sClient withRevision "0.21.22",
        Libraries.http4sCirce withRevision "0.21.22",
        Libraries.circeGeneric
      )
    )
    .settings(commonSettings: _*)

lazy val `load-test` = 
  (project in file("load-test"))
    .settings(
      libraryDependencies ++= Seq(
        Libraries.gatling,
        Libraries.gatlingHighcharts
      )
    )
    .settings(commonSettings: _*)
    .enablePlugins(GatlingPlugin)

lazy val commonSettings = Seq(
  assembly / test := {},
  addCompilerPlugin(Libraries.contextApplied)
)