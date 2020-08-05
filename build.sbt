val dottyVersion = "0.25.0"
scalaSource in Compile := baseDirectory.value / "src"

enablePlugins(GraalVMNativeImagePlugin)
graalVMNativeImageOptions ++= Seq(
        "--initialize-at-build-time",
        "-H:CCompilerOption=\"-I/home/rouli-freeman/Documents/NE\""
    )

lazy val root = project
    .in(file("."))
    .settings(
        name := "NE",
        version := "1.0.0",
        scalaVersion := dottyVersion,
        scalacOptions ++= Seq("-Yexplicit-nulls", "-Ycheck-init")
    )
addCommandAlias("native-image", "graalvm-native-image:packageBin")
javaHome := Some(file("~/graal"))
Global / onChangedBuildSource := ReloadOnSourceChanges
