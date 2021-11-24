# Changelog

## [0.3.0](https://www.github.com/Huskehhh/CTFd-rs/compare/v0.2.0...v0.3.0) (2021-11-24)


### Features

* **bot:** Include last updated timestamp in channel description ([29cb381](https://www.github.com/Huskehhh/CTFd-rs/commit/29cb381f4ab3d0c23463ac90191d0a1f9ccefa25))
* **bot:** Remove user from working if they solve a challenge ([5c26119](https://www.github.com/Huskehhh/CTFd-rs/commit/5c261190b9405990cfa21d7753cddf7f6d98c147))
* **bot:** Update channel description with HTB rank/stats. ([cbd644f](https://www.github.com/Huskehhh/CTFd-rs/commit/cbd644f8d82f5bb6f108ea25a80692cf86b12c2f))
* **ci:** add docker image layer cache to github registry ([655e107](https://www.github.com/Huskehhh/CTFd-rs/commit/655e10790be98dccbee1c5cd894ff4562a90d98e))
* **ci:** add release-please ([18c9fde](https://www.github.com/Huskehhh/CTFd-rs/commit/18c9fde0cd9880c0831d48b5116b5a0c90280b81))
* **ci:** sccache storage on s3 ([e655f2e](https://www.github.com/Huskehhh/CTFd-rs/commit/e655f2ee6c78b72c5ef7268137c686685a74112c))
* **db:** Run diesel migrations on init ([088dffe](https://www.github.com/Huskehhh/CTFd-rs/commit/088dffe9664d9e9f40717719cba1831911d408c0))


### Bug Fixes

* **bot:** add safety net if there is no db mapping for htb username ([69e5aa6](https://www.github.com/Huskehhh/CTFd-rs/commit/69e5aa6b4d7a08e80e41a76084bfd6452179ed4d))
* **bot:** reduce frequency of requests made to HTB ([#146](https://www.github.com/Huskehhh/CTFd-rs/issues/146)) ([c60fc49](https://www.github.com/Huskehhh/CTFd-rs/commit/c60fc49d8594214bb5582707b6cf2724a69f8bdc))
* **ci:** add extra sccache env var ([4785a4b](https://www.github.com/Huskehhh/CTFd-rs/commit/4785a4bf6c38b934fbd764572c3635434fb97a29))
* **ci:** Adjust secrets to be loaded into docker build ([47835fd](https://www.github.com/Huskehhh/CTFd-rs/commit/47835fd248adb874ec739adae94a7e547e58de7b))
* **ci:** apply docker image caching to all targets ([#138](https://www.github.com/Huskehhh/CTFd-rs/issues/138)) ([65e49f5](https://www.github.com/Huskehhh/CTFd-rs/commit/65e49f59540c1bf89ff0925f004008ea5207226e))
* **ci:** release-please tinkering ([381a110](https://www.github.com/Huskehhh/CTFd-rs/commit/381a11091e53280fe2dd0f194c845c50fdc32cf2))
* **ci:** simplify dockerfile ([de74a41](https://www.github.com/Huskehhh/CTFd-rs/commit/de74a41d38104e5f8240100568619d8c1ccc51a0))
* **ci:** swap to use base image with sccache pre-installed ([#140](https://www.github.com/Huskehhh/CTFd-rs/issues/140)) ([10c2e13](https://www.github.com/Huskehhh/CTFd-rs/commit/10c2e133d670509742f30f21128602210ba26a82))
* **ci:** swap to use sccache on docker builds ([#139](https://www.github.com/Huskehhh/CTFd-rs/issues/139)) ([db66a76](https://www.github.com/Huskehhh/CTFd-rs/commit/db66a76771820300c71e9451cb5b8113de3e63ce))

## [0.2.0](https://www.github.com/Huskehhh/CTFd-rs/compare/v0.1.1...v0.2.0) (2021-11-03)


### Features

* **ci:** tag docker images with version stored in env ([#129](https://www.github.com/Huskehhh/CTFd-rs/issues/129)) ([76f47e5](https://www.github.com/Huskehhh/CTFd-rs/commit/76f47e506f11c440173713e51ed8fda6ce210779))


### Bug Fixes

* **ci:** adjust docker image tag ([989139e](https://www.github.com/Huskehhh/CTFd-rs/commit/989139e3aa97551db5d1ff894b5eded35ea47be7))
* **ci:** remove image tag for dockerhub ([580d1e0](https://www.github.com/Huskehhh/CTFd-rs/commit/580d1e0040905fb95e4448433f84b6f67fed16c9))
* **ci:** Swap to Oracle ARM instance for docker compilation ([0f8c767](https://www.github.com/Huskehhh/CTFd-rs/commit/0f8c76778ceb904e2a7929ea49b247fbfc53a828))
