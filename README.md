# Python bindings for speedy web compiler using PyO3

:warning: Not functional - work in progress :warning:

## SWC

- [Project](https://swc.rs/)
- [Documentation](https://docs.rs/swc/latest/swc/)
- [Repository](https://github.com/swc-project/swc)

## PyO3

- [User Guide](https://pyo3.rs/v0.15.1/)
- [Documentation](https://docs.rs/pyo3/latest/pyo3/)
- [Repository](https://github.com/pyo3/pyo3)

## Set up environment

```bash
$ python3 -m venv env
$ source env/bin/activate
(env) $ pip install maturin
```

## Compile and install wheel in venv

```bash
(env) $ maturin develop
```

## Import and run library in python

```bash
(env) $ python test.py foo.js
```
