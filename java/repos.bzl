load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

def java_repositories():
    git_repository(
        name = "rules_jvm_external",
        remote = "https://github.com/bazelbuild/rules_jvm_external",
        # `commit` and `shallow_since` was given by first specifying:
        #     tag = "3.1"
        # and then following the debug messages given by Bazel.
        commit = "9aec21a7eff032dfbdcf728bb608fe1a02c54124",
        shallow_since = "1577467222 -0500",
    )
