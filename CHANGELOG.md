# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

## [0.70.2](https://github.com/boundaryml/baml/compare/0.70.1..0.70.2) - 2024-12-13

### Bug Fixes

- Remove log statements (#1230) - ([4bcdd19](https://github.com/boundaryml/baml/commit/4bcdd198f219cd016ee64cc6444dd62e69f796fb)) - hellovai
- Fix playground proxy related issues (#1228, #1229, #1237) - ([7384ba8](https://github.com/boundaryml/baml/commit/7384ba8cb5d1f012c50ddfb2a44a142ec9654397)) ([7bb6df4](https://github.com/boundaryml/baml/commit/7bb6df40fe37753b946ceeec6b30c4d9cdcc4ce7)) ([16054f5](https://github.com/boundaryml/baml/commit/16054f5f858dcaf80f013d466ceb9354c6a160b7)) - aaronvg

### DOCS

- deno run instead of dpx (#1225) - ([7c64299](https://github.com/boundaryml/baml/commit/7c642992cd7d52b7e7cd718542dfa68c41b5aab3)) - Jeffrey Konowitch
- Fix broken links (#1235) - ([859c699](https://github.com/boundaryml/baml/commit/859c6998cef7950d52cc3287f51d74106a58d89d)) - Samuel Lijin

### Features

- Support parsing primitive values from single-key objects (#1224) - ([935a190](https://github.com/boundaryml/baml/commit/935a190556d12077f961ce083723e7c1f816f387)) - revidious


## [0.70.1](https://github.com/boundaryml/baml/compare/0.70.0..0.70.1) - 2024-12-05

### Bug Fixes

- Make baml_py work with playwright/inspect (#1214) - ([6741999](https://github.com/boundaryml/baml/commit/674199992e21fb439a5c972c5868b6b3f106d267)) - Samuel Lijin
- Fix Python release pipeline (#1218) - ([bde634c](https://github.com/boundaryml/baml/commit/bde634cd6064784e77620f26f52202494fb659ec)) - Samuel Lijin

### Documentation

- Docs for LLM Clients paramaters updated (#1216) - ([6f99a28](https://github.com/boundaryml/baml/commit/6f99a28a918e557a75e2d763ac21ca587350adf4)) - hellovai


## [0.70.0](https://github.com/boundaryml/baml/compare/0.69.0..0.70.0) - 2024-12-04

### Bug Fixes

- Improvements for promptfiddle (#1201) - ([c6fb306](https://github.com/boundaryml/baml/commit/c6fb3067ce74f7864c8e071ed9ea3b3b1f69d00a)) - aaronvg
- Add vscode config to disable proxying (#1197) - ([c593284](https://github.com/boundaryml/baml/commit/c59328479a60847147d7141f0053fb208821d49a)) - aaronvg
- update lezer syntax for tests (#1199) - ([269ad9d](https://github.com/boundaryml/baml/commit/269ad9da5ca1dede5bf3d6a42f11f158cfe57dda)) - aaronvg
- Various playground fixes (#1202) - ([ce4f397](https://github.com/boundaryml/baml/commit/ce4f39737b88d2fcf27851ff8b230eda5a1e714b)) - aaronvg


### Documentation

- Add test-block constraints docs (#1198) - ([b566d4c](https://github.com/boundaryml/baml/commit/b566d4ceadab2bff0ae77765be63aadb4d3660d2)) - Greg Hale

### Features
- Fix azure client - ([9b57395](https://github.com/boundaryml/baml/commit/9b5739565b684c2179ac2ab24cabaa441a6269a7)) - hellovai
- Add new client paramters: allowed_roles, default_role, finish_reason_allow_list, finish_reason_deny_list (#1209) - ([9b57395](https://github.com/boundaryml/baml/commit/9b5739565b684c2179ac2ab24cabaa441a6269a7)) - hellovai


### Miscellaneous Chores
- cargo clippy (#1206) - ([c17e0da](https://github.com/boundaryml/baml/commit/c17e0da45db4188e0b0618d9e69f21220dc2fcff)) - Antonio Sarosi
- add colors to the CLI by default (#1208) - ([eba73c7](https://github.com/boundaryml/baml/commit/eba73c783c7f4e0013c0f128b0f2a7c20af330f0)) - Samuel Lijin
- simplify string formatting for readability (#1072) - ([3ebf08f](https://github.com/boundaryml/baml/commit/3ebf08fe54bcfcc384188296f32efa6a878416ec)) - Hamir Mahal


## [0.69.0](https://github.com/boundaryml/baml/compare/0.68.0..0.69.0) - 2024-11-26

### Documentation
- Move documentation link higher in README.md for better visibility (#1190) - ([aaa1149](https://github.com/boundaryml/baml/commit/aaa1149acca0b4552b2d84aba6e6ee933a3a6d6b)) - Dex
- Update Vertex docs for different publishers (#1191) - ([66b2274](https://github.com/boundaryml/baml/commit/66b2274f895615f15b5a6edba51444b7d98dcaa5)) - Antonio Sarosi
- Update TypeScript installation docs to use `pnpm exec` instead of deprecated `pnpx` (#1144) - ([56194b8](https://github.com/boundaryml/baml/commit/56194b8084a08447dfb6ca5bf537289cd36022c4)) - Manav Bokinala
- Update LM Studio documentation (#1176) - ([7689ce7](https://github.com/boundaryml/baml/commit/7689ce7c3c42d49a020b81e0bdca16ef8e0209c7)) - Jeff Winkler

### Features
- Support enums & literals as map keys (#1178) - ([39e0271](https://github.com/boundaryml/baml/commit/39e0271f605234535cc53470a6aedff07aaa0c6c)) - Antonio Sarosi
- Parse triple backtick strings, discarding the info header (#1162) - ([353b21e](https://github.com/boundaryml/baml/commit/353b21e0ba3689420dfea6ff50a9454cf87fa199)) - Samuel Lijin
- Add ability to validate types for template strings (#1161) - ([a578cc2](https://github.com/boundaryml/baml/commit/a578cc287abbd9c23697adc4c83bcf0979916fcf)) - hellovai
- Support single line quoteless JSON parsing (#1170) - ([b1b9cab](https://github.com/boundaryml/baml/commit/b1b9cabcd51f87afef0ef54c7ecd0e2349d97f83)) - hellovai
- Generated code includes docstrings from BAML source docstrings (#1177) - ([170ece9](https://github.com/boundaryml/baml/commit/170ece9e8d72e235a7f5d628739899cd564ee995)) - Greg Hale
- Add ability to parse clients statically whenever possible (#1193) - ([03d9475](https://github.com/boundaryml/baml/commit/03d947581ceb56a3c3498f2746f697ce06a55077)) - hellovai
- Support setting all env vars for AWS-bedrock (#1179) - ([fcdbdfb](https://github.com/boundaryml/baml/commit/fcdbdfbb80e5e7d09411b0e55aa0039b0be998bc)) - hellovai
- Add constraints to test blocks (#1185) - ([cafd2ea](https://github.com/boundaryml/baml/commit/cafd2ea35ac0d3129ddddb7c4fc81561a7316657)) - Greg Hale
- Add sum jinja filter (#1174) - ([2353862](https://github.com/boundaryml/baml/commit/2353862730ed3be9b354a9f6a6c20c4c75a6058f)) - Greg Hale
- Add openrouter key (#1186) - ([28d58c0](https://github.com/boundaryml/baml/commit/28d58c060320154bddfef03bdd6de67d27e26c0f)) - aaronvg

### Bug Fixes
- Fix image path in README.md (#1190) - ([aaa1149](https://github.com/boundaryml/baml/commit/aaa1149acca0b4552b2d84aba6e6ee933a3a6d6b)) - Dex
- Fix template string highlights (#1182) - ([60c823a](https://github.com/boundaryml/baml/commit/60c823a000507e6667670f96f1607ba2ea160c57)) - aaronvg
- Fix nextjs and TS server hot-reload (#1183) - ([22e6bbb](https://github.com/boundaryml/baml/commit/22e6bbb7dbe125b40f72d37e6fb8a73e603aade8)) - aaronvg
- Fix lang name (#1188) - ([8c3d536](https://github.com/boundaryml/baml/commit/8c3d5363dd36c32a512430f970da8c76788335e3)) - aaronvg
- Make id optional as gemini doesn't include it in openai generic (#1187) - ([97d1cd4](https://github.com/boundaryml/baml/commit/97d1cd48dc80bdfaeb08bf8a27b65c21a48145bd)) - aaronvg
- Correctly compute required_env_vars even for shorthand clients (#1164) - ([8b51b6e](https://github.com/boundaryml/baml/commit/8b51b6eb186b8c2853139e37e87a69a87e893059)) - hellovai
- Report wrong keyword errors in type defs (#1166) - ([3b1d152](https://github.com/boundaryml/baml/commit/3b1d15230c9ba6dae3cb8d9f0f7f7e9b75f8f00e)) - Antonio Sarosi
- Remove win32-arm64 support for now to fix yarn and deno builds (#1173) - ([c0234d7](https://github.com/boundaryml/baml/commit/c0234d730915506097ff17b54afd7316fdc850cd)) - aaronvg
- Validate fieldnames and types when using pydantic codegen (#1189) - ([93b393d](https://github.com/boundaryml/baml/commit/93b393ded048817fdb7ffef65cb698f9edb14764)) - Greg Hale


## [0.68.0](https://github.com/boundaryml/baml/compare/0.67.0..0.68.0) - 2024-11-11

### Documentation


### Features

- Recursive types! (#1065) - ([8100df9](https://github.com/boundaryml/baml/commit/8100df999e67690458e8bc6adc50575e855bd242)) - Antonio Sarosi
- Support specifying "region" for aws-bedrock (#1150) - ([cbe3c92](https://github.com/boundaryml/baml/commit/cbe3c9261b3fa5cd026b9492a2858c1822e354df)) - Samuel Lijin
- Add `hoisted_class_prefix` option in docs (#1154) - ([cf2298e](https://github.com/boundaryml/baml/commit/cf2298ec53c74c317c495c7b84e1a56a97193b4f)) - Antonio Sarosi
- Make render messages dynamic and use `hoisted_class_prefix` instead of `"schema"` (#1155) - ([873751b](https://github.com/boundaryml/baml/commit/873751ba84f736dfbcbd9cbb0b6debfe7081cc1f)) - Antonio Sarosi
- Support o1 in playground (allow certain models to disable streaming) (#1157) - ([09c6549](https://github.com/boundaryml/baml/commit/09c65497c3218387756775827ba22bcad16f0362)) - hellovai
- Add basic grammar for `a` vs `an` articles in ctx.output_format (#1158) - ([e084130](https://github.com/boundaryml/baml/commit/e0841307d4da809754d995a4524b39b87040f2d0)) - Antonio Sarosi


### Bug Fixes
- Improved syntax highlighting for template_strings (#1151) - ([8c43e37](https://github.com/boundaryml/baml/commit/8c43e37fdaa05d9f3626fde7ad56614610727348)) - Greg Hale
- Improved error detection for client&lt;llm&gt; parsing (#1026) - ([c6b1167](https://github.com/boundaryml/baml/commit/c6b116744f55f831352209c04cd6bce7b028eda9)) - hellovai
- Fix BAML_LOG_JSON logs for py, ruby, and TS (#1153) - ([9e08642](https://github.com/boundaryml/baml/commit/9e08642470435fbefca20b163de010dd805560b8)) - aaronvg


## [0.67.0](https://github.com/boundaryml/baml/compare/0.66.0..0.67.0) - 2024-11-05

### Bug Fixes
- URGENT: fix generated typescript code (#1147) - ([bd9da16](https://github.com/boundaryml/baml/commit/bd9da1683112d849595580866382cba2c6bed8be)) - hellovai


### Features
- Parser improvement: handle code within backticks (`) (#1146) - ([3d8ef34](https://github.com/boundaryml/baml/commit/3d8ef34af15a7f2b957876ffa71314ce38da2a01)) - hellovai

## [0.66.0](https://github.com/boundaryml/baml/compare/0.65.0..0.66.0) - 2024-11-04

### Features

- BAML_LOG supports JSON mode (#1137) - ([f140767](https://github.com/boundaryml/baml/commit/f1407674fc0d91c079fd93b655ff097a05475740)) - aaronvg
- Block-level constraints (#1124) - ([e931acb](https://github.com/boundaryml/baml/commit/e931acb7f765e86a70cb33cd86728aabe058024b)) - Greg Hale
- Parser improvement! Streaming arrays is much more stable and parsing file paths improved (#1134) - ([56570f0](https://github.com/boundaryml/baml/commit/56570f0fe6c4c09594eb757c8a78158cf0e73fcd)) - hellovai


### Documentation

- Improvements to Reference Documentation (#1125) - ([12c8fa7](https://github.com/boundaryml/baml/commit/12c8fa7ec5aea8571f27fb65b8f2a327a1a5e0ce)) - hellovai
- README.md: typo/readability fixes (#1092) - ([cb67e31](https://github.com/boundaryml/baml/commit/cb67e316dce2c4ee29b6fd625316f5df4409335f)) - Prathamesh Pawar
- README.md: Correct Promptfiddle link (#1108) - ([b296c4c](https://github.com/boundaryml/baml/commit/b296c4cf6104513e40ef89f17d534d6d8858f223)) - Sagar Sharma
- Fix broken links (#1133) - ([e0bfc94](https://github.com/boundaryml/baml/commit/e0bfc94f453f35971e871a4b121a1f35fa0b17cc)) - aaronvg


### Bug-fix

- Improve syntax highlighting for template strings (#1130) - ([54de4b6](https://github.com/boundaryml/baml/commit/54de4b6ed9144a68fe0a84d916679f9aec46fe28)) - hellovai
- Improved static analysis for literals in jinja (#1132) - ([b8a221f](https://github.com/boundaryml/baml/commit/b8a221ff44668e2b1d9fa75100c270ce5a227ed4)) - Greg Hale
- Adds missing imports to the sync_client template (#1131) - ([823f74c](https://github.com/boundaryml/baml/commit/823f74c88df3cc7b9ebb4b19b74b5ee6edbafd9c)) - Jesus Lizama
- Add `Checked` in baml client typescript (#1135) - ([ad759cd](https://github.com/boundaryml/baml/commit/ad759cdb67cb0b6a6d2bd0d16575e3e1bc847a68)) - Greg Hale
- Produce syntax error when user misses return type on functions (#1129) - ([034d6eb](https://github.com/boundaryml/baml/commit/034d6ebda38aded1c6a93321d363575156b0ecc6)) - hellovai

## [0.65.0](https://github.com/boundaryml/baml/compare/0.64.0..0.65.0) - 2024-10-31

### Documentation

- **New Documentation Structure**: Introduced version 3 of the documentation, enhancing clarity and organization. ([#1118](https://github.com/boundaryml/baml/commit/bab2767414172dd632437a57631c4cee04910518))

Co-authored by: Vaibhav Gupta <vbv@boundaryml.com>, Antonio Sarosi <sarosiantonio@gmail.com>

### Bug Fixes

- **Python Type Handling**: Moved Python Checked and Check types into `baml_client` for better type management. ([#1122](https://github.com/boundaryml/baml/commit/0ccf473fd821d25d431bbf4341c4e837967104bf))
- **Literal Input Type Checking**: Fixed an issue where literal inputs were not being type-checked correctly. ([#1121](https://github.com/boundaryml/baml/commit/aa5dc85026a175216b5caae6320d09a1fcd35752))


## [0.64.0](https://github.com/boundaryml/baml/compare/0.63.0..0.64.0) - 2024-10-29

### Bug Fixes
- **Playground Stability:** Prevented crashes in the playground due to malformed vertex credentials ([#1107](https://github.com/boundaryml/baml/commit/e665346fbc84a9b969a979cfdf1c70d530201e93)) - _Samuel Lijin_
- **Union Handling:** Addressed an issue with union types in the schema ([#1096](https://github.com/boundaryml/baml/commit/cb5ce7623d3e95464fb5e5152c4d2339458caa26)) - _Greg Hale_
- **WASM Function Signatures:** Resolved stack overflow when computing WASM function signatures ([#1100](https://github.com/boundaryml/baml/commit/aa736ed2d7386cae78421c22d5669c73d8921085)) - _aaronvg_
- **VSCode Extension:** Fixed crashes in the VSCode extension that caused the output panel to open unexpectedly ([#1103](https://github.com/boundaryml/baml/commit/cb5a266bc68f15483f3ec3fa0f4edbc8d176287a)) - _hellovai_
- **Static Analysis Improvements:** Enhanced static analysis on Jinja expressions and `regex_match` functions ([#1102](https://github.com/boundaryml/baml/commit/7ca8136ffbc690877091627415941674f6f14b2f), [#1104](https://github.com/boundaryml/baml/commit/83ddb1cfe81c9b5f6ae620c331c4eefe512c78bd)) - _hellovai_
- **Codegen Enhancements:** Fixed code generation for Python boolean literals and updated integration tests ([#1099](https://github.com/boundaryml/baml/commit/635976238fd9246bfb8764875358a36b4ec6a7f5)) - _Antonio Sarosi_
- **Enum Handling:** Improved substring alias handling for enums ([#1098](https://github.com/boundaryml/baml/commit/0c5cbd4ae03d2bc836ee4b61a7df638855bb72ca)) - _Miguel Cárdenas_
- **Syntax Highlighting:** Refined span calculations for Jinja expressions and improved VSCode syntax highlighting with Lezer ([#1110](https://github.com/boundaryml/baml/commit/a53072f5fe9fe83a0accb36e43a06550602a3c65)) - _hellovai_
- **Ruby Support:** Fixed literal boolean tests for Ruby ([#1109](https://github.com/boundaryml/baml/commit/23e590b0b2fdb51f80e7eced769baabd12b3be22)) - _Antonio Sarosi_

### Features
- **Constraint Support:** Added the ability to define constraints using Jinja expressions ([#1006](https://github.com/boundaryml/baml/commit/d794f28b4f8830b1a40cd08043ecdc562938d36e)) - _Greg Hale_
- **VSCode & Fiddle UI:** Introduced a new "Intro to Checks" UI for easier onboarding ([#1106](https://github.com/boundaryml/baml/commit/11efa5e97f8e9b8f385b7fb0e823f5ff2bc4c314)) - _Samuel Lijin_
- **Dev Container Configurations:** Added Dev Container configurations for streamlined development environments ([#1112](https://github.com/boundaryml/baml/commit/5790393d7ad320e9e257c09e461c9bc39310a834)) - _Antonio Sarosi_

### Documentation
- **Constraints Documentation:** Published new documentation for defining constraints in BAML ([#1113](https://github.com/boundaryml/baml/commit/6332021a59661d3931934adc2afbf4f99f6f4bee)) - _Greg Hale_
- **Dynamic Types Linking:** Added cross-links to dynamic types documentation for easier navigation ([#1116](https://github.com/boundaryml/baml/commit/8ce0a539d74d05438e8047e4e02022ddd7121e21)) - _Greg Hale_

### Miscellaneous
- **Code Quality:** Improved style and fixed typos in the codebase ([#1115](https://github.com/boundaryml/baml/commit/4c3970a6e6ce998a784e682f4c218ba2a69cf86a)) - _Greg Hale_
- **Parsing Stability:** Added logic to prevent assertions from parsing errors and ensured checks no longer affect parsing ([#1101](https://github.com/boundaryml/baml/commit/5ec89c92ab14622afddc3ce348c5b981b4840492)) - _hellovai_
- **Version Bump:** Bumped version to 0.64.0 ([#1114](https://github.com/boundaryml/baml/commit/90d3c17ba67bc1467ee5973ff6cf257069e265b9), [#ff7e152](https://github.com/boundaryml/baml/commit/ff7e152510395bab1d38afa60211226070d12cc2)) - _Vaibhav Gupta_


## [0.63.0](https://github.com/boundaryml/baml/compare/0.62.0..0.63.0) - 2024-10-23

### Bug Fixes
- Fix dynamic enums which already are defined in BAML (#1080) - ([22d0f1c](https://github.com/boundaryml/baml/commit/22d0f1cff3428c2cd58ea78c50c4fc7ea39c8d0c)) - hellovai

### Features
- Updated clients.baml to use the latest sonnet model (#1081) - ([71df0b7](https://github.com/boundaryml/baml/commit/71df0b7b627ba218d581d2c21be01fea4e4993c1)) - aaronvg
- Improved clients.baml generated via baml init (#1089) - ([682dd66](https://github.com/boundaryml/baml/commit/682dd66f4adab8c4fad13bfe32a3fc0268d8b511)) - hellovai

## [0.62.0](https://github.com/boundaryml/baml/compare/0.61.1..0.62.0) - 2024-10-21

### Features

- Support serializing/deserializing `baml_py.Image`, `baml_py.Audio` for pydantic (#1062) - ([11cb699](https://github.com/boundaryml/baml/commit/11cb69903dce1ae348c68f88a82b4731da3977a7)) - Samuel Lijin
- Support rendering input classes with aliases (#1045) - ([3824cda](https://github.com/boundaryml/baml/commit/3824cda75524105f3401e5c7e4c21e604d639f76)) - aaronvg
- Add unstable_internal_repr on FunctionResult in python (#1068) - ([00082e8](https://github.com/boundaryml/baml/commit/00082e8b941d3648ec499215d2c38091f36db944)) - hellovai
- Add literal support for type_builder (#1069) - ([c0085d9](https://github.com/boundaryml/baml/commit/c0085d908cbf8696623fd70f49de5ca8325de06c)) - hellovai

### Bug Fixes

- Surface errors in fallbacks containing only erroneous clients (#1061) - ([b69ef79](https://github.com/boundaryml/baml/commit/b69ef79542ec818b8779f9710dad65d33166c862)) - Greg Hale
- Fix parser so that we are able to correctly detect sequences of empty strings. (#1048) - ([977e277](https://github.com/boundaryml/baml/commit/977e2776119a6f1e79f29cbe596b1c31697becb5)) - hellovai
- Make substring match algorithm case insensitive (#1056) - ([fa2c477](https://github.com/boundaryml/baml/commit/fa2c4770791297a7a37a3f0c837ede4bb709f0ef)) - Antonio Sarosi
- Fix vertex-ai citation data being optional (#1058) - ([5eae0a7](https://github.com/boundaryml/baml/commit/5eae0a73be6cc8286ce045185537aeed0b9feb7d)) - aaronvg
- Fix bug to correctly cast to pydantic types in ambiguous scenarios where BAML knows better (#1059) - ([830b0cb](https://github.com/boundaryml/baml/commit/830b0cb194b99fa6f019928e7466dcf3e3992596)) - hellovai
- Parser: Prefer case sensitive match over case insensitive (#1063) - ([cd6b141](https://github.com/boundaryml/baml/commit/cd6b141020ec8dfd2514c82ffffaebc5678a025b)) - Antonio Sarosi
- Only popup the vscode env var dialog once (#1066) - ([1951474](https://github.com/boundaryml/baml/commit/19514745cfc8efeb8bda0be655e0fa2f216e4b29)) - aaronvg

### Documentation

- Docs for literal types (#1030) - ([55e5964](https://github.com/boundaryml/baml/commit/55e596419055c8da52b841b9ecbf16e328bc1033)) - Antonio Sarosi
- Contribution guide (#1055) - ([f09d943](https://github.com/boundaryml/baml/commit/f09d9432d95c876f5e63f3abdb47a40417c5c45a)) - aaronvg

### Misc

- Fix VSCode metrics (#1044) - ([a131336](https://github.com/boundaryml/baml/commit/a13133656e1610cac9a92aa4b4459c78340c7304)) - hellovai
- Add more test cases for unquoted strings in objects (#1054) - ([2d1b700](https://github.com/boundaryml/baml/commit/2d1b700e82604e444d904cfeb67f46ced97153a5)) - hellovai

## [0.61.1](https://github.com/boundaryml/baml/compare/0.61.0..0.61.1) - 2024-10-15

### Bug Fixes

- add musl to the ts release artifacts (#1042) - ([e74f3e9](https://github.com/boundaryml/baml/commit/e74f3e90489a403e38b39cc694d30d038ad38b8d)) - Samuel Lijin

## [0.61.0](https://github.com/boundaryml/baml/compare/0.60.0..0.61.0) - 2024-10-14

### Features

- Implement literal types (#978) - ([9e7431f](https://github.com/boundaryml/baml/commit/9e7431f43b74d4428e6a20b9aa3a1e93768ff905)) - Antonio Sarosi
- allow installing the TS library on node-alpine (#1029) - ([1c37a0d](https://github.com/boundaryml/baml/commit/1c37a0d71d921d13f05340ff6727255ba6074152)) - Samuel Lijin
- Add WYSIWYG UI (Swagger UI) to baml-cli dev (#1019) - ([0c73cab](https://github.com/boundaryml/baml/commit/0c73cab3d6ac3bbb04cc898ac102900ca9b17f86)) - Greg Hale
- Suppress streaming for Numbers (#1032) - ([3f4621b](https://github.com/boundaryml/baml/commit/3f4621b36555062312aabd9ba8435b965ba8fd92)) - Greg Hale

### Bug Fixes

- Add limit on connection pool to prevent stalling issues in pyo3 and other ffi boundaries (#1027) - ([eb90e62](https://github.com/boundaryml/baml/commit/eb90e62ffe21109e0da1bd74439d36bb37246ec3)) - hellovai
- Update docs (#1025) - ([2dd1bb6](https://github.com/boundaryml/baml/commit/2dd1bb6cf743c20af53d7147db8a4573de9cdbe0)) - Farookh Zaheer Siddiqui
- Fix parsing for streaming of objects more stable (#1031) - ([8aa9c00](https://github.com/boundaryml/baml/commit/8aa9c00b8f26a8c30178ff25aecc1c3b47b6696e)) - hellovai
- Fix python BamlValidationError type (#1036) - ([59a9510](https://github.com/boundaryml/baml/commit/59a9510c9d2c1216df01b0701cc23afb02e3f700)) - aaronvg

### Miscellaneous

- Popup settings dialog when no env vars set (#1033) - ([b9fa52a](https://github.com/boundaryml/baml/commit/b9fa52aea8686f8095878e7f210c2d937b533c63)) - aaronvg
- Bump version to 0.61.0 - ([ca2242b](https://github.com/boundaryml/baml/commit/ca2242b26214699268fda9e9ac07338c6491026d)) - Aaron Villalpando

## [0.60.0](https://github.com/boundaryml/baml/compare/0.59.0..0.60.0) - 2024-10-09

### Miscellaneous Chores

- update Dockerfile (#1017) - ([51539b7](https://github.com/boundaryml/baml/commit/51539b7b5778d6a3e6619698d2033d4f66f15d27)) - Ikko Eltociear Ashimine
- Revert "feat: add a WYSIWYG UI (Swagger UI) to `baml-cli dev` (#1011)" (#1018) - ([f235050](https://github.com/boundaryml/baml/commit/f235050a57916116aff8359236b819ac69011a21)) - Greg Hale

### Bug fixes

- Fix python types for BamlValidationError (#1020) - ([520a09c](https://github.com/boundaryml/baml/commit/520a09c478ea8c5eb811447ce9b36689692aa01d)) - aaronvg
- coerce floats and ints with commas and other special cases (#1023) - ([904492e](https://github.com/boundaryml/baml/commit/904492ee298727085e00a391beb628c8d999083e)) - aaronvg

### Docs

- Add Docs for Jupyter notebook usage (#1008) - ([c51d918](https://github.com/boundaryml/baml/commit/c51d918f76f63ce55b353661459ba3b27b9a0ea7)) - aaronvg

## [0.59.0](https://github.com/boundaryml/baml/compare/0.58.0..0.59.0) - 2024-10-04

### Features

- **(vertex)** allow specifying creds as JSON object (#1009) - ([98868da](https://github.com/boundaryml/baml/commit/98868da4e75dde3a00178cbf60afebc501d37b0c)) - Samuel Lijin
- Add prompt, raw_output and error message to BamlValidationError in TS and Python (#1005) - ([447dbf4](https://github.com/boundaryml/baml/commit/447dbf4e0d0cf0744307ef50f89050752334d982)) - aaronvg
- Add BamlValidationError to `baml-cli serve` (#1007) - ([3b8cf16](https://github.com/boundaryml/baml/commit/3b8cf1636594c1a7245a733556efa690da40e139)) - aaronvg
- Include a WYSIWYG UI (Swagger UI) to `baml-cli dev` (#1011) - ([fe9dde4](https://github.com/BoundaryML/baml/commit/fe9dde4f3a7ff0503fd13087da50e4da9d97c3a0)) - imalsogreg

## [0.58.0](https://github.com/boundaryml/baml/compare/0.57.1..0.58.0) - 2024-10-02

### Features

- Add client registry support for BAML over Rest (OpenAPI) (#1000) - ([abe70bf](https://github.com/boundaryml/baml/commit/abe70bf368c9361a3ab32643735f68e0fafd8425)) - Lorenz Ohly

### Bug Fixes

- Improve performance of parsing escaped characters in strings during streaming. (#1002) - ([b35ae2c](https://github.com/boundaryml/baml/commit/b35ae2c4777572206a79af5c2943f5bdd6ada081)) - hellovai

### Documentation

- Add Docs for Document Extraction API (#996) - ([da1a5e8](https://github.com/boundaryml/baml/commit/da1a5e876368074235f4474673a1ebfe632e11ed)) - aaronvg

## [0.57.1](https://github.com/boundaryml/baml/compare/0.57.0..0.57.1) - 2024-09-29

### Bug Fixes

- [BUGFIX] Parser should require a space between class keyword and class name (#990) - ([7528247](https://github.com/boundaryml/baml/commit/752824723404a4ed4c4b1e31c43d140e9346dca2)) - Greg Hale
- Remove dynamic string attributes (#991) - ([0960ab2](https://github.com/boundaryml/baml/commit/0960ab2e0d16c50fef58772336b91297ddac6919)) - Greg Hale
- ts fixes (#992) - ([36af43f](https://github.com/boundaryml/baml/commit/36af43f4f773e1565527916eff7d7837d9f8a983)) - aaronvg
- Bump version to 0.57.1 - ([0aa71dd](https://github.com/boundaryml/baml/commit/0aa71dd4d3aa7082db6a19f0a3a976ff55789d83)) - Aaron Villalpando

## [0.57.0](https://github.com/boundaryml/baml/compare/0.56.1..0.57.0) - 2024-09-27

### Documentation

- Fix Python dynamic types example (#979) - ([eade116](https://github.com/boundaryml/baml/commit/eade116de14bcc15d738fec911d8653685c13706)) - lorenzoh

### Features

- teach vscode/fiddle to explain when we drop information (#897) - ([93e2b9b](https://github.com/boundaryml/baml/commit/93e2b9b8d54a4ced0853ce72596d0b0a9896a0da)) - Samuel Lijin
- Add ability for users to reset env vars to their desire. (#984) - ([69e6c29](https://github.com/boundaryml/baml/commit/69e6c29c82ccc06f8939b9ece75dd7797c8f6b98)) - hellovai

### Bug Fixes

- Fixed panic during logging for splitting on UTF-8 strings. (#987) - ([c27a64f](https://github.com/boundaryml/baml/commit/c27a64f6320515cd5ab6385ab93013d3d7ba88b8)) - hellovai
- Improve SAP for triple quoted strings along with unions (#977) - ([44202ab](https://github.com/boundaryml/baml/commit/44202ab63aa3d2881485b9b32fa744797c908e33)) - hellovai
- Add more unit tests for parsing logic inspired by user (#980) - ([48dd09f](https://github.com/boundaryml/baml/commit/48dd09f89b6447cbc1a539ecade66ab4da87b8dc)) - hellovai
- Improve syntax errors e.g. class / enum parsing and also update pestmodel to handle traling comments (#981) - ([adbb6ae](https://github.com/boundaryml/baml/commit/adbb6ae38833d700bfe0123ac712cd90d7e4d970)) - hellovai
- Updating docs for env vars (#985) - ([305d6b3](https://github.com/boundaryml/baml/commit/305d6b3e5a57513adc43c8ab9068c523dfc2e69c)) - hellovai
- When using openai-generic, use a string as the content type in the api request if theres no media (#988) - ([e8fa739](https://github.com/boundaryml/baml/commit/e8fa739838cc124a8eed49103871b1b971063821)) - aaronvg

## [0.56.1](https://github.com/boundaryml/baml/compare/0.56.0..0.56.1) - 2024-09-21

### Bug Fixes

- Improved parser for unions (#975) - ([b390521](https://github.com/boundaryml/baml/commit/b39052111529f217762b3271846006bec4a604de)) - hellovai
- [syntax] Allow lists to contain trailing comma (#974) - ([9e3dc6c](https://github.com/boundaryml/baml/commit/9e3dc6c90954905a96b599ef28c40094fe48a43e)) - Greg Hale

## [0.56.0](https://github.com/boundaryml/baml/compare/0.55.3..0.56.0) - 2024-09-20

Shout outs to Nico for fixing some internal Rust dependencies, and to Lorenz for correcting our documentation! We really appreciate it :)

### Features

- use better default for openapi/rust client (#958) - ([b74ef15](https://github.com/boundaryml/baml/commit/b74ef15fd4dc09ecc7d1ac8284e7f22cd6d5864c)) - Samuel Lijin

### Bug Fixes

- push optional-list and optional-map validation to post-parse (#959) - ([c0480d5](https://github.com/boundaryml/baml/commit/c0480d5cfd46ce979e957223dc7b5fa744778552)) - Samuel Lijin
- improve OpenAPI instructions for windows/java (#962) - ([6010efb](https://github.com/boundaryml/baml/commit/6010efbb7990fda966640c3af267de41362d3fa4)) - Samuel Lijin
- assorted fixes: unquoted strings, openai-generic add api_key for bearer auth, support escape characters in quoted strings (#965) - ([847f3a9](https://github.com/boundaryml/baml/commit/847f3a9bb0f00303eae7e410663efc63e54c38b6)) - hellovai
- serde-serialize can cause a package dependency cycle (#967) - ([109ae09](https://github.com/boundaryml/baml/commit/109ae0914852f2ee4a771d27103e4e46ad672647)) - Nico
- make anthropic work in fiddle/vscode (#970) - ([32eccae](https://github.com/boundaryml/baml/commit/32eccae44b27c3fec5fbc3270b6657819d75a426)) - Samuel Lijin
- make dynamic enums work as outputs in Ruby (#972) - ([7530402](https://github.com/boundaryml/baml/commit/7530402f0dc063f10f57cf7aa7f06790574de705)) - Samuel Lijin

### Documentation

- suggest correct python init command in vscode readme (#954) - ([e99c5dd](https://github.com/boundaryml/baml/commit/e99c5dd1903078d08aef451e4addc6110d7ca279)) - Samuel Lijin
- add more vscode debugging instructions (#955) - ([342b657](https://github.com/boundaryml/baml/commit/342b657da69441306fa7711d7d14893cf8036f84)) - Samuel Lijin
- NextJS hook needs to be bound to the correct context (#957) - ([ee80451](https://github.com/boundaryml/baml/commit/ee80451de85063b37e658ba58571c791e8514273)) - aaronvg
- update nextjs hooks and docs (#952) - ([01cf855](https://github.com/boundaryml/baml/commit/01cf855500159066fdcd162dc2e2087768d5ba28)) - aaronvg
- Fix some documentation typos (#966) - ([5193cd7](https://github.com/boundaryml/baml/commit/5193cd70686173c863af5ce40fd6bb3792406951)) - Greg Hale
- Keywords AI router (#953) - ([1c6f975](https://github.com/boundaryml/baml/commit/1c6f975d8cc793841745da0db82ee1e2f1908e56)) - aaronvg
- Fix `post_generate` comment (#968) - ([919c79f](https://github.com/boundaryml/baml/commit/919c79fa8cd85a96e6559055b2bb436d925dcb2a)) - lorenzoh

### Bug Fixes

- show actionable errors for string[]? and map<...>? type validation (#946) - ([48879c0](https://github.com/boundaryml/baml/commit/48879c0744f79b482ef0d2b0624464053558ada4)) - Samuel Lijin

### Documentation

- add reference docs about env vars (#945) - ([dd43bc5](https://github.com/boundaryml/baml/commit/dd43bc59087e809e09ca7d3caf628e179a28fc3e)) - Samuel Lijin

## [0.55.2](https://github.com/boundaryml/baml/compare/0.55.1..0.55.2) - 2024-09-11

### Bug Fixes

- use correct locking strategy inside baml-cli serve (#943) - ([fcb694d](https://github.com/boundaryml/baml/commit/fcb694d033317d8538cc7b2c61aaa94f772778db)) - Samuel Lijin

### Features

- allow using DANGER_ACCEPT_INVALID_CERTS to disable https verification (#901) - ([8873fe7](https://github.com/boundaryml/baml/commit/8873fe7577bc879cf0d550063252c4532dcdfced)) - Samuel Lijin

## [0.55.1](https://github.com/boundaryml/baml/compare/0.55.0..0.55.1) - 2024-09-10

### Bug Fixes

- in generated TS code, put eslint-disable before ts-nocheck - ([16d04c6](https://github.com/BoundaryML/baml/commit/16d04c6e360eefca10b4e0d008b03c34de279491)) - Sam Lijin
- baml-cli in python works again - ([b57ca0f](https://github.com/boundaryml/baml/commit/b57ca0f529c80f59b79b19132a8f1339a6b7bfe2)) - Sam Lijin

### Documentation

- update java install instructions (#933) - ([b497003](https://github.com/boundaryml/baml/commit/b49700356f2f69c4acbdc953a66a95224656ffaf)) - Samuel Lijin

### Miscellaneous Chores

- add version headers to the openapi docs (#931) - ([21545f2](https://github.com/boundaryml/baml/commit/21545f2a4d9b3987134d98ac720705dde2045290)) - Samuel Lijin

## [0.55.0](https://github.com/boundaryml/baml/compare/0.54.2..0.55.0) - 2024-09-09

With this release, we're announcing support for BAML in all languages: we now
allow you to call your functions over an HTTP interface, and will generate an
OpenAPI specification for your BAML functions, so you can now generate a client
in any language of your choice, be it Golang, Java, PHP, Ruby, Rust, or any of
the other languages which OpenAPI supports.

Start here to learn more: https://docs.boundaryml.com/docs/get-started/quickstart/openapi

### Features

- implement BAML-over-HTTP (#908) - ([484fa93](https://github.com/boundaryml/baml/commit/484fa93a5a4b4677f531e6ef03bb88d144925c12)) - Samuel Lijin
- Add anonymous telemetry about playground actions (#925) - ([6f58c9e](https://github.com/boundaryml/baml/commit/6f58c9e3e464a8e774771706c2b0d76adb9e6cda)) - hellovai

## [0.54.2](https://github.com/boundaryml/baml/compare/0.54.1..0.54.2) - 2024-09-05

### Features

- Add a setting to disable restarting TS server in VSCode (#920) - ([628f236](https://github.com/boundaryml/baml/commit/628f2360c415fa8a7b0cd90d7249733ff06acaa9)) - aaronvg
- Add prompt prefix for map types in ctx.output_format and add more type validation for map params (#919) - ([4d304c5](https://github.com/boundaryml/baml/commit/4d304c583b9188c1963a34e2a153baaf003e36ac)) - hellovai

### Bug fixes

- Fix glibC issues for python linux-x86_64 (#922) - ([9161bec](https://github.com/boundaryml/baml/commit/9161becccf626f8d13a15626481720f29e0f992c)) - Samuel Lijin

### Documentation

- Add nextjs hooks (#921) - ([fe14f5a](https://github.com/boundaryml/baml/commit/fe14f5a4ef95c9ccda916ff80ce852d3855554a3)) - aaronvg

## [0.54.1](https://github.com/boundaryml/baml/compare/0.54.0..0.54.1) - 2024-09-03

### BREAKING CHANGE

- Fix escape characters in quoted strings (#905) - ([9ba6eb8](https://github.com/boundaryml/baml/commit/9ba6eb834e0145f4c57e582b63730d3d0ac9b2e9)) - hellovai

Prior `"\n"` was interpreted as `"\\n"` in quoted strings. This has been fixed to interpret `"\n"` as newline characters and true for other escape characters.

### Documentation

- updated dead vs-code-extension link (#914) - ([b12f164](https://github.com/boundaryml/baml/commit/b12f1649cf5bfd0d457c5d6d117fd3a21ba5dc6b)) - Christian Warmuth
- Update docs for setting env vars (#904) - ([ec1ca94](https://github.com/boundaryml/baml/commit/ec1ca94c91af2a51b4190a0bad0e0bc1c052f2a3)) - hellovai
- Add docs for LMStudio (#906) - ([ea4c187](https://github.com/boundaryml/baml/commit/ea4c18782de1f713e8d69d473f9e1818c97024c6)) - hellovai
- Fix docs for anthropic (#910) - ([aba2764](https://github.com/boundaryml/baml/commit/aba2764e5b04820d00b08bf52bda603ee27631f1)) - hellovai
- Update discord links on docs (#911) - ([927357d](https://github.com/boundaryml/baml/commit/927357dd64b36c25513352ed4968ebc62dad6132)) - hellovai

### Features

- BAML_LOG will truncate messages to 1000 characters (modify using env var BOUNDARY_MAX_LOG_CHUNK_SIZE) (#907) - ([d266e5c](https://github.com/boundaryml/baml/commit/d266e5c4157f3b28d2f6454a7ea265dda7296bb2)) - hellovai

### Bug Fixes

- Improve parsing parsing when there are initial closing `]` or `}` (#903) - ([46b0cde](https://github.com/boundaryml/baml/commit/46b0cdeffb15bbab20a43728f52ad2a05623e6f7)) - hellovai
- Update build script for ruby to build all platforms (#915) - ([df2f51e](https://github.com/boundaryml/baml/commit/df2f51e52615451b3643cc124e7262f11965f3ef)) - hellovai
- Add unit-test for openai-generic provider and ensure it compiles (#916) - ([fde7c50](https://github.com/boundaryml/baml/commit/fde7c50c939c505906417596d16c7c4607173339)) - hellovai

## [0.54.0](https://github.com/boundaryml/baml/compare/0.53.1..0.54.0) - 2024-08-27

### BREAKING CHANGE

- Update Default Gemini Base URL to v1beta (#891) - ([a5d8c58](https://github.com/boundaryml/baml/commit/a5d8c588e0fd0b7e186d7c71f1f6171334250629)) - gleed

The default base URL for the Gemini provider has been updated to v1beta. This change is should have no impact on existing users as v1beta is the default version for the Gemini python library, we are mirroring this change in BAML.

### Bug Fixes

- Allow promptfiddle to talk to localhost ollama (#886) - ([5f02b2a](https://github.com/boundaryml/baml/commit/5f02b2ac688ceeb5a34e848a8ff87fd43a6b093a)) - Samuel Lijin
- Update Parser for unions so they handle nested objects better (#900) - ([c5b9a75](https://github.com/boundaryml/baml/commit/c5b9a75ea6da7c45da1999032e2b256bec97d922)) - hellovai

### Documentation

- Add ollama to default prompt fiddle example (#888) - ([49146c0](https://github.com/boundaryml/baml/commit/49146c0e50c88615e4cc97adb595849c23bad8ae)) - Samuel Lijin
- Adding improved docs + unit tests for caching (#895) - ([ff7be44](https://github.com/boundaryml/baml/commit/ff7be4478b706da049085d432b2ec98627b5da1f)) - hellovai

### Features

- Allow local filepaths to be used in tests in BAML files (image and audio) (#871) - ([fa6dc03](https://github.com/boundaryml/baml/commit/fa6dc03fcdd3255dd83e25d0bfb3b0e740991408)) - Samuel Lijin
- Add support for absolute file paths in the file specifier (#881) - ([fcd189e](https://github.com/boundaryml/baml/commit/fcd189ed7eb81712bf3b641eb3dde158fc6a62af)) - hellovai
- Implement shorthand clients (You can now use "openai/gpt-4o" as short for creating a complete client.) (#879) - ([ddd15c9](https://github.com/boundaryml/baml/commit/ddd15c92c3e8d81c24cb7305c9fcbb36b819900f)) - Samuel Lijin
- Add support for arbritrary metadata (e.g. cache_policy for anthropic) (#893) - ([0d63a70](https://github.com/boundaryml/baml/commit/0d63a70332477761a97783e203c98fd0bf67f151)) - hellovai
- Expose Exceptions to user code: BamlError, BamlInvalidArgumentError, BamlClientError, BamlClientHttpError, BamlValidationError (#770) - ([7da14c4](https://github.com/boundaryml/baml/commit/7da14c480506e9791b3f4ce52ac73836a042d38a)) - hellovai

### Internal

- AST Restructuring (#857) - ([75b51cb](https://github.com/boundaryml/baml/commit/75b51cbf80a0c8ba19ae05b021ef3c94dacb4e30)) - Anish Palakurthi

## [0.53.1](https://github.com/boundaryml/baml/compare/0.53.0..0.53.1) - 2024-08-11

### Bug Fixes

- fix github release not passing params to napi script causing issues in x86_64 (#872)

- ([06b962b](https://github.com/boundaryml/baml/commit/06b962b945f958bf0637d13fec22bd2d59c64c5f)) - aaronvg

### Features

- Add Client orchestration graph in playground (#801) - ([24b5895](https://github.com/boundaryml/baml/commit/24b5895a1f45ac04cba0f19e6da727b5ee766186)) - Anish Palakurthi
- increase range of python FFI support (#870) - ([ec9b66c](https://github.com/boundaryml/baml/commit/ec9b66c31faf97a58c81c264c7fa1b32e0e9f0ae)) - Samuel Lijin

### Misc

- Bump version to 0.53.1 - ([e4301e3](https://github.com/boundaryml/baml/commit/e4301e37835483f51edf1cad6478e46ff67508fc)) - Aaron Villalpando

## [0.53.0](https://github.com/boundaryml/baml/compare/0.52.1..0.53.0) - 2024-08-05

### Bug Fixes

- make image[] render correctly in prompts (#855) - ([4a17dce](https://github.com/boundaryml/baml/commit/4a17dce43c05efd5f4ea304f2609fe140de1dd8c)) - Samuel Lijin

### Features

- **(ruby)** implement dynamic types, dynamic clients, images, and audio (#842) - ([4a21eed](https://github.com/boundaryml/baml/commit/4a21eed668f32b042fba61f24c9efb8b3794a420)) - Samuel Lijin
- Codelenses for test cases (#812) - ([7cd8794](https://github.com/boundaryml/baml/commit/7cd87942bf50a72de0ad46154f164fb2c174f25b)) - Anish Palakurthi

### Issue

- removed vertex auth token printing (#846) - ([b839316](https://github.com/boundaryml/baml/commit/b83931665a2c3b840eb6c6d31cf3d01c7926e52e)) - Anish Palakurthi
- Fix google type deserialization issue - ([a55b9a1](https://github.com/boundaryml/baml/commit/a55b9a106176ed1ce34bb63397610c2640b37f16)) - Aaron Villalpando

### Miscellaneous Chores

- clean up release stuff (#836) - ([eed41b7](https://github.com/boundaryml/baml/commit/eed41b7474417d2e65b2c5d742234cc20fc5644e)) - Samuel Lijin
- Add bfcl results to readme, fix links icons (#856) - ([5ef7f3d](https://github.com/boundaryml/baml/commit/5ef7f3db99d8d23ff97f1e8372ee71ab7aa127aa)) - aaronvg
- Fix prompt fiddle and playground styles, add more logging, and add stop-reason to playground (#858) - ([38e3153](https://github.com/boundaryml/baml/commit/38e3153843a17ae1e87ae9879ab4374b083d77d0)) - aaronvg
- Bump version to 0.53.0 - ([fd16839](https://github.com/boundaryml/baml/commit/fd16839a2c0b9d92bd5bdcb57f950e22d0a29959)) - Aaron Villalpando

## [0.52.1](https://github.com/boundaryml/baml/compare/0.52.0..0.52.1) - 2024-07-24

### Bug Fixes

- build python x86_64-linux with an older glibc (#834) - ([db12540](https://github.com/boundaryml/baml/commit/db12540a92abf055e286c60864299f53c246b62a)) - Samuel Lijin

## [0.52.0](https://github.com/boundaryml/baml/compare/0.51.3..0.52.0) - 2024-07-24

### Features

- Add official support for ruby (#823) - ([e81cc79](https://github.com/boundaryml/baml/commit/e81cc79498809a79f427864704b140967a41277a)) - Samuel Lijin

### Bug Fixes

- Fix ClientRegistry for Typescript code-gen (#828) - ([b69921f](https://github.com/boundaryml/baml/commit/b69921f45df0182072b09ab28fe6231ccfaa5767)) - hellovai

## [0.51.2](https://github.com/boundaryml/baml/compare/0.51.1..0.51.2) - 2024-07-24

### Features

- Add support for unions / maps / null in TypeBuilder. (#820) - ([8d9e92d](https://github.com/boundaryml/baml/commit/8d9e92d3050a67edbec5ee6056397becbcdb754b)) - hellovai

### Bug Fixes

- [Playground] Add a feedback button (#818) - ([f749f2b](https://github.com/boundaryml/baml/commit/f749f2b19b247de2f050beccd1fe8e50b7625757)) - Samuel Lijin

### Documentation

- Improvements across docs (#807) - ([bc0c176](https://github.com/boundaryml/baml/commit/bc0c1761699ee2485a0a8ee61cf4fda6b579f974)) - Anish Palakurthi

## [0.51.1](https://github.com/boundaryml/baml/compare/0.51.0..0.51.1) - 2024-07-21

### Features

- Add a feedback button to VSCode Extension (#811) - ([f371912](https://github.com/boundaryml/baml/commit/f3719127174d8f998579747f14fae8675dafba4c)) - Samuel Lijin

### Bug

- Allow default_client_mode in the generator #813 (#815) - ([6df7fca](https://github.com/boundaryml/baml/commit/6df7fcabc1eb55b08a50741f2346440f631abd63)) - hellovai

## [0.51.0](https://github.com/boundaryml/baml/compare/0.50.0..0.51.0) - 2024-07-19

### Bug Fixes

- Improve BAML Parser for numbers and single-key objects (#785) - ([c5af7b0](https://github.com/boundaryml/baml/commit/c5af7b0d0e881c3046171ca17f317d820e8882e3)) - hellovai
- Add docs for VLLM (#792) - ([79e8773](https://github.com/boundaryml/baml/commit/79e8773e38da524795dda606b9fae09a274118e1)) - hellovai
- LLVM install and rebuild script (#794) - ([9ee66ed](https://github.com/boundaryml/baml/commit/9ee66ed2dd14bc0ee12a788f41eae64377e7f2b0)) - Anish Palakurthi
- Prevent version mismatches when generating baml_client (#791) - ([d793603](https://github.com/boundaryml/baml/commit/d7936036e6afa4a0e738242cfb3feaa9e15b3657)) - aaronvg
- fiddle build fix (#800) - ([d304203](https://github.com/boundaryml/baml/commit/d304203241726ac0ba8781db7ac5693339189eb4)) - aaronvg
- Dont drop extra fields in dynamic classes when passing them as inputs to a function (#802) - ([4264c9b](https://github.com/boundaryml/baml/commit/4264c9b143edda0239af197d110357b1969bf12c)) - aaronvg
- Adding support for a sync client for Python + Typescript (#803) - ([62085e7](https://github.com/boundaryml/baml/commit/62085e79d4d86f580ce189bc60f36bd1414893c4)) - hellovai
- Fix WASM-related issues introduced in #803 (#804) - ([0a950e0](https://github.com/boundaryml/baml/commit/0a950e084748837ee2e269504d22dba66f339ca4)) - hellovai
- Adding various fixes (#806) - ([e8c1a61](https://github.com/boundaryml/baml/commit/e8c1a61a96051160566b6458dac5c89d5ddfb86e)) - hellovai

### Features

- implement maps in BAML (#797) - ([97d7e62](https://github.com/boundaryml/baml/commit/97d7e6223c68e9c338fe7110554f1f26b966f7e3)) - Samuel Lijin
- Support Vertex AI (Google Cloud SDK) (#790) - ([d98ee81](https://github.com/boundaryml/baml/commit/d98ee81a9440de0aaa6de05b33b8d3f709003a00)) - Anish Palakurthi
- Add copy buttons to test results in playground (#799) - ([b5eee3d](https://github.com/boundaryml/baml/commit/b5eee3d15a1be4373e25cc8ef1cf6e70d5dd39c9)) - aaronvg

### Miscellaneous Chores

- in fern config, defer to installed version (#789) - ([479f1b2](https://github.com/boundaryml/baml/commit/479f1b2b0b52faf47bc529e4c06c533a9467269a)) - fern
- publish docs on every push to the default branch (#796) - ([180824a](https://github.com/boundaryml/baml/commit/180824a3857a32eae679e4df5704abba3aa6246c)) - Samuel Lijin
- 🌿 introducing fern docs (#779) - ([46f06a9](https://github.com/boundaryml/baml/commit/46f06a95a1e262e62476768b812b372b696da1be)) - fern
- Add test for dynamic list input (#798) - ([7528d6a](https://github.com/boundaryml/baml/commit/7528d6ae10427c1304e356cf5b3c664e4fb2b1b1)) - aaronvg

## [0.50.0](https://github.com/boundaryml/baml/compare/0.49.0..0.50.0) - 2024-07-11

### Bug Fixes

- [Playground] Environment variable button is now visible on all themes (#762) - ([adc4da1](https://github.com/boundaryml/baml/commit/adc4da1fa36cc9c30ea36e25de1a6cefcce0bc97)) - aaronvg
- [Playground] Fix to cURL rendering and mime_type overriding (#763) - ([67f9c6a](https://github.com/boundaryml/baml/commit/67f9c6add5ea8bbbd5ee82c28476fe0ebbefe344)) - Anish Palakurthi

### Features

- [Runtime] Add support for clients that change at runtime using ClientRegistry (#683) - ([c0fb454](https://github.com/boundaryml/baml/commit/c0fb4540d9193194fcafd7fcef71468442d9e6fa)) - hellovai
  https://docs.boundaryml.com/docs/calling-baml/client-registry

### Documentation

- Add more documentation for TypeBuilder (#767) - ([85dc8ab](https://github.com/boundaryml/baml/commit/85dc8ab41e0df3267249a1efc4a95f010e52cc73)) - Samuel Lijin

## [0.49.0](https://github.com/boundaryml/baml/compare/0.46.0..0.49.0) - 2024-07-08

### Bug Fixes

- Fixed Azure / Ollama clients. Removing stream_options from azure and ollama clients (#760) - ([30bf88f](https://github.com/boundaryml/baml/commit/30bf88f65c8583ab02db6a7b7db40c1e9f3b05b6)) - hellovai

### Features

- Add support for arm64-linux (#751) - ([adb8ee3](https://github.com/boundaryml/baml/commit/adb8ee3097fd386370f75b3ba179d18b952e9678)) - Samuel Lijin

## [0.48.0](https://github.com/boundaryml/baml/compare/0.47.0..0.48.0) - 2024-07-04

### Bug Fixes

- Fix env variables dialoge on VSCode (#750)
- Playground selects correct function after loading (#757) - ([09963a0](https://github.com/boundaryml/baml/commit/09963a02e581da9eb8f7bafd3ba812058c97f672)) - aaronvg

### Miscellaneous Chores

- Better error messages on logging failures to Boundary Studio (#754) - ([49c768f](https://github.com/boundaryml/baml/commit/49c768fbe8eb8023cba28b8dc68c2553d8b2318a)) - aaronvg

## [0.47.0](https://github.com/boundaryml/baml/compare/0.46.0..0.47.0) - 2024-07-03

### Bug Fixes

- make settings dialog work in vscode again (#750) ([c94e355](https://github.com/boundaryml/baml/commit/c94e35551872f65404136b60f800fb1688902c11)) - aaronvg
- restore releases on arm64-linux (#751) - ([adb8ee3](https://github.com/boundaryml/baml/commit/adb8ee3097fd386370f75b3ba179d18b952e9678)) - Samuel Lijin

## [0.46.0](https://github.com/boundaryml/baml/compare/0.45.0..0.46.0) - 2024-07-03

### Bug Fixes

- Fixed tracing issues for Boundary Studio (#740) - ([77a4db7](https://github.com/boundaryml/baml/commit/77a4db7ef4b939636472ad4975d74e9d1a577cbf)) - Samuel Lijin
- Fixed flush() to be more reliable (#744) - ([9dd5fda](https://github.com/boundaryml/baml/commit/9dd5fdad5c2897b49a5a536df2e9ef775857a39d)) - Samuel Lijin
- Remove error when user passes in extra fields in a class (#746) - ([2755b43](https://github.com/boundaryml/baml/commit/2755b43257f9405ae66a30982d9711fc3f2c0854)) - aaronvg

### Features

- Add support for base_url for the google-ai provider (#747) - ([005b1d9](https://github.com/boundaryml/baml/commit/005b1d93b7f7d2aa12a1487911766cccd9c25e98)) - hellovai
- Playground UX improvements (#742) - ([5cb56fd](https://github.com/boundaryml/baml/commit/5cb56fdc39496f0aedacd79766c0e93cb0e401b8)) - hellovai
- Prompt Fiddle now auto-switches functions when to change files (#745)

### Documentation

- Added a large example project on promptfiddle.com (#741) - ([f80da1e](https://github.com/boundaryml/baml/commit/f80da1e1dd11f0457b5789bc9ce6923a8ed88b51)) - aaronvg
- Mark ruby as in beta (#743) - ([901109d](https://github.com/boundaryml/baml/commit/901109dbb327e6e3e1b65fda37100fcd45f97e07)) - Samuel Lijin

## [0.45.0](https://github.com/boundaryml/baml/compare/0.44.0..0.45.0) - 2024-06-29

### Bug Fixes

- Fixed streaming in Python Client which didn't show result until later (#726) - ([e4f2daa](https://github.com/boundaryml/baml/commit/e4f2daa9e85bb1711d112fb0c87c0d769be0bb2d)) - Anish Palakurthi
- Improve playground stability on first load (#732) - ([2ac7b32](https://github.com/boundaryml/baml/commit/2ac7b328e89400cba0d9eb4f6d09c6a03feb71a5)) - Anish Palakurthi
- Add improved static analysis for jinja (#734) - ([423faa1](https://github.com/boundaryml/baml/commit/423faa1af5a594b7f78f7bb5620e3146a8989da5)) - hellovai

### Documentation

- Docs for Dynamic Types (#722) [https://docs.boundaryml.com/docs/calling-baml/dynamic-types](https://docs.boundaryml.com/docs/calling-baml/dynamic-types)

### Features

- Show raw cURL request in Playground (#723) - ([57928e1](https://github.com/boundaryml/baml/commit/57928e178549cb3e5118ce374aab5d0fbad7038b)) - Anish Palakurthi
- Support bedrock as a provider (#725) - ([c64c665](https://github.com/boundaryml/baml/commit/c64c66522a1d496493a30f593103209acd201364)) - Samuel Lijin

## [0.44.0](https://github.com/boundaryml/baml/compare/0.43.0..0.44.0) - 2024-06-26

### Bug Fixes

- Fix typebuilder for random enums (#721)

## [0.43.0](https://github.com/boundaryml/baml/compare/0.42.0..0.43.0) - 2024-06-26

### Bug Fixes

- fix pnpm lockfile issue (#720)

## [0.42.0](https://github.com/boundaryml/baml/compare/0.41.0..0.42.0) - 2024-06-26

### Bug Fixes

- correctly propagate LICENSE to baml-py (#695) - ([3fda880](https://github.com/boundaryml/baml/commit/3fda880bf39b32191b425ae75e8b491d10884cf6)) - Samuel Lijin

### Miscellaneous Chores

- update jsonish readme (#685) - ([b19f04a](https://github.com/boundaryml/baml/commit/b19f04a059ba18d54544cb278b6990b95170d3f3)) - Samuel Lijin

### Vscode

- add link to tracing, show token counts (#703) - ([64aa18a](https://github.com/boundaryml/baml/commit/64aa18a9cc34071655141c8f6e2ad04ac90e7be1)) - Samuel Lijin

## [0.41.0] - 2024-06-20

### Bug Fixes

- rollback git lfs, images broken in docs rn (#534) - ([6945506](https://github.com/boundaryml/baml/commit/694550664fa45b5f76987e2663c9d7e7a9a6a2d2)) - Samuel Lijin
- search for markdown blocks correctly (#641) - ([6b8abf1](https://github.com/boundaryml/baml/commit/6b8abf1ccf55bbe7c3bc1046c78081126e01f134)) - Samuel Lijin
- restore one-workspace-per-folder (#656) - ([a464bde](https://github.com/boundaryml/baml/commit/a464bde566199ace45285a78a7f542cd7217fb65)) - Samuel Lijin
- ruby generator should be ruby/sorbet (#661) - ([0019f39](https://github.com/boundaryml/baml/commit/0019f3951b8fe2b49e62eb11d869516b8088e9cb)) - Samuel Lijin
- ruby compile error snuck in (#663) - ([0cb2583](https://github.com/boundaryml/baml/commit/0cb25831788eb8b3eb0a38383917f6d1ffb5633a)) - Samuel Lijin

### Documentation

- add typescript examples (#477) - ([532481c](https://github.com/boundaryml/baml/commit/532481c3df4063b37a8834a5fe2bbce3bb37d2f5)) - Samuel Lijin
- add titles to code blocks for all CodeGroup elems (#483) - ([76c6b68](https://github.com/boundaryml/baml/commit/76c6b68b27ee37972fa226be0b4dfe31f7b4b5ec)) - Samuel Lijin
- add docs for round-robin clients (#500) - ([221f902](https://github.com/boundaryml/baml/commit/221f9020d850e6d24fe2fd8a684081726a0659af)) - Samuel Lijin
- add ruby example (#689) - ([16e187f](https://github.com/boundaryml/baml/commit/16e187f6698a1cc86a37eedf2447648d810370ad)) - Samuel Lijin

### Features

- implement `baml version --check --output json` (#444) - ([5f076ac](https://github.com/boundaryml/baml/commit/5f076ace1f92dc2141b231c9e62f4dc23f7fef18)) - Samuel Lijin
- show update prompts in vscode (#451) - ([b66da3e](https://github.com/boundaryml/baml/commit/b66da3ee355fcd6a8677d834ecb05af44cbf8f20)) - Samuel Lijin
- add tests to check that baml version --check works (#454) - ([be1499d](https://github.com/boundaryml/baml/commit/be1499dfa82ff8ab923a16d45290758120d95015)) - Samuel Lijin
- parse typescript versions in version --check (#473) - ([b4b2250](https://github.com/boundaryml/baml/commit/b4b2250c37b900db899256159bbfc3aa2ec819cb)) - Samuel Lijin
- implement round robin client strategies (#494) - ([599fcdd](https://github.com/boundaryml/baml/commit/599fcdd2a45c5b1e935f36769784ca944566b88c)) - Samuel Lijin
- add integ-tests support to build (#542) - ([f59cf2e](https://github.com/boundaryml/baml/commit/f59cf2e1a9ec7edbe174f4bc7ff9391f2cff3208)) - Samuel Lijin
- make ruby work again (#650) - ([6472bec](https://github.com/boundaryml/baml/commit/6472bec231b581076ee7edefaab2e7979b2bf336)) - Samuel Lijin
- Add RB2B tracking script (#682) - ([54547a3](https://github.com/boundaryml/baml/commit/54547a34d40cd40a43767919dbc9faa68a82faea)) - hellovai

### Miscellaneous Chores

- add nodemon config to typescript/ (#435) - ([231b396](https://github.com/boundaryml/baml/commit/231b3967bc947c4651156bc55fd66552782824c9)) - Samuel Lijin
- finish gloo to BoundaryML renames (#452) - ([88a7fda](https://github.com/boundaryml/baml/commit/88a7fdacc826e78ef21c6b24745ee469d9d02e6a)) - Samuel Lijin
- set up lfs (#511) - ([3a43143](https://github.com/boundaryml/baml/commit/3a431431e8e38dfc68763f15ccdcd1d131f23984)) - Samuel Lijin
- add internal build tooling for sam (#512) - ([9ebacca](https://github.com/boundaryml/baml/commit/9ebaccaa542760cb96382ae2a91d780f1ade613b)) - Samuel Lijin
- delete clients dir, this is now dead code (#652) - ([ec2627f](https://github.com/boundaryml/baml/commit/ec2627f59c7fe9edfff46fcdb65f9b9f0e2e072c)) - Samuel Lijin
- consolidate vscode workspace, bump a bunch of deps (#654) - ([82bf6ab](https://github.com/boundaryml/baml/commit/82bf6ab1ad839f84782a7ef0441f21124c368757)) - Samuel Lijin
- Add RB2B tracking script to propmt fiddle (#681) - ([4cf806b](https://github.com/boundaryml/baml/commit/4cf806bba26563fd8b6ddbd68296ab8bdfac21c4)) - hellovai
- Adding better release script (#688) - ([5bec282](https://github.com/boundaryml/baml/commit/5bec282d39d2250b39ef4aba5d6bba9830a35988)) - hellovai

### [AUTO

- patch] Version bump for nightly release [NIGHTLY:cli] [NIGHTLY:vscode_ext] [NIGHTLY:client-python] - ([d05a22c](https://github.com/boundaryml/baml/commit/d05a22ca4135887738adbce638193d71abca42ec)) - GitHub Action

### Build

- fix baml-core-ffi script (#521) - ([b1b7f4a](https://github.com/boundaryml/baml/commit/b1b7f4af0991ef6453f888f27930f3faaae337f5)) - Samuel Lijin
- fix engine/ (#522) - ([154f646](https://github.com/boundaryml/baml/commit/154f6468ec0aa6de1b033ee1cbc76e60acc363ea)) - Samuel Lijin

### Integ-tests

- add ruby test - ([c0bc101](https://github.com/boundaryml/baml/commit/c0bc10126ea32d099f1398f2c5faa08b111554ba)) - Sam Lijin

### Readme

- add function calling, collapse the table (#505) - ([2f9024c](https://github.com/boundaryml/baml/commit/2f9024c28ba438267de37ac43c6570a2f0398b5a)) - Samuel Lijin

### Release

- bump versions for everything (#662) - ([c0254ae](https://github.com/boundaryml/baml/commit/c0254ae680365854c51c7a4e58ea68d1901ea033)) - Samuel Lijin

### Vscode

- check for updates on the hour (#434) - ([c70a3b3](https://github.com/boundaryml/baml/commit/c70a3b373cb2346a0df9a1eba0ebacb74d59b53e)) - Samuel Lijin

<!-- generated by git-cliff -->
