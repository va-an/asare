import Dependencies._

ThisBuild / organization := "io.vaan"
ThisBuild / scalaVersion := "2.13.5"
ThisBuild / version := "0.0.1-SNAPSHOT"

ThisBuild / scalacOptions ++= Seq(
  "-deprecation",
  "-feature",
  "-language:_",
  "-unchecked",
  "-Wvalue-discard",
  "-Xfatal-warnings",
  "-Ymacro-annotations"
)

lazy val core =
  project
    .in(file("core"))
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
      mainClass in assembly := Some("io.vaan.asare.Main")
    )
    .settings(commonSettings: _*)

lazy val `asare-backend` =
  project
    .in(file("."))
    .aggregate(
      core
    )

lazy val commonSettings = Seq(
  version := "1.0.2",
  organization := "io.vaan",
  scalaVersion := "2.13.5",
  test in assembly := {},
  addCompilerPlugin(Libraries.contextApplied)
)