load("@rules_jvm_external//:defs.bzl", "maven_install")

def java_dependencies():
    maven_install(
        artifacts = [
            "junit:junit:4.13",
            "org.openjdk.jmh:jmh-core:1.22",
            "org.openjdk.jmh:jmh-generator-annprocess:1.22",
        ],
        repositories = [
            "https://repo1.maven.org/maven2",
        ],
    )
