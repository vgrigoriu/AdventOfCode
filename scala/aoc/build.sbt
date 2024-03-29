val scala3Version = "3.2.1"

lazy val root = project
  .in(file("."))
  .settings(
    name := "aoc2022",
    version := "0.1.0-SNAPSHOT",

    scalaVersion := scala3Version,

    libraryDependencies += "org.scalameta" %% "munit" % "0.7.29" % Test
  )

scalacOptions ++= Seq(
  "-deprecation",
  "-feature",
  "-Ysafe-init",
  "-unchecked",
  "-language:strictEquality",
  "-Werror",
  "-Wunused:all",
)