# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.


## 0.4.0 - 2024-07-05
#### Bug Fixes
- **(macros)** default controller methods output to response type - (b736094) - elcharitas
- **(routes)** grouped routes resolving with blank screen issue - (c0c890c) - elcharitas
- **(routes)** sync provider impl to support `http_method` - (7bebcfe) - elcharitas
- **(routes)** temp drop support for http_method - (8eee401) - elcharitas
- **(routes)** route registry and handler discover - (478ac5b) - elcharitas
- **(routes)** stabilize server routing - (1fa1f23) - elcharitas
- **(routes)** allow for grouped routes via controllers - (2cea892) - elcharitas
- **(routes)** remove register from route_get macro - (f64af6c) - elcharitas
- **(routes)** allow one route registration per controller - (829b7b4) - elcharitas
- **(routing)** remove clutters and enforce impls - (69218c7) - elcharitas
- **(routing)** middleware works - (51d6385) - elcharitas
- **(sample)** force consistent return type - (2088f5a) - elcharitas
- restructure dto and validation (#109) - (4dde061) - elcharitas
- improve macro outputs to improve perf (#108) - (456f98e) - elcharitas
- generics in macros (#107) - (04adad1) - elcharitas
- response to bytes parse improvement (#104) - (d4c47ab) - elcharitas
- use `serde_json` in context value parsing (#103) - (a9d20f9) - elcharitas
- resolve empty request body in hyper platforms (#93) - (d9f664e) - elcharitas
- improve controller/injectable stability (#92) - (01d0511) - elcharitas
- improve platforms stability (#90) - (79efeb7) - elcharitas
- named route paths discovery (#82) - (b36c947) - elcharitas
- use context over request - (30a1189) - elcharitas
#### Documentation
- add more info for traits - (d43af47) - elcharitas
- add license + improve contrib docs - (a0eecae) - elcharitas
- update readme with more information - (d0812bc) - elcharitas
#### Features
- **(structure)** use crates to contain all crates - (647e4b7) - elcharitas
- setup docs website (#110) - (230d43e) - elcharitas
- add support for mutable transformation (#111) - (1fc95b2) - elcharitas
- platforms crates + shuttle support (#105) - (0cc8e1b) - elcharitas
- injectable init attribute (#102) - (7391645) - elcharitas
- add support for Dto attribute macro (#100) - (640c0bf) - elcharitas
- add support for route path prefixes in controllers (#95) - (a622518) - elcharitas
- add support for validation (#94) - (1d3a2f8) - elcharitas
- implement match strategy for hyper platform - (23a2cdc) - elcharitas
- create hyper platform - (2e83902) - elcharitas

- - -

## 0.3.1 - 2024-05-13
### Packages
- ngyn_macros locked to ngyn_macros-0.3.1
- ngyn locked to ngyn-0.3.1
- ngyn_shared locked to ngyn_shared-0.3.1
- ngyn_cli locked to ngyn_cli-0.3.1
#### Features
- add support for transducer transformers (#78) - (7a77a3b) - elcharitas
- support grouped routes gates (#73) - (80bc126) - elcharitas

- - -

## 0.3.0 - 2023-12-27
### Packages
- ngyn_macros locked to ngyn_macros-0.3.0
- ngyn locked to ngyn-0.3.0
- ngyn_shared locked to ngyn_shared-0.3.0
- ngyn_cli locked to ngyn_cli-0.3.0
### Global changes
#### Bug Fixes
- re-export common modules into prelude (#75) - (e653403) - elcharitas
- improve response body peeking and conversion (#74) - (0322480) - elcharitas
-  make requests partially mutable (#72) - (311bf5a) - elcharitas
- cog action/config (#59) - (076744a) - elcharitas
- improve controller/injectable patterns (#56) - (c0314bc) - elcharitas
- use optional catch all routes in vercel app (#54) - (3e52f62) - elcharitas
- use less ambiguous name (#44) - (b6ef3f1) - elcharitas
- status/set_status fixes (#40) - (242fc15) - elcharitas
- use ureq in weather api sample (#30) - (00489b2) - elcharitas
- further macro improvements (#28) - (8ef8bbc) - elcharitas
- basic app deployment (#23) - (dfce88e) - elcharitas
- ngyn version in basic app (#21) - (0bd8b7a) - elcharitas
#### Features
- controller/routes macro v2 (#70) - (16fd433) - elcharitas
- improve request processing and handling (#68) - (b497899) - elcharitas
- add platform macro (#67) - (f54e955) - elcharitas
- support body returns to match specs (#65) - (7fc7233) - elcharitas
- add feature flag for tide platform engine (#43) - (2061c37) - elcharitas
- proper response handling (#33) - (cd46101) - elcharitas
- add support for route specific gates (#26) - (70df61f) - elcharitas
- add support for gates and middlewares (#19) - (966087e) - elcharitas
- add support for vercel service (#24) - (175a06b) - elcharitas
- add support for release workflow (#16) - (537eede) - elcharitas
- add support for module interop (#13) - (fc75be9) - elcharitas
- add more http methods support to engine (#12) - (067ce21) - elcharitas
#### Miscellaneous Chores
- **(bump)** ngyn@0.2.0 -> ngyn@0.2.1 (#36) - (ab3e43e) - *elcharitas*
- **(bump)** ngyn_macros@v0.2.1 -> ngyn_macros@v0.2.2 (#35) - (f0ecded) - *elcharitas*
- **(version)** ngyn-0.3.0 - (44af6cf) - elcharitas
- **(version)** 0.2.8 - (1a3bff5) - elcharitas
- update project readme - (f42ebcc) - elcharitas
- add pre-bump script (#64) - (f1473d9) - elcharitas
- update sample controllers - (d88166d) - elcharitas
- sync cargo.toml - (bf38fc9) - elcharitas
- remove more fields from cog.toml - (99e4555) - elcharitas
- clean cog.toml unused configs - (c00e912) - elcharitas
- Update release workflow and add cog.toml (#55) - (420e66c) - elcharitas
- bump version to 0.2.7 - (19dfb6b) - elcharitas
- rename body to match specs (#53) - (9cfd24a) - elcharitas
- update basic_app example - (43adcbb) - elcharitas
- bump version to 0.2.6 - (e016500) - elcharitas
- bump version to 0.2.5 (#48) - (cf04092) - elcharitas
- rename ngyn::server to ngyn::platforms (#47) - (c4eeb65) - elcharitas
- bump version to 0.2.4 (#41) - (0b36fc1) - elcharitas
- bump version to 0.2.3 - (b3c0c20) - elcharitas
- bump versions to 0.2.2 (#39) - (452d3f6) - elcharitas
- update cargo.lock - (67602ce) - elcharitas
- cleanup install scripts (#31) - (7b516e4) - elcharitas
- update vercel example (#29) - (c9ec76e) - elcharitas
- disable windows test suite - (5e82a06) - elcharitas
- bump crates version - (ada1c92) - elcharitas
- bump crates version number (#22) - (d55b86a) - elcharitas
- update readme information (#20) - (138a66f) - elcharitas
- revamp weather api using reqwest + tokio (#18) - (db501df) - elcharitas
- add docker file (#17) - (46ab603) - elcharitas
- lean out macro outputs (#15) - (a944f92) - elcharitas
- add sample weather app (#14) - (3da0030) - elcharitas
- stabilize formats + repo url (#7) - (c9b1088) - elcharitas
- do not allow dead code (#6) - (a7471b9) - elcharitas

- - -

## 0.2.8 - 2023-11-26
### Packages
- ngyn_shared locked to ngyn_shared-0.2.8
- ngyn_macros locked to ngyn_macros-0.2.8
- ngyn_cli locked to ngyn_cli-0.1.0
- ngyn locked to ngyn-0.2.8
### Global changes

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).