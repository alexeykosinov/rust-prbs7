# Simple Pseudorandom binary sequence implementation

Useful if need to check BER for some interface (uart, spi, etc.).

Maximum nonrepeatable values is 128 for PRBS-7.

Polynomial is:
```math
x^7 + x^6 + 1
```
