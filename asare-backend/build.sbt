import Dependencies._

Global / onChangedBuildSource := ReloadOnSourceChanges

ThisBuild / organization := "io.vaan"
ThisBuild / scalaVersion := "2.13.5"
ThisBuild / version := "1.1.0"

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
        Libraries.scalaTest,
        Libraries.http4sDsl,
        Libraries.http4sServer,
        Libraries.http4sCirce,
        Libraries.circeGeneric,
        Libraries.log4catsCore,
        Libraries.log4catsSlf4j,
        Libraries.logback
      ),
      assembly / mainClass := Some("io.vaan.asare.Main")
    )
    .settings(commonSettings: _*)

lazy val loadtest = 
  (project in file("loadtest"))
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