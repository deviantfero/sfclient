# -*- coding: utf-8 -*-

from setuptools import setup, find_packages


with open('README.rst') as f:
    readme = f.read()

with open('LICENSE') as f:
    license = f.read()

setup(
    name='sfc',
    version='0.1.0',
    description='Python client for a C based file server',
    long_description=readme,
    author='UCA',
    url='https://github.com/deviantfero/sfclient',
    license=license,
    entry_points={
        "console_scripts": ["sfc=sfc.__main__:main"]
    },
    python_requires=">=3.5",
    install_requires=[
        'posix_ipc>=1.0.0'
    ],
    packages=find_packages(exclude=('tests', 'docs'))
)
