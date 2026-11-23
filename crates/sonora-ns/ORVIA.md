# Orvia integration

Based on [sonora-ns 0.3.0](https://github.com/dignifiedquire/sonora/tree/1632d47c1e77d0857f9145bd175134b348541e11).

Orvia copies the adaptive noise estimators, Wiener filter and band delay into preallocated storage when matching stereo channels diverge. The FFT and per-frame workspace stay local to each channel.

When the estimator and Wiener filter receive the same input and overlap, both use one forward transform. Skipped analysis and different overlaps retain their independent processing path.
