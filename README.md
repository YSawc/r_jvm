# r_jvm

[![CircleCI](https://circleci.com/gh/YSawc/r_jvm.svg?style=shield)](https://circleci.com/gh/YSawc/r_jvm)
[![](http://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

For learing jvm very thank to [ferrugo](https://github.com/maekawatoshiki/ferrugo).
This project construct with reference of [Java SE8 Edition](https://docs.oracle.com/javase/specs/jvms/se8/html/index.html).

#### Dependencies

- llvm6.0
```sh
# e.g. Ubuntu
apt-get install llvm-6.0
ln -s /usr/bin/llvm-config-6.0 /usr/bin/llvm-config
```

##### Test
```sh
cargo test -- --nocapture
```
