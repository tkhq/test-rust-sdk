# Local interoperability examples

These synthetic examples perform no network or cluster operations. Images use
`example.invalid` and are not runnable deployments. Do not apply the manifests.
The worker needs get/list access to namespace secrets. An ingress-facing gateway
must not receive that worker credential. The worker uses an explicit projected
token; Pod automount is disabled. JSON is also valid YAML 1.2.

This is an independent Cargo workspace; the SDK toolchain is unchanged. With
installed Rust 1.94, run from the repository root:

```sh
cargo +1.94 test --offline --manifest-path review-lab/Cargo.toml
cargo +1.94 build --offline --manifest-path review-lab/Cargo.toml
printf '%s' '{"type":"activity.completed","activityId":"fixture-1"}' | review-lab/target/debug/review-lab event
printf '%s' '{}' | review-lab/target/debug/review-lab options
printf '%s' '{"replicas":0}' | review-lab/target/debug/review-lab options
printf '%s' '{"replicas":3}' | review-lab/target/debug/review-lab options
review-lab/target/debug/review-lab deployment
```

If dependencies are not cached, omit `--offline` for dependency installation.
Feed actual producer JSON on stdin to `event` or `options`, including via a pipe
from a companion producer. A completed event returns processed=1 and its ID;
unknown types fail with nonzero exit. Absent replicas returns mode=default,
zero returns mode=disabled, and positive values return mode=explicit. Null,
negative, fractional and string replicas are rejected.

Copied automation is retained with `.disabled` extensions for this local example
series. Only the fixture workflow is enabled; it has read-only permissions and
runs no credentialed examples, releases, deployments or integration calls.
