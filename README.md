# Rust Dependency Injection Demo

by [Tim Abell](https://timwise.co.uk/)

This repo is to try out the various ways to swap out dependencies for fakes/mocks/stubs/doubles etc. for testing.

GPT came up with some ideas here: <https://gist.github.com/timabell/8813d851399908987396c1725aa8b6d6>

Running main will show how many stars there are on a repo by way of a http api call to github. But for tests that will be replaced with a test harness so the tests can run fast, offline, and will not "flap" based on availability of github.

You can see the equivalent by running:

```bash
curl --silent https://api.github.com/repos/timabell/gitopolis | jq '.stargazers_count'
```

Which pulls the json data from <https://api.github.com/repos/timabell/gitopolis> and uses [jq](https://stedolan.github.io/jq/) to extract a single value from the returned json.

## Further reading

- <https://medium.com/geekculture/dependency-injection-in-rust-3822bf689888>
- <https://codereview.stackexchange.com/questions/252017/simple-constructor-di-implementation-in-rust>
- <https://asomers.github.io/mock_shootout/>

### DI crates

<https://crates.io/search?q=dependency+injection>

Scripts: [crates analyser script](crates-analyser.sh) / [crate analyser script](crate-analyser.sh)

Stats as of 21 June 2023

| crate	| last_version	| version	| downloads	| repo	| description	|
| --	| --	| --	| --	| --	| --	|
| [aerosol](https://crates.io/crates/aerosol)	| 40047	| 0.3.0	| 40047	| [Diggsey/aerosol](https://github.com/Diggsey/aerosol)	| Dependency injection with compile-time guarantees	|
| [anthill-di](https://crates.io/crates/anthill-di)	| 2910	| 1.2.4	| 2910	| [Vidrochka/anthill-di](https://github.com/Vidrochka/anthill-di)	| Rust di containers system	|
| [async-di](https://crates.io/crates/async-di)	| 427	| 0.2.0	| 427	| [BSpaceinc/async-di](https://github.com/BSpaceinc/async-di)	| Async dependency injection container	|
| [async-injector](https://crates.io/crates/async-injector)	| 8218	| 0.19.1	| 8218	| [udoprog/async-injector](https://github.com/udoprog/async-injector)	| Reactive dependency injection for Rust.	|
| [autowired](https://crates.io/crates/autowired)	| 7235	| 0.1.8	| 7235	| [nintha/autowired-rs](https://github.com/nintha/autowired-rs)	| Rust dependency injection	|
| [blackbox_core](https://crates.io/crates/blackbox_core)	| 228	| 0.1.1	| 228	| [MichailShcherbakov/blackbox_di](https://github.com/MichailShcherbakov/blackbox_di)	| Rust dependency injection library	|
| [blackbox_core_codegen](https://crates.io/crates/blackbox_core_codegen)	| 276	| 0.1.1	| 276	| [MichailShcherbakov/blackbox_di](https://github.com/MichailShcherbakov/blackbox_di)	| Rust dependency injection library	|
| [blackbox_di](https://crates.io/crates/blackbox_di)	| 144	| 0.1.1	| 144	| [MichailShcherbakov/blackbox_di](https://github.com/MichailShcherbakov/blackbox_di)	| Rust dependency injection library	|
| [blue_typemap](https://crates.io/crates/blue_typemap)	| 77	| 1.0.0	| 77	| [AryanpurTech/BlueTypeMap](https://github.com/AryanpurTech/BlueTypeMap)	| A TypeMap Dependency Injection method for dynamic function parameters	|
| [busybody](https://crates.io/crates/busybody)	| 186	| 0.2.5	| 186	| [shiftrightonce/busybody](https://github.com/shiftrightonce/busybody)	| Service Container and Dependency injector crate	|
| [chassis](https://crates.io/crates/chassis)	| 1093	| 0.1.1	| 1093	| [R1tschY/chassis](https://github.com/R1tschY/chassis)	| Compile-time dependency injection framework	|
| [coi](https://crates.io/crates/coi)	| 5919	| 0.10.1	| 5919	| [Nashenas88/coi](https://github.com/Nashenas88/coi)	| coi is a Dependency Injection library.	|
| [ddi](https://crates.io/crates/ddi)	| 204	| 0.2.1	| 204	| [EYHN/ddi](https://github.com/EYHN/ddi)	|  Dynamic dependency injection library for rust.	|
| [depcon](https://crates.io/crates/depcon)	| 456	| 0.3.0	| 456	| [andybarron/depcon](https://github.com/andybarron/depcon)	| Simple & flexible dependency injection framework	|
| [derive_di](https://crates.io/crates/derive_di)	| 622	| 0.3.0	| 622	| [Mnwa/derive_di](https://github.com/Mnwa/derive_di)	| This crate is realized the dependency injection pattern	|
| [dfdi](https://crates.io/crates/dfdi)	| 140	| 0.2.0	| 140	| [gtsiam/dfdi](https://github.com/gtsiam/dfdi)	| Dependency For Dependency Injection	|
| [dhl](https://crates.io/crates/dhl)	| 1365	| 0.1.2	| 1365	| [Popog/dhl](https://github.com/Popog/dhl)	| Dependency Hijacking Library	|
| [di-derive](https://crates.io/crates/di-derive)	| 116	| null	| 116	| [sunli829/di](https://github.com/sunli829/di)	| A dependency injection library for rust	|
| [di](https://crates.io/crates/di)	| 3688	| 0.1.2	| 3688	| [Nercury/di-rs](https://github.com/Nercury/di-rs)	| Dependency injection container.	|
| [dilib](https://crates.io/crates/dilib)	| 5780	| 0.2.1	| 5780	| [Neo-Ciber94/dilib-rs/](https://github.com/Neo-Ciber94/dilib-rs/)	| A dependency injection library for Rust	|
| [dill](https://crates.io/crates/dill)	| 3937	| 0.6.1	| 3937	| [sergiimk/dill-rs](https://github.com/sergiimk/dill-rs)	| Runtime depenency injection library.	|
| [dose](https://crates.io/crates/dose)	| 3317	| 0.1.3	| 3317	| [noostech/dose](https://github.com/noostech/dose)	| Your daily dose of structs and functions.	|
| [dptree](https://crates.io/crates/dptree)	| 67694	| 0.3.0	| 67694	| [p0lunin/dptree](https://github.com/p0lunin/dptree)	| An asynchronous event dispatch mechanism for Rust	|
| [dyn_inject](https://crates.io/crates/dyn_inject)	| 47	| 0.1.0	| 47	| [NotAPenguin0/dyn_inject](https://github.com/NotAPenguin0/dyn_inject)	| Rust dependency injection that works with trait objects.	|
| [easy-di](https://crates.io/crates/easy-di)	| 247	| null	| 247	| [eisberg-labs/easy-di](https://github.com/eisberg-labs/easy-di)	| Simple dependency injection container for Rust.	|
| [entrait](https://crates.io/crates/entrait)	| 2583	| 0.5.3	| 2583	| [audunhalland/entrait/](https://github.com/audunhalland/entrait/)	| Loosely coupled Rust application design made easy	|
| [env_wrapper](https://crates.io/crates/env_wrapper)	| 865	| 0.1.0	| 865	| [Aembit/env_wrapper/](https://github.com/Aembit/env_wrapper/)	| A wrapper around std::env to facilitate testing	|
| [he_di](https://crates.io/crates/he_di)	| 1876	| 0.2.1	| 1876	| [bgbahoue/he-di](https://github.com/bgbahoue/he-di)	| Dependency Inversion / Dependency Injection / Inversion of control container for Rust	|
| [hypospray](https://crates.io/crates/hypospray)	| 2817	| 0.1.2	| 2817	| [jonysy/hypospray](https://github.com/jonysy/hypospray)	| Lightweight dependency injection library	|
| [indep](https://crates.io/crates/indep)	| 1468	| 0.1.1	| 1468	| [snuk182/indep](https://github.com/snuk182/indep)	| Depencency injection library for Rust	|
| [inject](https://crates.io/crates/inject)	| 1140	| 0.1.3	| 1140	| [tobni/inject-rs](https://github.com/tobni/inject-rs)	| Experimental IOC library for Rust	|
| [io-providers](https://crates.io/crates/io-providers)	| 3345	| 0.1.2	| 3345	| [pshendry/io-providers](https://github.com/pshendry/io-providers)	| Enables dependency injection for many I/O operations	|
| [jabba](https://crates.io/crates/jabba)	| 55	| 0.1.0	| 55	| [zortax/jabba](https://github.com/zortax/jabba)	| Async dependency injection framework	|
| [kamikaze_di](https://crates.io/crates/kamikaze_di)	| 778	| 0.1.0	| 778	| [fabianbadoi/kamikaze_di](https://github.com/fabianbadoi/kamikaze_di)	| Exploration of Dependency Injection in Rust	|
| [kizuna](https://crates.io/crates/kizuna)	| 36	| 0.1.0	| 36	| [Neo-Ciber94/kizuna](https://github.com/Neo-Ciber94/kizuna)	| A simple service locator	|
| [lifeline](https://crates.io/crates/lifeline)	| 11547	| 0.6.1	| 11547	| [austinjones/lifeline-rs](https://github.com/austinjones/lifeline-rs)	| Lifeline is a dependency injection library for asynchronous message-based applications.	|
| [lockjaw](https://crates.io/crates/lockjaw)	| 1001	| 0.2.2	| 1001	| [azureblaze/lockjaw](https://github.com/azureblaze/lockjaw)	| Compile time dependency injection framework inspired by dagger	|
| [matr](https://crates.io/crates/matr)	| 91	| 0.2.0	| 91	| [google/matr](https://github.com/google/matr)	| A metaprogramming library for Rust.	|
| [minfac](https://crates.io/crates/minfac)	| 382	| 0.0.1	| 382	| [mineichen/minfac](https://github.com/mineichen/minfac)	| Lightweight Inversion Of Control	|
| [mold](https://crates.io/crates/mold)	| 598	| 0.0.1	| 598	| [YellowInnovation/mold](https://github.com/YellowInnovation/mold)	| Elegant lazy dependency injection library for Rust	|
| [more-di](https://crates.io/crates/more-di)	| 734	| 2.1.1	| 734	| [commonsensesoftware/more-rs-di](https://github.com/commonsensesoftware/more-rs-di)	| Provides support for dependency injection (DI)	|
| [more-options](https://crates.io/crates/more-options)	| 105	| 2.0.0	| 105	| [commonsensesoftware/more-rs-options](https://github.com/commonsensesoftware/more-rs-options)	| Provides support for options	|
| [mydi](https://crates.io/crates/mydi)	| 113	| 0.1.2	| 113	| [a14e/mydi](https://github.com/a14e/mydi)	| MyDI. Dependency Injection library	|
| [mydi_macros](https://crates.io/crates/mydi_macros)	| 145	| 0.1.2	| 145	| [a14e/mydi](https://github.com/a14e/mydi)	| Macroses for MyDI library	|
| [nject-macro](https://crates.io/crates/nject-macro)	| 387	| 0.2.3	| 387	| [nicolascotton/nject](https://github.com/nicolascotton/nject)	| Zero cost dependency injection macros	|
| [nject](https://crates.io/crates/nject)	| 249	| 0.2.3	| 249	| [nicolascotton/nject](https://github.com/nicolascotton/nject)	| Zero cost dependency injection module	|
| [oxur-component](https://crates.io/crates/oxur-component)	| 582	| 0.1.2	| 582	| [oxur/component](https://github.com/oxur/component)	| Enabling the creation of application components with lifecycle and dependency support	|
| [portaldi](https://crates.io/crates/portaldi)	| 219	| 0.1.4	| 219	| [mtn81/portaldi](https://github.com/mtn81/portaldi)	| An ergonomic lightweight compile-time depencency injection library.	|
| [qregister](https://crates.io/crates/qregister)	| 4110	| 0.2.2	| 4110	| [qrlpx/qregister](https://github.com/qrlpx/qregister)	| A simple IoC (Inversion of Control) container.	|
| [rdi](https://crates.io/crates/rdi)	| 193	| 0.1.0	| 193	| [dudykr/rust-commons.git](https://github.com/dudykr/rust-commons.git)	| Simple dependency injection library for rust.	|
| [rdi_macros](https://crates.io/crates/rdi_macros)	| 478	| 0.1.0	| 478	| [dudykr/rust-commons.git](https://github.com/dudykr/rust-commons.git)	| Macros for rdi, simple dependency injection library for rust.	|
| [rocket-dependency-injection-derive](https://crates.io/crates/rocket-dependency-injection-derive)	| 171	| 0.1.0	| 171	| [Asafrose/rocket-dependency-injection](https://github.com/Asafrose/rocket-dependency-injection)	| Derive macros for rocket-dependency-injection crate	|
| [rocket-dependency-injection](https://crates.io/crates/rocket-dependency-injection)	| 65	| 0.1.0	| 65	| [Asafrose/rocket-dependency-injection](https://github.com/Asafrose/rocket-dependency-injection)	| Small experimental crate for adding dependency injection functionalities to rocket	|
| [rscontainer](https://crates.io/crates/rscontainer)	| 183	| 0.1.0	| 183	| [yvesdum/rscontainer](https://github.com/yvesdum/rscontainer)	| Manages dependencies between objects.	|
| [ruice](https://crates.io/crates/ruice)	| 410	| 0.1.2	| 410	| [yumemi-inc/ruice.git](https://github.com/yumemi-inc/ruice.git)	| Runtime based dependency injection for Rust.	|
| [runtime_injector](https://crates.io/crates/runtime_injector)	| 1361	| 0.4.0	| 1361	| [TehPers/runtime_injector](https://github.com/TehPers/runtime_injector)	| Runtime dependency injection container	|
| [runtime_injector_actix](https://crates.io/crates/runtime_injector_actix)	| 405	| 0.2.0	| 405	| [TehPers/runtime_injector](https://github.com/TehPers/runtime_injector)	| Runtime dependency injection container for actix-web	|
| [sai](https://crates.io/crates/sai)	| 964	| 0.1.4	| 964	| [zhming0/sai](https://github.com/zhming0/sai)	| IoC/DI framework for Rust	|
| [sai_component_derive](https://crates.io/crates/sai_component_derive)	| 1302	| 0.1.4	| 1302	| [zhming0/sai](https://github.com/zhming0/sai)	| Implementation of #[derive(Component)] for Sai	|
| [sdi](https://crates.io/crates/sdi)	| 44	| 0.1.1	| 44	| [](https://github.com/)	| Rust statically resolved dependency injection lib	|
| [shaku](https://crates.io/crates/shaku)	| 28316	| 0.6.1	| 28316	| [Mcat12/shaku](https://github.com/Mcat12/shaku)	| Compile Time Dependency Injection for Rust	|
| [shaku_actix](https://crates.io/crates/shaku_actix)	| 1586	| 0.2.0	| 1586	| [AzureMarker/shaku](https://github.com/AzureMarker/shaku)	| Integration between shaku and Actix Web	|
| [shaku_axum](https://crates.io/crates/shaku_axum)	| 567	| 0.4.0	| 567	| [AzureMarker/shaku](https://github.com/AzureMarker/shaku)	| Integration between shaku and the axum web framework	|
| [shaku_derive](https://crates.io/crates/shaku_derive)	| 28705	| 0.6.1	| 28705	| [Mcat12/shaku](https://github.com/Mcat12/shaku)	| Code generation for the shaku dependency injection framework	|
| [shaku_rocket](https://crates.io/crates/shaku_rocket)	| 2796	| 0.6.0	| 2796	| [AzureMarker/shaku](https://github.com/AzureMarker/shaku)	| Integration between shaku and Rocket	|
| [simpledi-rs](https://crates.io/crates/simpledi-rs)	| 1317	| 0.1.0	| 1317	| [innobead/simpledi-rs](https://github.com/innobead/simpledi-rs)	| simpledi-rs, a simple and thread-safe dependency injection library	|
| [springtime-di-derive](https://crates.io/crates/springtime-di-derive)	| 278	| 0.3.0	| 278	| [krojew/springtime](https://github.com/krojew/springtime)	| Derive support for springtime-di crate.	|
| [springtime-di](https://crates.io/crates/springtime-di)	| 347	| 1.0.0	| 347	| [krojew/springtime](https://github.com/krojew/springtime)	| Dependency injection framework based on automatic component discovery and runtime autowiring.	|
| [springtime-migrate-refinery](https://crates.io/crates/springtime-migrate-refinery)	| 52	| 0.2.0	| 52	| [krojew/springtime](https://github.com/krojew/springtime)	| SQL migration framework based on dependency injection.	|
| [springtime-web-axum](https://crates.io/crates/springtime-web-axum)	| 97	| 1.0.0	| 97	| [krojew/springtime](https://github.com/krojew/springtime)	| Web framework based on Springtime and axum.	|
| [springtime](https://crates.io/crates/springtime)	| 196	| 1.0.0	| 196	| [krojew/springtime](https://github.com/krojew/springtime)	| Dependency injection based application bootstrapping and execution crate.	|
| [subcutaneous](https://crates.io/crates/subcutaneous)	| 19	| 0.1.0	| 19	| [austreelis/subcutaneous](https://github.com/austreelis/subcutaneous)	| Dependency-injection framework inspired by the Bevy Engine	|
| [syrette](https://crates.io/crates/syrette)	| 634	| 0.4.2	| 634	| [at.com/syrette](https://github.com/at.com/syrette)	| The convenient dependency injection framework	|
| [syrette_macros](https://crates.io/crates/syrette_macros)	| 829	| 0.4.2	| 829	| [at.com/syrette](https://github.com/at.com/syrette)	| Macros for Syrette, the convenient dependency injection framework	|
| [syringe-di-derive](https://crates.io/crates/syringe-di-derive)	| 66	| 0.0.2	| 66	| [aburkhalter512/syringe](https://github.com/aburkhalter512/syringe)	| Macros for syringe, a compile-time dependency injection framework	|
| [syringe-di](https://crates.io/crates/syringe-di)	| 69	| 0.0.2	| 69	| [aburkhalter512/syringe](https://github.com/aburkhalter512/syringe)	| A compile-time dependency injection framework for Rust	|
| [teloc](https://crates.io/crates/teloc)	| 2826	| 0.2.0	| 2826	| [p0lunin/teloc](https://github.com/p0lunin/teloc)	| Simple, compile-time DI framework for Rust	|
| [tfsi](https://crates.io/crates/tfsi)	| 176	| 0.1.0	| 176	| [RGafiyatullin/tfsi-rs.git](https://github.com/RGafiyatullin/tfsi-rs.git)	| a simple compile-time dependency injection kit	|
| [thruster-jab](https://crates.io/crates/thruster-jab)	| 2814	| 0.1.0	| 2814	| [](https://github.com/)	| Quick and simple dependency injection.	|
| [waiter_di](https://crates.io/crates/waiter_di)	| 6045	| 1.6.5	| 6045	| [dmitryb-dev/waiter](https://github.com/dmitryb-dev/waiter)	| Dependency injection	|
