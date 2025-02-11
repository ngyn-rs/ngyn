# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

## Unreleased
#### Bug Fixes

#### Features

#### Miscellaneous Chores

## 0.5.3 - 2025-01-14
#### Bug Fixes
- [#243](../../../../pull/243) **routing**: manually drop app state to prevent poisoning

## 0.5.2 - 2024-12-30
#### Bug Fixes
- [#229](../../../../pull/229) **core**: route handlers improvements

#### Features

#### Miscellaneous Chores
- [#229](../../../../pull/229) **core**: deprecate `NgynContext::response`


## 0.5.1 - 2024-12-21
#### Bug Fixes
- [#220](../../../../pull/220) **core**: static files don't show up on release binary

#### Miscellaneous Chores
- [#221](../../../../pull/221) **core**: improve str to Bytes transform performance

## 0.5.0 - 2024-12-16
#### Bug Fixes
- [#212](../../../../pull/212) **core**: static routes rendering as plain u8 text
- invalid values returned from params
- [#216](../../../../pull/216) **routing**: unwrap grouped routes
- [#208](../../../../pull/208) **core**: param and query transforms

#### Features
- [#192](../../../../pull/192) **routing**: add support for async handlers
- [#197](../../../../pull/197) **core**: add support for async middleware and gates
- [#202](../../../../pull/202) **routing**: add redirect handlers
- [#205](../../../../pull/205) **routing**: drop support for controlled routing
- [#206](../../../../pull/206) **routing**: add support for multipart formdata
- [#209](../../../../pull/209) **core**: new ngyn router
- [#214](../../../../pull/214) **platforms**: websockets implementation
- [#215](../../../../pull/215) **routing**: grouped routes and router

#### Miscellaneous Chores
- [#195](../../../../pull/195) **core**: add info to explain how arc to box works
- [#203](../../../../pull/203) **routing**: add reexports for handlers
- [#204](../../../../pull/204) **routing**: remove is_valid_route function
- [#213](../../../../pull/213) **core**: `ToBytes` implementation + Crates keywords
- [#207](../../../../pull/207) **core**: cleanup interpreter and async traits

## 0.3.0 - 2023-12-27
#### Bug Fixes
- re-export common modules into prelude (#75) - (e653403) - elcharitas
- broken test case in body - (d3885de) - elcharitas
- improve response body peeking and conversion (#74) - (0322480) - elcharitas
-  make requests partially mutable (#72) - (311bf5a) - elcharitas
- handle errors in abstract controllers (#71) - (7e6f29f) - elcharitas
- cargo check recommendations (#66) - (a695884) - elcharitas
- improve controller/injectable patterns (#56) - (c0314bc) - elcharitas
- use less ambiguous name (#44) - (b6ef3f1) - elcharitas
- update response values in vercel engine (#38) - (274e0e0) - elcharitas
- readme symlinking final fix - (5559c5b) - elcharitas
- sub crates readme symlinks (#37) - (0922eb3) - elcharitas
- handle uncaught clippy and test issues (#34) - (6d5ae62) - elcharitas
- further macro improvements (#28) - (8ef8bbc) - elcharitas
- address all clippy errors (#9) - (68f7c6e) - elcharitas
#### Features
- **(structure)** use crates to contain all crates - (647e4b7) - elcharitas
- extend response body (#69) - (476ec43) - elcharitas
- improve request processing and handling (#68) - (b497899) - elcharitas
- add platform macro (#67) - (f54e955) - elcharitas
- support body returns to match specs (#65) - (7fc7233) - elcharitas
- add feature flag for tide platform engine (#43) - (2061c37) - elcharitas
- proper response handling (#33) - (cd46101) - elcharitas
- add support for route specific gates (#26) - (70df61f) - elcharitas
- add support for gates and middlewares (#19) - (966087e) - elcharitas
- add support for vercel service (#24) - (175a06b) - elcharitas
- add support for module interop (#13) - (fc75be9) - elcharitas
- add support for async routes (#11) - (e5cdc19) - elcharitas
#### Miscellaneous Chores
- **(bump)** ngyn@0.2.0 -> ngyn@0.2.1 (#36) - (ab3e43e) - *elcharitas*
- **(version)** ngyn_shared-0.2.8 - (146e75e) - elcharitas
- clean out NgynResponse::new - (48a88f6) - elcharitas
- sync cargo.toml - (bf38fc9) - elcharitas
- bump version to 0.2.7 - (19dfb6b) - elcharitas
- rename body to match specs (#53) - (9cfd24a) - elcharitas
- bump version to 0.2.6 - (e016500) - elcharitas
- bump version to 0.2.5 (#48) - (cf04092) - elcharitas
- rename ngyn::server to ngyn::platforms (#47) - (c4eeb65) - elcharitas
- move engine traits to shared (#45) - (5484665) - elcharitas
- bump version to 0.2.4 (#41) - (0b36fc1) - elcharitas
- bump version to 0.2.3 - (b3c0c20) - elcharitas
- bump versions to 0.2.2 (#39) - (452d3f6) - elcharitas
- clean unused code or parts (#32) - (8c83312) - elcharitas
- bump crates version - (ada1c92) - elcharitas
- bump crates version number (#22) - (d55b86a) - elcharitas
- lean out macro outputs (#15) - (a944f92) - elcharitas
- stabilize formats + repo url (#7) - (c9b1088) - elcharitas
- Rename to ngyn (#4) - (c81adc9) - elcharitas
- add http specific methods to server (#1) - (a8ee7ec) - elcharitas
#### Refactoring
- NgynRequest and NgynResponse methods (#57) - (6223d6a) - elcharitas

- - -

## ngyn_shared-0.2.8 - 2023-11-26

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).