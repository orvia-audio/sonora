# Orvia integration

Based on [sonora 0.3.0](https://github.com/dignifiedquire/sonora/tree/1632d47c1e77d0857f9145bd175134b348541e11).

Orvia adds a fixed 48 kHz mono noise-only channel with the upstream High preset. Two matching channels can share its output until their inputs diverge. The stream-copy operation then transfers the resampler, filter-bank, high-pass and noise-estimator history.

When analysis and suppression operate on the same band, the capture pipeline uses a single forward transform. Pre-echo analysis stays separate when echo cancellation can change the input.

The fixed mono capture interface releases render, downmix, and integer conversion storage when it is constructed.
