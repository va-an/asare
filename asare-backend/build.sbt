import Dependencies._

Global / onChangedBuildSource := ReloadOnSourceChanges

ThisBuild / organization := "io.vaan"
ThisBuild / scalaVersion := "2.13.5"
ThisBuild / version := "1.1.1"

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
      ),
      assembly / mainClass := Some("io.vaan.asare.Main")
    )
    .settings(commonSettings: _*)

lazy val `telegram-bot` = 
  (project in file("telegram-bot"))
    .settings(
      libraryDependencies ++= Seq(
        Libraries.canoe,
        Libraries.log4catsSlf4j % "1.2.2",
        Libraries.logback
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