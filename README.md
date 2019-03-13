# Port of the postcard sized pathtracer to Rust

![rendered result](https://user-images.githubusercontent.com/8337848/54014287-71a68780-417c-11e9-87ae-256728078823.png "Pathtracer output. samples_count = 16384")

This repository hosts the code belonging to this Medium article: [https://medium.com/@cfsamson/from-48s-to-5s-optimizing-a-350-line-pathtracer-in-rust-191ab4a1a412](https://medium.com/@cfsamson/from-48s-to-5s-optimizing-a-350-line-pathtracer-in-rust-191ab4a1a412)

I've added a branch called `max_opt` for PRs that try to optimize the code without caring about keeping similarities with the original. So far we've reduced runtime by another ~25% compared to the current master branch, so if you want the fastest version, or want to contribute to a branch where all changes that makes the code faster is accepted, checkout the `max_opt` branch.

# Contribute

If you have any changes or suggestions, please file a PR. We're not looking to change the algorithms here but show some of the Rust constructs that help you write performant code.

