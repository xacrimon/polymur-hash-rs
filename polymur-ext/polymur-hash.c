#include "polymur-hash.h"

void polymur_init_params_ext(PolymurHashParams* p, uint64_t k_seed, uint64_t s_seed) {
    polymur_init_params(p, k_seed, s_seed);
}

void polymur_init_params_from_seed_ext(PolymurHashParams* p, uint64_t seed) {
    polymur_init_params_from_seed(p, seed);
}

uint64_t polymur_hash_ext(const uint8_t* buf, size_t len, const PolymurHashParams* p, uint64_t tweak) {
    return polymur_hash(buf, len, p, tweak);
}

