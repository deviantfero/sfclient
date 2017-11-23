# -*- coding: utf-8 -*-

from setuptools import setup, find_packages


with open('README.rst') as f:
    readme = f.read()

with open('LICENSE') as f:
    license = f.read()

setup(
    name='pythonclient',
    version='0.1.0',
    description='Python client for a C based file server',
    long_description=readme,
    author='UCA',
    url='https://github.com/deviantfero/sfclient',
    license=license,
    packages=find_packages(exclude=('tests', 'docs'))
)

