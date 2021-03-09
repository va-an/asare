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
        Libraries.http4sServer
      )
    )
    .settings(commonSettings: _*)

lazy val `asare-backend` =
  project
    .in(file("."))
    .aggregate(
      core
    )

lazy val commonSettings = Seq(
  addCompilerPlugin(Libraries.contextApplied)
)