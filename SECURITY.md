# Security Policy

## Supported Versions

Security fixes are only made for the latest released versions of `xsd-parser` and `xsd-parser-types`.

## Reporting a Vulnerability

Please do not report security vulnerabilities in public issues. Use GitHub's private vulnerability reporting instead: open the [Security tab](https://github.com/Bergmann89/xsd-parser/security) of the repository and click "Report a vulnerability".

Please include a description of the problem, the affected versions and, if possible, a schema, configuration or XML document that reproduces it. You will get an answer as soon as possible.

Relevant are for example panics, unbounded memory usage or endless loops that can be triggered by XML documents read with the generated code, or problems in the dependencies of `xsd-parser-types`. Note that `xsd-parser` processes schemas and configurations from the developer at build time, schemas from untrusted sources should be reviewed before they are used.
