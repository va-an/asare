import sbt._

object Dependencies {
  object Versions {
    val cats           = "2.5.0"
    val catsEffect     = "2.4.1"
    val http4s         = "0.21.22"
    val scalaTest      = "3.2.5"
    val contextApplied = "0.1.4"
    val circe          = "0.13.0"
    val log4cats       = "1.2.2"
    val logback        = "1.2.3"
    val gatling        = "3.5.1"
  }

  object Libraries {
    def http4s(artifact: String): ModuleID   = "org.http4s"    %% artifact % Versions.http4s
    def circe(artifact: String): ModuleID    = "io.circe"      %% artifact % Versions.circe
    def log4cats(artifact: String): ModuleID = "org.typelevel" %% artifact % Versions.log4cats

    val cats       = "org.typelevel" %% "cats-core"   % Versions.cats
    val catsEffect = "org.typelevel" %% "cats-effect" % Versions.catsEffect

    // http4s
    val http4sDsl    = http4s("http4s-dsl")
    val http4sServer = http4s("http4s-blaze-server")
    val http4sClient = http4s("http4s-blaze-client")
    val http4sCirce  = http4s("http4s-circe")

    // logging
    val log4catsCore  = log4cats("log4cats-core")
    val log4catsSlf4j = log4cats("log4cats-slf4j")
    val logback       = "ch.qos.logback" % "logback-classic" % Versions.logback

    val circeGeneric = circe("circe-generic")

    val scalaTest = "org.scalatest" %% "scalatest" % Versions.scalaTest

    // format: off
    val gatlingHighcharts = "io.gatling.highcharts" % "gatling-charts-highcharts" % Versions.gatling % "it"
    val gatling           = "io.gatling"            % "gatling-test-framework"    % Versions.gatling % "it"
    // format: on

    val contextApplied = "org.augustjune" %% "context-applied" % Versions.contextApplied
  }
}
