ThisBuild / scalaVersion := "2.12.6"
ThisBuild / organization := "eu.vgrigoriu"

lazy val notQuiteLisp = (project in file("."))
  .settings(
    name := "Not Quite Lisp",
    libraryDependencies += "org.scalatest" %% "scalatest" % "3.0.5" % Test,
  )

