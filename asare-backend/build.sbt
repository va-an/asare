import Dependencies._

scalaVersion := "2.13.4"

ThisBuild / organization := "io.vaan.asare"
ThisBuild / scalaVersion := "2.13.4"
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
    .dependsOn(
      org.typelevel.`cats-core`,
      org.typelevel.`cats-effect`,
    )

lazy val root =
  project
    .in(file("."))
    .aggregate(
      core
    )
