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
        org.typelevel.`cats-core`,
        org.typelevel.`cats-effect`,
        org.scalatest.scalatest,
        org.http4s.`http4s-dsl`,
        org.http4s.`http4s-blaze-server`
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
  addCompilerPlugin(org.augustjune.`context-applied`)
)