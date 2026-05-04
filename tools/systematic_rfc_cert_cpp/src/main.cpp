#include <algorithm>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <limits>
#include <map>
#include <random>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <unordered_map>
#include <utility>
#include <vector>

namespace {

constexpr double NegInf = -std::numeric_limits<double>::infinity();

struct Options {
    std::uint64_t k0 = 1;
    int depth = -1;
    int parityExpansion = -1;
    double fieldBits = 128.0;
    double securityBits = 80.0;
    bool compare = false;
    bool monotoneThresholds = false;
    bool dumpFinalProfile = false;
    std::string certificatePath;
    std::string fullThresholdsPath;
    std::string verifyPath;
    bool idealFirstMoment = false;
    bool sampleRfcFirstMoment = false;
    bool sampleRfcProductFirstMoment = false;
    bool sampleRfcOneStep = false;
    int totalExpansion = 8;
    int prime = 5;
    int samples = 100;
    std::uint64_t seed = 1;
    std::string spectrumPath;
    std::string supportSpectrumPath;
    std::string categoryPath;
};

struct RoundState {
    std::uint64_t k = 0;
    std::uint64_t parityN = 0;
    std::vector<int> thresholds;
};

struct DistanceResult {
    int distance = 0;
    int support = 0;
};

struct CertRow {
    int round = 0;
    std::uint64_t k = 0;
    std::uint64_t parityN = 0;
    int support = 0;
    int threshold = 0;
    int distance = 0;
    double relativeDistance = 0.0;
    double worstLog2Slack = 0.0;
    int worstSupport = 0;
};

double log2Add(double lhs, double rhs) {
    if (lhs == NegInf) {
        return rhs;
    }
    if (rhs == NegInf) {
        return lhs;
    }
    if (rhs > lhs) {
        std::swap(lhs, rhs);
    }
    return lhs + std::log2(1.0 + std::exp2(rhs - lhs));
}

void log2AddTo(double& lhs, double rhs) {
    lhs = log2Add(lhs, rhs);
}

std::vector<double> log2Factorials(std::uint64_t n) {
    std::vector<double> out(static_cast<std::size_t>(n + 1), 0.0);
    for (std::uint64_t i = 2; i <= n; ++i) {
        out[static_cast<std::size_t>(i)] = out[static_cast<std::size_t>(i - 1)] + std::log2(static_cast<double>(i));
    }
    return out;
}

double log2Comb(const std::vector<double>& logFact, std::uint64_t n, std::uint64_t k) {
    if (k > n) {
        return NegInf;
    }
    return logFact[static_cast<std::size_t>(n)] - logFact[static_cast<std::size_t>(k)] -
           logFact[static_cast<std::size_t>(n - k)];
}

std::vector<double> log2Combinations(std::uint64_t n) {
    auto fact = log2Factorials(n);
    std::vector<double> out(static_cast<std::size_t>(n + 1), 0.0);
    for (std::uint64_t k = 0; k <= n; ++k) {
        out[static_cast<std::size_t>(k)] = log2Comb(fact, n, k);
    }
    return out;
}

double log2OneMinusQInv(double fieldBits) {
    if (fieldBits > 64.0) {
        return 0.0;
    }
    return std::log2(1.0 - std::exp2(-fieldBits));
}

double log2BadRootProb(double fieldBits) {
    return std::log2(2.002) - fieldBits;
}

double log2OutsideTail(
    std::uint64_t n,
    std::uint64_t commonZeros,
    std::uint64_t neededZeros,
    double logBadProb,
    const std::vector<double>& logFactN) {
    const auto available = n - commonZeros;
    if (neededZeros == 0) {
        return 0.0;
    }
    if (neededZeros > available) {
        return NegInf;
    }
    return log2Comb(logFactN, available, neededZeros) + static_cast<double>(neededZeros) * logBadProb;
}

double log2KBound(
    const std::vector<double>& logCombK,
    const std::vector<int>& thresholds,
    double fieldBits,
    std::uint64_t support,
    std::uint64_t zeroCount) {
    if (support == 0) {
        return 0.0;
    }
    if (zeroCount >= static_cast<std::uint64_t>(thresholds[static_cast<std::size_t>(support)])) {
        return NegInf;
    }

    const auto supportChoices = logCombK[static_cast<std::size_t>(support)];
    const auto kernelBound =
        supportChoices +
        static_cast<double>(thresholds[static_cast<std::size_t>(support)] - static_cast<int>(zeroCount)) * fieldBits;
    const auto exactSupportUniverse =
        supportChoices + static_cast<double>(support) * (fieldBits + log2OneMinusQInv(fieldBits));
    return std::min(kernelBound, exactSupportUniverse);
}

std::vector<int> prefixMaxThresholds(const std::vector<int>& thresholds) {
    auto out = thresholds;
    if (out.size() <= 2) {
        return out;
    }
    auto current = out[1];
    for (std::size_t i = 2; i < out.size(); ++i) {
        current = std::max(current, out[i]);
        out[i] = current;
    }
    return out;
}

std::vector<double> buildSplitTerms(
    std::uint64_t k,
    std::uint64_t n,
    const std::vector<int>& prevThresholds,
    double fieldBits,
    const std::vector<double>& logCombK,
    const std::vector<double>& logCombN) {
    const auto parentK = 2 * k;
    const auto cols = n + 1;
    std::vector<double> splitTerms(static_cast<std::size_t>((parentK + 1) * cols), NegInf);
    std::vector<std::pair<std::uint64_t, double>> active;
    active.reserve(static_cast<std::size_t>(k + 1));

    for (std::uint64_t a = 0; a <= n; ++a) {
        active.clear();
        for (std::uint64_t u = 0; u <= k; ++u) {
            const auto bound = log2KBound(logCombK, prevThresholds, fieldBits, u, a);
            if (bound != NegInf) {
                active.emplace_back(u, bound);
            }
        }

        const auto commonZeroChoices = logCombN[static_cast<std::size_t>(a)];
        for (const auto& left : active) {
            for (const auto& right : active) {
                const auto support = left.first + right.first;
                const auto term = commonZeroChoices + left.second + right.second;
                auto& cell = splitTerms[static_cast<std::size_t>(support * cols + a)];
                log2AddTo(cell, term);
            }
        }
    }
    return splitTerms;
}

double log2FailureFromSplitTerms(
    std::uint64_t n,
    double fieldBits,
    const std::vector<double>& splitTerms,
    std::uint64_t support,
    int threshold,
    const std::vector<double>& logFactN) {
    const auto cols = n + 1;
    const auto logBadProb = log2BadRootProb(fieldBits);
    double total = NegInf;

    for (std::uint64_t a = 0; a <= n; ++a) {
        auto term = splitTerms[static_cast<std::size_t>(support * cols + a)];
        if (term == NegInf) {
            continue;
        }
        const auto forced = 2 * a;
        if (threshold > 0 && static_cast<std::uint64_t>(threshold) > forced) {
            const auto needed = static_cast<std::uint64_t>(threshold) - forced;
            const auto tail = log2OutsideTail(n, a, needed, logBadProb, logFactN);
            if (tail == NegInf) {
                continue;
            }
            term += tail;
        }
        log2AddTo(total, term);
    }
    return total;
}

std::vector<int> nextThresholds(
    std::uint64_t k,
    int parityExpansion,
    const std::vector<int>& thresholds,
    double fieldBits,
    double securityBits,
    bool monotone) {
    const auto prev = monotone ? prefixMaxThresholds(thresholds) : thresholds;
    const auto n = static_cast<std::uint64_t>(parityExpansion) * k;
    const auto parentN = 2 * n;
    const auto parentK = 2 * k;
    const auto perSupportBudget = -securityBits - std::log2(static_cast<double>(parentK));

    const auto logCombK = log2Combinations(k);
    const auto logCombN = log2Combinations(n);
    const auto logFactN = log2Factorials(n);
    const auto splitTerms = buildSplitTerms(k, n, prev, fieldBits, logCombK, logCombN);

    std::vector<int> next(static_cast<std::size_t>(parentK + 1), static_cast<int>(parentN + 1));
    next[0] = static_cast<int>(parentN + 1);

    for (std::uint64_t support = 1; support <= parentK; ++support) {
        int lo = 1;
        int hi = static_cast<int>(parentN);
        while (lo < hi) {
            const auto mid = (lo + hi) >> 1;
            const auto failure = log2FailureFromSplitTerms(n, fieldBits, splitTerms, support, mid, logFactN);
            if (failure <= perSupportBudget) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        const auto failure = log2FailureFromSplitTerms(n, fieldBits, splitTerms, support, lo, logFactN);
        if (failure > perSupportBudget) {
            std::ostringstream ss;
            ss << "no admissible threshold for parent support " << support;
            throw std::runtime_error(ss.str());
        }
        next[static_cast<std::size_t>(support)] = lo;
    }
    return next;
}

DistanceResult systematicDistance(std::uint64_t k, std::uint64_t n, const std::vector<int>& thresholds) {
    DistanceResult best;
    best.distance = static_cast<int>(k + n + 1);
    for (std::uint64_t support = 1; support <= k; ++support) {
        const auto distance = static_cast<int>(support + n - static_cast<std::uint64_t>(thresholds[static_cast<std::size_t>(support)] - 1));
        if (distance < best.distance) {
            best.distance = distance;
            best.support = static_cast<int>(support);
        }
    }
    return best;
}

std::pair<double, int> theoremConditionReport(
    std::uint64_t k,
    std::uint64_t n,
    const std::vector<int>& prevThresholds,
    const std::vector<int>& next,
    double fieldBits,
    double securityBits) {
    const auto parentK = 2 * k;
    const auto perSupportBudget = -securityBits - std::log2(static_cast<double>(parentK));
    const auto logCombK = log2Combinations(k);
    const auto logCombN = log2Combinations(n);
    const auto logFactN = log2Factorials(n);
    const auto splitTerms = buildSplitTerms(k, n, prevThresholds, fieldBits, logCombK, logCombN);

    double worstFailure = NegInf;
    int worstSupport = 0;
    for (std::uint64_t support = 1; support <= parentK; ++support) {
        const auto failure =
            log2FailureFromSplitTerms(n, fieldBits, splitTerms, support, next[static_cast<std::size_t>(support)], logFactN);
        if (failure > worstFailure) {
            worstFailure = failure;
            worstSupport = static_cast<int>(support);
        }
    }
    return {worstFailure - perSupportBudget, worstSupport};
}

int basefoldGlobalNextThreshold(std::uint64_t n, int prevThreshold, double fieldBits, double securityBits) {
    const auto denominator = fieldBits - 1.001;
    if (denominator <= 0.0) {
        throw std::runtime_error("field_bits must exceed 1.001");
    }
    const auto ell =
        (2.0 * std::log2(static_cast<double>(n)) + securityBits + 2.002 * prevThreshold + 0.6 * n) / denominator;
    return 2 * prevThreshold + static_cast<int>(std::ceil(ell));
}

double basefoldGlobalDistance(int depth, int expansion, double fieldBits, double securityBits, std::uint64_t k0) {
    auto k = k0;
    auto n = static_cast<std::uint64_t>(expansion) * k;
    auto threshold = static_cast<int>(k0);
    for (int i = 1; i <= depth; ++i) {
        threshold = basefoldGlobalNextThreshold(n, threshold, fieldBits, securityBits);
        k *= 2;
        n *= 2;
    }
    const auto distance = static_cast<double>(n - static_cast<std::uint64_t>(threshold - 1));
    return distance / static_cast<double>(n);
}

double log2ExpectedRandomLinearLowWeight(
    std::uint64_t k,
    std::uint64_t n,
    int maxWeight,
    double fieldBits,
    const std::vector<double>& logFactN) {
    const auto logQMinusOne = fieldBits + log2OneMinusQInv(fieldBits);
    const auto codeRateTerm = static_cast<double>(k) * fieldBits - static_cast<double>(n) * fieldBits;
    double total = NegInf;
    for (int h = 1; h <= maxWeight && h <= static_cast<int>(n); ++h) {
        const auto term = log2Comb(logFactN, n, static_cast<std::uint64_t>(h)) +
                          static_cast<double>(h) * logQMinusOne + codeRateTerm;
        log2AddTo(total, term);
    }
    return total;
}

double log2UniformVectorLowWeightProb(
    std::uint64_t n,
    int maxWeight,
    double fieldBits,
    const std::vector<double>& logFactN) {
    if (maxWeight < 0) {
        return NegInf;
    }
    const auto logQMinusOne = fieldBits + log2OneMinusQInv(fieldBits);
    double total = NegInf;
    const auto capped = std::min<std::uint64_t>(n, static_cast<std::uint64_t>(maxWeight));
    for (std::uint64_t h = 0; h <= capped; ++h) {
        const auto term = log2Comb(logFactN, n, h) + static_cast<double>(h) * logQMinusOne -
                          static_cast<double>(n) * fieldBits;
        log2AddTo(total, term);
    }
    return total;
}

double log2ExpectedSystematicRandomParityLowWeight(
    std::uint64_t k,
    std::uint64_t parityN,
    int maxWeight,
    double fieldBits,
    const std::vector<double>& logFactK,
    const std::vector<double>& logFactParity) {
    const auto logQMinusOne = fieldBits + log2OneMinusQInv(fieldBits);
    double total = NegInf;
    const auto maxSupport = std::min<std::uint64_t>(k, static_cast<std::uint64_t>(std::max(maxWeight, 0)));
    for (std::uint64_t s = 1; s <= maxSupport; ++s) {
        const auto parityBudget = maxWeight - static_cast<int>(s);
        const auto parityTail = log2UniformVectorLowWeightProb(parityN, parityBudget, fieldBits, logFactParity);
        if (parityTail == NegInf) {
            continue;
        }
        const auto messageCount =
            log2Comb(logFactK, k, s) + static_cast<double>(s) * logQMinusOne;
        log2AddTo(total, messageCount + parityTail);
    }
    return total;
}

int largestDistanceBelowBudget(
    std::uint64_t totalN,
    const std::function<double(int)>& logExpectation,
    double securityBits) {
    int best = 0;
    const auto budget = -securityBits;
    for (int d = 1; d <= static_cast<int>(totalN); ++d) {
        if (logExpectation(d) <= budget) {
            best = d;
        } else {
            break;
        }
    }
    return best;
}

int randomLinearFirstMomentDistance(
    std::uint64_t k,
    std::uint64_t n,
    double fieldBits,
    double securityBits,
    const std::vector<double>& logFactN) {
    const auto logQMinusOne = fieldBits + log2OneMinusQInv(fieldBits);
    const auto codeRateTerm = static_cast<double>(k) * fieldBits - static_cast<double>(n) * fieldBits;
    const auto budget = -securityBits;
    double cumulative = NegInf;
    int best = 0;
    for (std::uint64_t h = 1; h <= n; ++h) {
        const auto term = log2Comb(logFactN, n, h) + static_cast<double>(h) * logQMinusOne + codeRateTerm;
        log2AddTo(cumulative, term);
        if (cumulative <= budget) {
            best = static_cast<int>(h);
        } else {
            break;
        }
    }
    return best;
}

int systematicRandomParityFirstMomentDistance(
    std::uint64_t k,
    std::uint64_t parityN,
    double fieldBits,
    double securityBits,
    const std::vector<double>& logFactK,
    const std::vector<double>& logFactParity) {
    const auto totalN = k + parityN;
    const auto logQMinusOne = fieldBits + log2OneMinusQInv(fieldBits);
    const auto budget = -securityBits;
    std::vector<double> exact(static_cast<std::size_t>(totalN + 1), NegInf);

    for (std::uint64_t support = 1; support <= k; ++support) {
        const auto messageCount =
            log2Comb(logFactK, k, support) + static_cast<double>(support) * logQMinusOne;
        for (std::uint64_t parityWeight = 0; parityWeight <= parityN; ++parityWeight) {
            const auto parityMass =
                log2Comb(logFactParity, parityN, parityWeight) +
                static_cast<double>(parityWeight) * logQMinusOne -
                static_cast<double>(parityN) * fieldBits;
            auto& cell = exact[static_cast<std::size_t>(support + parityWeight)];
            log2AddTo(cell, messageCount + parityMass);
        }
    }

    double cumulative = NegInf;
    int best = 0;
    for (std::uint64_t d = 1; d <= totalN; ++d) {
        log2AddTo(cumulative, exact[static_cast<std::size_t>(d)]);
        if (cumulative <= budget) {
            best = static_cast<int>(d);
        } else {
            break;
        }
    }
    return best;
}

int runIdealFirstMoment(const Options& opts) {
    if (opts.depth < 0) {
        throw std::runtime_error("--depth is required for --ideal-first-moment");
    }
    if (opts.totalExpansion <= 1) {
        throw std::runtime_error("--total-expansion must be greater than 1");
    }
    if (opts.k0 != 1) {
        throw std::runtime_error("--ideal-first-moment currently assumes k0=1");
    }

    std::cout
        << "depth,k,total_n,parity_n,security_bits,"
        << "random_linear_distance,random_linear_relative,"
        << "systematic_random_parity_distance,systematic_random_parity_relative\n";

    for (int depth = 1; depth <= opts.depth; ++depth) {
        const auto k = opts.k0 << depth;
        const auto totalN = static_cast<std::uint64_t>(opts.totalExpansion) * k;
        const auto parityN = totalN - k;
        const auto securityBits = opts.securityBits + std::ceil(std::log2(static_cast<double>(depth)));
        const auto logFactTotal = log2Factorials(totalN);
        const auto logFactK = log2Factorials(k);
        const auto logFactParity = log2Factorials(parityN);

        const auto randomDistance =
            randomLinearFirstMomentDistance(k, totalN, opts.fieldBits, securityBits, logFactTotal);
        const auto systematicDistance = systematicRandomParityFirstMomentDistance(
            k,
            parityN,
            opts.fieldBits,
            securityBits,
            logFactK,
            logFactParity);

        std::cout << depth << ',' << k << ',' << totalN << ',' << parityN << ',' << securityBits << ','
                  << randomDistance << ',' << std::fixed << std::setprecision(8)
                  << static_cast<double>(randomDistance) / static_cast<double>(totalN) << ','
                  << systematicDistance << ','
                  << static_cast<double>(systematicDistance) / static_cast<double>(totalN) << '\n';
    }
    return 0;
}

int modPrime(int value, int prime) {
    value %= prime;
    return value < 0 ? value + prime : value;
}

std::vector<int> rfcGeneratorPrime(
    int depth,
    int expansion,
    int prime,
    std::mt19937_64& rng) {
    std::uint64_t k = 1;
    std::uint64_t n = static_cast<std::uint64_t>(expansion);
    std::vector<int> generator(static_cast<std::size_t>(k * n), 1);
    std::uniform_int_distribution<int> nonzero(1, prime - 1);

    for (int round = 0; round < depth; ++round) {
        const auto nextK = 2 * k;
        const auto nextN = 2 * n;
        std::vector<int> next(static_cast<std::size_t>(nextK * nextN), 0);
        std::vector<int> diagonal(static_cast<std::size_t>(n));
        for (std::uint64_t j = 0; j < n; ++j) {
            diagonal[static_cast<std::size_t>(j)] = nonzero(rng);
        }

        for (std::uint64_t row = 0; row < k; ++row) {
            const auto topRow = row;
            const auto bottomRow = row + k;
            for (std::uint64_t j = 0; j < n; ++j) {
                const auto entry = generator[static_cast<std::size_t>(row * n + j)];
                const auto t = diagonal[static_cast<std::size_t>(j)];
                next[static_cast<std::size_t>(topRow * nextN + j)] = modPrime((1 - t) * entry, prime);
                next[static_cast<std::size_t>(topRow * nextN + n + j)] = modPrime((1 - (t + 1)) * entry, prime);
                next[static_cast<std::size_t>(bottomRow * nextN + j)] = modPrime(t * entry, prime);
                next[static_cast<std::size_t>(bottomRow * nextN + n + j)] = modPrime((t + 1) * entry, prime);
            }
        }
        generator = std::move(next);
        k = nextK;
        n = nextN;
    }
    return generator;
}

bool incrementMessage(std::vector<int>& message, int prime) {
    for (auto& digit : message) {
        ++digit;
        if (digit < prime) {
            return true;
        }
        digit = 0;
    }
    return false;
}

std::pair<int, int> messageSupportAndEncodedWeight(
    const std::vector<int>& message,
    const std::vector<int>& generator,
    std::uint64_t n,
    int prime,
    std::vector<int>& scratch) {
    std::fill(scratch.begin(), scratch.end(), 0);
    int support = 0;
    for (std::uint64_t row = 0; row < message.size(); ++row) {
        const auto value = message[static_cast<std::size_t>(row)];
        if (value == 0) {
            continue;
        }
        ++support;
        const auto* rowPtr = generator.data() + static_cast<std::ptrdiff_t>(row * n);
        for (std::uint64_t j = 0; j < n; ++j) {
            scratch[static_cast<std::size_t>(j)] =
                (scratch[static_cast<std::size_t>(j)] + value * rowPtr[j]) % prime;
        }
    }
    int encodedWeight = 0;
    for (const auto value : scratch) {
        encodedWeight += value != 0;
    }
    return {support, encodedWeight};
}

struct SampleSpectrum {
    std::vector<double> oldSpectrum;
    std::vector<double> systematicSpectrum;
    std::vector<double> oldBySupport;
    std::vector<double> systematicBySupport;
    std::uint64_t supportCols = 0;
    double oldMinSum = 0.0;
    double systematicMinSum = 0.0;
};

SampleSpectrum sampleRfcSpectra(const Options& opts) {
    if (opts.depth < 0) {
        throw std::runtime_error("--depth is required for --sample-rfc-first-moment");
    }
    if (opts.totalExpansion <= 1) {
        throw std::runtime_error("--total-expansion must be greater than 1");
    }
    if (opts.prime <= 2) {
        throw std::runtime_error("--prime must be an odd prime for this sampler");
    }
    if (opts.samples <= 0) {
        throw std::runtime_error("--samples must be positive");
    }

    const auto k = std::uint64_t{1} << opts.depth;
    const auto totalN = static_cast<std::uint64_t>(opts.totalExpansion) * k;
    const auto parityN = static_cast<std::uint64_t>(opts.totalExpansion - 1) * k;
    SampleSpectrum result;
    result.oldSpectrum.assign(static_cast<std::size_t>(totalN + 1), 0.0);
    result.systematicSpectrum.assign(static_cast<std::size_t>(totalN + 1), 0.0);
    result.supportCols = totalN + 1;
    result.oldBySupport.assign(static_cast<std::size_t>((k + 1) * result.supportCols), 0.0);
    result.systematicBySupport.assign(static_cast<std::size_t>((k + 1) * result.supportCols), 0.0);

    std::mt19937_64 rng(opts.seed);
    std::vector<int> message(static_cast<std::size_t>(k), 0);
    std::vector<int> oldScratch(static_cast<std::size_t>(totalN), 0);
    std::vector<int> sysScratch(static_cast<std::size_t>(parityN), 0);

    for (int sample = 0; sample < opts.samples; ++sample) {
        const auto oldGenerator = rfcGeneratorPrime(opts.depth, opts.totalExpansion, opts.prime, rng);
        const auto sysParityGenerator = rfcGeneratorPrime(opts.depth, opts.totalExpansion - 1, opts.prime, rng);
        std::fill(message.begin(), message.end(), 0);

        int oldMin = static_cast<int>(totalN + 1);
        int sysMin = static_cast<int>(totalN + 1);
        while (incrementMessage(message, opts.prime)) {
            const auto oldPair =
                messageSupportAndEncodedWeight(message, oldGenerator, totalN, opts.prime, oldScratch);
            const auto sysPair =
                messageSupportAndEncodedWeight(message, sysParityGenerator, parityN, opts.prime, sysScratch);
            const auto oldWeight = oldPair.second;
            const auto sysWeight = sysPair.first + sysPair.second;
            result.oldSpectrum[static_cast<std::size_t>(oldWeight)] += 1.0;
            result.systematicSpectrum[static_cast<std::size_t>(sysWeight)] += 1.0;
            result.oldBySupport[static_cast<std::size_t>(oldPair.first * result.supportCols + oldWeight)] += 1.0;
            result.systematicBySupport[static_cast<std::size_t>(sysPair.first * result.supportCols + sysWeight)] += 1.0;
            oldMin = std::min(oldMin, oldWeight);
            sysMin = std::min(sysMin, sysWeight);
        }
        result.oldMinSum += oldMin;
        result.systematicMinSum += sysMin;

        if (opts.samples >= 10 && (sample + 1) % std::max(1, opts.samples / 10) == 0) {
            std::cerr << "sample=" << (sample + 1) << "/" << opts.samples << '\n';
        }
    }

    for (auto& value : result.oldSpectrum) {
        value /= static_cast<double>(opts.samples);
    }
    for (auto& value : result.systematicSpectrum) {
        value /= static_cast<double>(opts.samples);
    }
    for (auto& value : result.oldBySupport) {
        value /= static_cast<double>(opts.samples);
    }
    for (auto& value : result.systematicBySupport) {
        value /= static_cast<double>(opts.samples);
    }
    result.oldMinSum /= static_cast<double>(opts.samples);
    result.systematicMinSum /= static_cast<double>(opts.samples);
    return result;
}

int firstCumulativeAtLeastOne(const std::vector<double>& spectrum) {
    double total = 0.0;
    for (std::size_t i = 1; i < spectrum.size(); ++i) {
        total += spectrum[i];
        if (total >= 1.0) {
            return static_cast<int>(i);
        }
    }
    return -1;
}

std::uint64_t integerPow(std::uint64_t base, std::uint64_t exponent) {
    std::uint64_t out = 1;
    while (exponent-- != 0) {
        out *= base;
    }
    return out;
}

std::vector<double> convolveDistribution(
    const std::vector<double>& lhs,
    const std::vector<double>& rhs,
    std::uint64_t maxDegree) {
    std::vector<double> out(static_cast<std::size_t>(maxDegree + 1), 0.0);
    for (std::uint64_t i = 0; i < lhs.size(); ++i) {
        const auto left = lhs[static_cast<std::size_t>(i)];
        if (left == 0.0) {
            continue;
        }
        const auto maxJ = std::min<std::uint64_t>(rhs.size() - 1, maxDegree - i);
        for (std::uint64_t j = 0; j <= maxJ; ++j) {
            const auto right = rhs[static_cast<std::size_t>(j)];
            if (right == 0.0) {
                continue;
            }
            out[static_cast<std::size_t>(i + j)] += left * right;
        }
    }
    return out;
}

std::vector<double> powerDistribution(
    const std::vector<double>& base,
    int exponent,
    std::uint64_t maxDegree) {
    std::vector<double> out(static_cast<std::size_t>(maxDegree + 1), 0.0);
    out[0] = 1.0;
    auto cur = base;
    auto exp = exponent;
    while (exp != 0) {
        if ((exp & 1) != 0) {
            out = convolveDistribution(out, cur, maxDegree);
        }
        exp >>= 1;
        if (exp != 0) {
            cur = convolveDistribution(cur, cur, maxDegree);
        }
    }
    return out;
}

SampleSpectrum sampleRfcProductSpectra(const Options& opts) {
    if (opts.depth < 0) {
        throw std::runtime_error("--depth is required for --sample-rfc-product-first-moment");
    }
    if (opts.totalExpansion <= 1) {
        throw std::runtime_error("--total-expansion must be greater than 1");
    }
    if (opts.prime <= 2) {
        throw std::runtime_error("--prime must be an odd prime for this sampler");
    }
    if (opts.samples <= 0) {
        throw std::runtime_error("--samples must be positive");
    }

    const auto k = std::uint64_t{1} << opts.depth;
    const auto treeN = k;
    const auto totalN = static_cast<std::uint64_t>(opts.totalExpansion) * k;
    const auto numMessages = integerPow(static_cast<std::uint64_t>(opts.prime), k);

    SampleSpectrum result;
    result.oldSpectrum.assign(static_cast<std::size_t>(totalN + 1), 0.0);
    result.systematicSpectrum.assign(static_cast<std::size_t>(totalN + 1), 0.0);
    result.supportCols = totalN + 1;
    result.oldBySupport.assign(static_cast<std::size_t>((k + 1) * result.supportCols), 0.0);
    result.systematicBySupport.assign(static_cast<std::size_t>((k + 1) * result.supportCols), 0.0);

    const auto singleCols = treeN + 1;
    std::vector<std::uint32_t> singleCounts(static_cast<std::size_t>(numMessages * singleCols), 0);
    std::vector<std::uint8_t> supports(static_cast<std::size_t>(numMessages), 0);
    std::mt19937_64 rng(opts.seed);
    std::vector<int> message(static_cast<std::size_t>(k), 0);
    std::vector<int> scratch(static_cast<std::size_t>(treeN), 0);

    for (int sample = 0; sample < opts.samples; ++sample) {
        const auto generator = rfcGeneratorPrime(opts.depth, 1, opts.prime, rng);
        std::fill(message.begin(), message.end(), 0);
        std::uint64_t messageIndex = 0;
        while (incrementMessage(message, opts.prime)) {
            ++messageIndex;
            const auto [support, encodedWeight] =
                messageSupportAndEncodedWeight(message, generator, treeN, opts.prime, scratch);
            supports[static_cast<std::size_t>(messageIndex)] = static_cast<std::uint8_t>(support);
            ++singleCounts[static_cast<std::size_t>(messageIndex * singleCols + encodedWeight)];
        }
        if (messageIndex + 1 != numMessages) {
            throw std::runtime_error("message enumeration count mismatch");
        }
        if (opts.samples >= 10 && (sample + 1) % std::max(1, opts.samples / 10) == 0) {
            std::cerr << "single_tree_sample=" << (sample + 1) << "/" << opts.samples << '\n';
        }
    }

    std::vector<double> single(static_cast<std::size_t>(singleCols), 0.0);
    const auto invSamples = 1.0 / static_cast<double>(opts.samples);
    for (std::uint64_t messageIndex = 1; messageIndex < numMessages; ++messageIndex) {
        const auto support = static_cast<std::uint64_t>(supports[static_cast<std::size_t>(messageIndex)]);
        std::fill(single.begin(), single.end(), 0.0);
        for (std::uint64_t h = 0; h <= treeN; ++h) {
            single[static_cast<std::size_t>(h)] =
                static_cast<double>(singleCounts[static_cast<std::size_t>(messageIndex * singleCols + h)]) *
                invSamples;
        }

        const auto oldDist = powerDistribution(single, opts.totalExpansion, totalN);
        const auto sysDist = powerDistribution(single, opts.totalExpansion - 1, totalN);
        for (std::uint64_t h = 0; h <= totalN; ++h) {
            const auto oldValue = oldDist[static_cast<std::size_t>(h)];
            if (oldValue != 0.0) {
                result.oldSpectrum[static_cast<std::size_t>(h)] += oldValue;
                result.oldBySupport[static_cast<std::size_t>(support * result.supportCols + h)] += oldValue;
            }
            if (support + h <= totalN) {
                const auto sysValue = sysDist[static_cast<std::size_t>(h)];
                if (sysValue != 0.0) {
                    result.systematicSpectrum[static_cast<std::size_t>(support + h)] += sysValue;
                    result.systematicBySupport[
                        static_cast<std::size_t>(support * result.supportCols + support + h)] += sysValue;
                }
            }
        }
    }

    return result;
}

int runSampleRfcProductFirstMoment(const Options& opts) {
    const auto k = std::uint64_t{1} << opts.depth;
    const auto totalN = static_cast<std::uint64_t>(opts.totalExpansion) * k;
    const auto spectra = sampleRfcProductSpectra(opts);
    std::cout << "p=" << opts.prime << " depth=" << opts.depth << " k=" << k << " total_n=" << totalN
              << " single_tree_samples=" << opts.samples << " total_expansion=" << opts.totalExpansion << '\n';
    std::cout << "old_product_first_moment_crossing=" << firstCumulativeAtLeastOne(spectra.oldSpectrum) << '\n';
    std::cout << "systematic_product_first_moment_crossing="
              << firstCumulativeAtLeastOne(spectra.systematicSpectrum) << '\n';
    std::cout << "weight,old_expected_count,systematic_expected_count\n";
    for (std::size_t h = 0; h < spectra.oldSpectrum.size(); ++h) {
        if (spectra.oldSpectrum[h] != 0.0 || spectra.systematicSpectrum[h] != 0.0) {
            std::cout << h << ',' << std::setprecision(12) << spectra.oldSpectrum[h] << ','
                      << spectra.systematicSpectrum[h] << '\n';
        }
    }

    if (!opts.spectrumPath.empty()) {
        std::ofstream out(opts.spectrumPath);
        if (!out) {
            throw std::runtime_error("failed to open spectrum output path");
        }
        out << "weight,old_expected_count,systematic_expected_count\n";
        out << std::setprecision(17);
        for (std::size_t h = 0; h < spectra.oldSpectrum.size(); ++h) {
            out << h << ',' << spectra.oldSpectrum[h] << ',' << spectra.systematicSpectrum[h] << '\n';
        }
    }
    if (!opts.supportSpectrumPath.empty()) {
        std::ofstream out(opts.supportSpectrumPath);
        if (!out) {
            throw std::runtime_error("failed to open support spectrum output path");
        }
        out << "support,weight,old_expected_count,systematic_expected_count\n";
        out << std::setprecision(17);
        for (std::uint64_t support = 1; support <= k; ++support) {
            for (std::uint64_t h = 0; h < spectra.oldSpectrum.size(); ++h) {
                out << support << ',' << h << ','
                    << spectra.oldBySupport[static_cast<std::size_t>(support * spectra.supportCols + h)] << ','
                    << spectra.systematicBySupport[static_cast<std::size_t>(support * spectra.supportCols + h)]
                    << '\n';
            }
        }
    }
    return 0;
}

int runSampleRfcFirstMoment(const Options& opts) {
    const auto k = std::uint64_t{1} << opts.depth;
    const auto totalN = static_cast<std::uint64_t>(opts.totalExpansion) * k;
    const auto spectra = sampleRfcSpectra(opts);
    std::cout << "p=" << opts.prime << " depth=" << opts.depth << " k=" << k << " total_n=" << totalN
              << " samples=" << opts.samples << " total_expansion=" << opts.totalExpansion << '\n';
    std::cout << "old_average_min_distance=" << std::fixed << std::setprecision(6) << spectra.oldMinSum
              << " relative=" << std::setprecision(8) << spectra.oldMinSum / static_cast<double>(totalN)
              << " first_moment_crossing=" << firstCumulativeAtLeastOne(spectra.oldSpectrum) << '\n';
    std::cout << "systematic_average_min_distance=" << std::fixed << std::setprecision(6)
              << spectra.systematicMinSum << " relative=" << std::setprecision(8)
              << spectra.systematicMinSum / static_cast<double>(totalN)
              << " first_moment_crossing=" << firstCumulativeAtLeastOne(spectra.systematicSpectrum) << '\n';
    std::cout << "weight,old_expected_count,systematic_expected_count\n";
    for (std::size_t h = 0; h < spectra.oldSpectrum.size(); ++h) {
        if (spectra.oldSpectrum[h] != 0.0 || spectra.systematicSpectrum[h] != 0.0) {
            std::cout << h << ',' << std::setprecision(12) << spectra.oldSpectrum[h] << ','
                      << spectra.systematicSpectrum[h] << '\n';
        }
    }

    if (!opts.spectrumPath.empty()) {
        std::ofstream out(opts.spectrumPath);
        if (!out) {
            throw std::runtime_error("failed to open spectrum output path");
        }
        out << "weight,old_expected_count,systematic_expected_count\n";
        out << std::setprecision(17);
        for (std::size_t h = 0; h < spectra.oldSpectrum.size(); ++h) {
            out << h << ',' << spectra.oldSpectrum[h] << ',' << spectra.systematicSpectrum[h] << '\n';
        }
    }
    if (!opts.supportSpectrumPath.empty()) {
        std::ofstream out(opts.supportSpectrumPath);
        if (!out) {
            throw std::runtime_error("failed to open support spectrum output path");
        }
        out << "support,weight,old_expected_count,systematic_expected_count\n";
        out << std::setprecision(17);
        const auto messageCount = std::uint64_t{1} << opts.depth;
        for (std::uint64_t support = 1; support <= messageCount; ++support) {
            for (std::uint64_t h = 0; h < spectra.oldSpectrum.size(); ++h) {
                out << support << ',' << h << ','
                    << spectra.oldBySupport[static_cast<std::size_t>(support * spectra.supportCols + h)] << ','
                    << spectra.systematicBySupport[static_cast<std::size_t>(support * spectra.supportCols + h)]
                    << '\n';
            }
        }
    }
    return 0;
}

struct EncodedMessage {
    int support = 0;
    std::vector<int> word;
};

std::vector<EncodedMessage> enumerateEncodedMessages(
    const std::vector<int>& generator,
    std::uint64_t k,
    std::uint64_t n,
    int prime) {
    std::vector<EncodedMessage> out;
    std::vector<int> message(static_cast<std::size_t>(k), 0);
    std::vector<int> scratch(static_cast<std::size_t>(n), 0);
    const auto totalMessages = static_cast<std::size_t>(std::round(std::pow(static_cast<double>(prime), static_cast<double>(k))));
    out.reserve(totalMessages);

    do {
        auto [support, encodedWeight] = messageSupportAndEncodedWeight(message, generator, n, prime, scratch);
        (void)encodedWeight;
        out.push_back(EncodedMessage{support, scratch});
    } while (incrementMessage(message, prime));

    return out;
}

std::vector<double> binomialPmf(int count, double probability) {
    std::vector<double> out(static_cast<std::size_t>(count + 1), 0.0);
    if (count == 0) {
        out[0] = 1.0;
        return out;
    }
    const auto logP = std::log(probability);
    const auto logQ = std::log(1.0 - probability);
    for (int s = 0; s <= count; ++s) {
        const auto logValue =
            std::lgamma(static_cast<double>(count + 1)) -
            std::lgamma(static_cast<double>(s + 1)) -
            std::lgamma(static_cast<double>(count - s + 1)) +
            static_cast<double>(s) * logP +
            static_cast<double>(count - s) * logQ;
        out[static_cast<std::size_t>(s)] = std::exp(logValue);
    }
    return out;
}

std::vector<double> localParentWeightDistribution(
    int equalNonzero,
    int singleRoot,
    int doubleRoot,
    int prime) {
    const auto p1 = 1.0 / static_cast<double>(prime - 1);
    const auto p2 = 2.0 / static_cast<double>(prime - 1);
    const auto single = binomialPmf(singleRoot, p1);
    const auto generic = binomialPmf(doubleRoot, p2);
    const auto maxWeight = 2 * (equalNonzero + singleRoot + doubleRoot);
    std::vector<double> out(static_cast<std::size_t>(maxWeight + 1), 0.0);
    for (int a = 0; a <= singleRoot; ++a) {
        for (int b = 0; b <= doubleRoot; ++b) {
            const auto weight = maxWeight - a - b;
            out[static_cast<std::size_t>(weight)] +=
                single[static_cast<std::size_t>(a)] * generic[static_cast<std::size_t>(b)];
        }
    }
    return out;
}

std::uint64_t categoryKey(int equalNonzero, int singleRoot, int doubleRoot) {
    return static_cast<std::uint64_t>(equalNonzero) |
           (static_cast<std::uint64_t>(singleRoot) << 16) |
           (static_cast<std::uint64_t>(doubleRoot) << 32);
}

std::uint64_t categorySupportKey(int support, int equalNonzero, int singleRoot, int doubleRoot) {
    return static_cast<std::uint64_t>(support) |
           (static_cast<std::uint64_t>(equalNonzero) << 16) |
           (static_cast<std::uint64_t>(singleRoot) << 32) |
           (static_cast<std::uint64_t>(doubleRoot) << 48);
}

void accumulateOneStepPairSpectrum(
    const std::vector<EncodedMessage>& messages,
    std::uint64_t childN,
    int prime,
    bool systematic,
    std::vector<double>& spectrum,
    std::unordered_map<std::uint64_t, std::vector<double>>& localCache,
    std::unordered_map<std::uint64_t, double>* categoryCounts) {
    for (const auto& left : messages) {
        for (const auto& right : messages) {
            if (left.support == 0 && right.support == 0) {
                continue;
            }

            int equalNonzero = 0;
            int singleRoot = 0;
            int doubleRoot = 0;
            for (std::uint64_t j = 0; j < childN; ++j) {
                const auto l = left.word[static_cast<std::size_t>(j)];
                const auto r = right.word[static_cast<std::size_t>(j)];
                if (l == 0 && r == 0) {
                    continue;
                }
                if (l == r) {
                    ++equalNonzero;
                } else if (l == 0 || r == 0) {
                    ++singleRoot;
                } else {
                    ++doubleRoot;
                }
            }

            const auto parentSupport = left.support + right.support;
            const auto key = categoryKey(equalNonzero, singleRoot, doubleRoot);
            if (categoryCounts != nullptr) {
                (*categoryCounts)[categorySupportKey(parentSupport, equalNonzero, singleRoot, doubleRoot)] += 1.0;
            }
            auto it = localCache.find(key);
            if (it == localCache.end()) {
                it = localCache
                         .emplace(
                             key,
                             localParentWeightDistribution(equalNonzero, singleRoot, doubleRoot, prime))
                         .first;
            }

            const auto systematicWeight = systematic ? parentSupport : 0;
            const auto& local = it->second;
            for (std::size_t h = 0; h < local.size(); ++h) {
                const auto totalWeight = systematicWeight + static_cast<int>(h);
                if (totalWeight >= 0 && totalWeight < static_cast<int>(spectrum.size())) {
                    spectrum[static_cast<std::size_t>(totalWeight)] += local[h];
                }
            }
        }
    }
}

int runSampleRfcOneStep(const Options& opts) {
    if (opts.depth <= 0) {
        throw std::runtime_error("--sample-rfc-one-step requires --depth >= 1");
    }
    if (opts.totalExpansion <= 1) {
        throw std::runtime_error("--total-expansion must be greater than 1");
    }
    if (opts.prime <= 2) {
        throw std::runtime_error("--prime must be an odd prime for this sampler");
    }
    if (opts.samples <= 0) {
        throw std::runtime_error("--samples must be positive");
    }

    const auto parentDepth = opts.depth;
    const auto childDepth = parentDepth - 1;
    const auto childK = std::uint64_t{1} << childDepth;
    const auto parentK = std::uint64_t{1} << parentDepth;
    const auto oldChildN = static_cast<std::uint64_t>(opts.totalExpansion) * childK;
    const auto sysChildN = static_cast<std::uint64_t>(opts.totalExpansion - 1) * childK;
    const auto totalN = static_cast<std::uint64_t>(opts.totalExpansion) * parentK;

    std::vector<double> oldSpectrum(static_cast<std::size_t>(totalN + 1), 0.0);
    std::vector<double> sysSpectrum(static_cast<std::size_t>(totalN + 1), 0.0);
    std::unordered_map<std::uint64_t, double> oldCategoryCounts;
    std::unordered_map<std::uint64_t, double> sysCategoryCounts;
    std::mt19937_64 rng(opts.seed);

    for (int sample = 0; sample < opts.samples; ++sample) {
        const auto oldChild = rfcGeneratorPrime(childDepth, opts.totalExpansion, opts.prime, rng);
        const auto sysChild = rfcGeneratorPrime(childDepth, opts.totalExpansion - 1, opts.prime, rng);
        const auto oldMessages = enumerateEncodedMessages(oldChild, childK, oldChildN, opts.prime);
        const auto sysMessages = enumerateEncodedMessages(sysChild, childK, sysChildN, opts.prime);
        std::unordered_map<std::uint64_t, std::vector<double>> oldCache;
        std::unordered_map<std::uint64_t, std::vector<double>> sysCache;
        accumulateOneStepPairSpectrum(
            oldMessages,
            oldChildN,
            opts.prime,
            false,
            oldSpectrum,
            oldCache,
            &oldCategoryCounts);
        accumulateOneStepPairSpectrum(
            sysMessages,
            sysChildN,
            opts.prime,
            true,
            sysSpectrum,
            sysCache,
            &sysCategoryCounts);
        if (opts.samples >= 10 && (sample + 1) % std::max(1, opts.samples / 10) == 0) {
            std::cerr << "sample=" << (sample + 1) << "/" << opts.samples << '\n';
        }
    }

    for (auto& value : oldSpectrum) {
        value /= static_cast<double>(opts.samples);
    }
    for (auto& value : sysSpectrum) {
        value /= static_cast<double>(opts.samples);
    }
    for (auto& item : oldCategoryCounts) {
        item.second /= static_cast<double>(opts.samples);
    }
    for (auto& item : sysCategoryCounts) {
        item.second /= static_cast<double>(opts.samples);
    }

    std::cout << "p=" << opts.prime << " parent_depth=" << parentDepth << " parent_k=" << parentK
              << " total_n=" << totalN << " samples=" << opts.samples << " total_expansion="
              << opts.totalExpansion << '\n';
    std::cout << "old_one_step_first_moment_crossing=" << firstCumulativeAtLeastOne(oldSpectrum) << '\n';
    std::cout << "systematic_one_step_first_moment_crossing=" << firstCumulativeAtLeastOne(sysSpectrum) << '\n';
    std::cout << "weight,old_expected_count,systematic_expected_count\n";
    for (std::size_t h = 0; h < oldSpectrum.size(); ++h) {
        if (oldSpectrum[h] != 0.0 || sysSpectrum[h] != 0.0) {
            std::cout << h << ',' << std::setprecision(12) << oldSpectrum[h] << ',' << sysSpectrum[h]
                      << '\n';
        }
    }

    if (!opts.spectrumPath.empty()) {
        std::ofstream out(opts.spectrumPath);
        if (!out) {
            throw std::runtime_error("failed to open spectrum output path");
        }
        out << "weight,old_expected_count,systematic_expected_count\n";
        out << std::setprecision(17);
        for (std::size_t h = 0; h < oldSpectrum.size(); ++h) {
            out << h << ',' << oldSpectrum[h] << ',' << sysSpectrum[h] << '\n';
        }
    }
    if (!opts.categoryPath.empty()) {
        std::ofstream out(opts.categoryPath);
        if (!out) {
            throw std::runtime_error("failed to open category output path");
        }
        out << "ensemble,parent_support,equal_nonzero,single_root,double_root,expected_pair_count,"
               "min_parity_weight,max_parity_weight,min_total_weight,max_total_weight\n";
        out << std::setprecision(17);
        auto writeCategoryRows = [&](const char* ensemble, const auto& counts) {
            std::vector<std::pair<std::uint64_t, double>> rows(counts.begin(), counts.end());
            std::sort(rows.begin(), rows.end(), [](const auto& a, const auto& b) {
                return a.second > b.second;
            });
            for (const auto& row : rows) {
                const auto support = static_cast<int>(row.first & 0xffff);
                const auto equalNonzero = static_cast<int>((row.first >> 16) & 0xffff);
                const auto singleRoot = static_cast<int>((row.first >> 32) & 0xffff);
                const auto doubleRoot = static_cast<int>((row.first >> 48) & 0xffff);
                const auto maxParityWeight = 2 * (equalNonzero + singleRoot + doubleRoot);
                const auto minParityWeight = maxParityWeight - singleRoot - doubleRoot;
                const auto minTotalWeight =
                    std::string_view(ensemble) == "systematic" ? minParityWeight + support : minParityWeight;
                const auto maxTotalWeight =
                    std::string_view(ensemble) == "systematic" ? maxParityWeight + support : maxParityWeight;
                out << ensemble << ',' << support << ',' << equalNonzero << ',' << singleRoot << ','
                    << doubleRoot << ',' << row.second << ',' << minParityWeight << ',' << maxParityWeight
                    << ',' << minTotalWeight << ',' << maxTotalWeight << '\n';
            }
        };
        writeCategoryRows("original", oldCategoryCounts);
        writeCategoryRows("systematic", sysCategoryCounts);
    }
    return 0;
}

std::vector<std::string> splitCsvLine(const std::string& line) {
    std::vector<std::string> out;
    std::string cell;
    std::stringstream ss(line);
    while (std::getline(ss, cell, ',')) {
        out.push_back(cell);
    }
    return out;
}

void writeCertificate(const std::string& path, const std::vector<CertRow>& rows) {
    std::ofstream out(path);
    if (!out) {
        throw std::runtime_error("failed to open certificate path");
    }
    out << "round,k,parity_n,support,threshold,distance,relative_distance,worst_log2_slack,worst_support\n";
    out << std::setprecision(17);
    for (const auto& row : rows) {
        out << row.round << ',' << row.k << ',' << row.parityN << ',' << row.support << ',' << row.threshold << ','
            << row.distance << ',' << row.relativeDistance << ',' << row.worstLog2Slack << ',' << row.worstSupport << '\n';
    }
}

void writeThresholds(const std::string& path, const std::vector<RoundState>& rounds) {
    std::ofstream out(path);
    if (!out) {
        throw std::runtime_error("failed to open thresholds path");
    }
    out << "round,k,parity_n,support,threshold,distance_at_support,relative_distance_at_support\n";
    out << std::setprecision(17);
    for (std::size_t round = 0; round < rounds.size(); ++round) {
        const auto& state = rounds[round];
        for (std::uint64_t support = 1; support <= state.k; ++support) {
            const auto threshold = state.thresholds[static_cast<std::size_t>(support)];
            const auto distance = static_cast<int>(support + state.parityN - static_cast<std::uint64_t>(threshold - 1));
            const auto relative = static_cast<double>(distance) / static_cast<double>(state.k + state.parityN);
            out << round << ',' << state.k << ',' << state.parityN << ',' << support << ',' << threshold << ',' << distance
                << ',' << relative << '\n';
        }
    }
}

std::map<int, RoundState> readThresholds(const std::string& path) {
    std::ifstream in(path);
    if (!in) {
        throw std::runtime_error("failed to open thresholds CSV");
    }

    std::string line;
    if (!std::getline(in, line)) {
        throw std::runtime_error("empty thresholds CSV");
    }

    std::map<int, std::vector<std::tuple<std::uint64_t, int, int, std::uint64_t>>> rowsByRound;
    while (std::getline(in, line)) {
        if (line.empty()) {
            continue;
        }
        const auto cells = splitCsvLine(line);
        if (cells.size() < 5) {
            throw std::runtime_error("malformed thresholds CSV row");
        }
        const auto round = std::stoi(cells[0]);
        const auto k = static_cast<std::uint64_t>(std::stoull(cells[1]));
        const auto parityN = static_cast<std::uint64_t>(std::stoull(cells[2]));
        const auto support = std::stoi(cells[3]);
        const auto threshold = std::stoi(cells[4]);
        rowsByRound[round].emplace_back(k, support, threshold, parityN);
    }

    std::map<int, RoundState> rounds;
    for (auto& item : rowsByRound) {
        auto& rows = item.second;
        std::sort(rows.begin(), rows.end(), [](const auto& a, const auto& b) {
            return std::get<1>(a) < std::get<1>(b);
        });
        const auto k = static_cast<std::uint64_t>(rows.size());
        const auto declaredK = std::get<0>(rows[0]);
        const auto parityN = std::get<3>(rows[0]);
        if (declaredK != k) {
            throw std::runtime_error("declared k does not match support count");
        }
        RoundState state;
        state.k = k;
        state.parityN = parityN;
        state.thresholds.assign(static_cast<std::size_t>(k + 1), static_cast<int>(parityN + 1));
        for (std::uint64_t i = 0; i < rows.size(); ++i) {
            const auto support = std::get<1>(rows[static_cast<std::size_t>(i)]);
            if (support != static_cast<int>(i + 1)) {
                throw std::runtime_error("supports are not contiguous");
            }
            if (std::get<0>(rows[static_cast<std::size_t>(i)]) != declaredK ||
                std::get<3>(rows[static_cast<std::size_t>(i)]) != parityN) {
                throw std::runtime_error("inconsistent k or parity_n in threshold round");
            }
            state.thresholds[static_cast<std::size_t>(support)] = std::get<2>(rows[static_cast<std::size_t>(i)]);
        }
        rounds[item.first] = std::move(state);
    }
    return rounds;
}

void checkMonotone(int round, const std::vector<int>& thresholds, std::vector<std::string>& errors) {
    for (std::size_t i = 2; i < thresholds.size(); ++i) {
        if (thresholds[i] < thresholds[i - 1]) {
            std::ostringstream ss;
            ss << "round " << round << ": threshold decreases at support " << i;
            errors.push_back(ss.str());
        }
    }
}

void checkAdmissible(int round, std::uint64_t n, const std::vector<int>& thresholds, std::vector<std::string>& errors) {
    for (std::size_t i = 1; i < thresholds.size(); ++i) {
        if (thresholds[i] < 1 || thresholds[i] > static_cast<int>(n)) {
            std::ostringstream ss;
            ss << "round " << round << " support " << i << ": threshold outside 1.." << n;
            errors.push_back(ss.str());
        }
    }
}

int runVerify(const Options& opts) {
    if (opts.fieldBits < 10.0) {
        throw std::runtime_error("the verifier requires field_bits >= 10");
    }
    const auto rounds = readThresholds(opts.verifyPath);
    if (rounds.empty()) {
        throw std::runtime_error("no threshold rounds found");
    }

    std::vector<std::string> errors;
    double worstSlack = NegInf;
    int worstRound = 0;
    int worstSupport = 0;

    const auto firstRound = rounds.begin()->first;
    if (firstRound != 0) {
        errors.push_back("rounds do not start at 0");
    }
    const auto firstK = rounds.begin()->second.k;
    const auto firstN = rounds.begin()->second.parityN;
    const auto parityExpansion = firstN / firstK;
    if (firstN % firstK != 0) {
        errors.push_back("round 0 parity_n is not divisible by k");
    }

    int expectedRound = 0;
    for (const auto& item : rounds) {
        const auto round = item.first;
        const auto& state = item.second;
        if (round != expectedRound++) {
            errors.push_back("rounds are not contiguous");
        }
        if (parityExpansion != 0 && state.parityN != parityExpansion * state.k) {
            errors.push_back("parity expansion is not constant");
        }
        checkMonotone(round, state.thresholds, errors);
        checkAdmissible(round, state.parityN, state.thresholds, errors);
        if (round == 0) {
            continue;
        }
        const auto prevIt = rounds.find(round - 1);
        if (prevIt == rounds.end()) {
            errors.push_back("missing previous round");
            continue;
        }
        const auto& prev = prevIt->second;
        if (state.k != 2 * prev.k || state.parityN != 2 * prev.parityN) {
            errors.push_back("round does not double k and parity_n");
            continue;
        }

        const auto perSupportBudget = -opts.securityBits - std::log2(static_cast<double>(state.k));
        const auto logCombK = log2Combinations(prev.k);
        const auto logCombN = log2Combinations(prev.parityN);
        const auto logFactN = log2Factorials(prev.parityN);
        const auto splitTerms =
            buildSplitTerms(prev.k, prev.parityN, prev.thresholds, opts.fieldBits, logCombK, logCombN);

        for (std::uint64_t support = 1; support <= state.k; ++support) {
            const auto failure = log2FailureFromSplitTerms(
                prev.parityN,
                opts.fieldBits,
                splitTerms,
                support,
                state.thresholds[static_cast<std::size_t>(support)],
                logFactN);
            const auto slack = failure - perSupportBudget;
            if (slack > worstSlack) {
                worstSlack = slack;
                worstRound = round;
                worstSupport = static_cast<int>(support);
            }
            if (slack > 0.0) {
                std::ostringstream ss;
                ss << "round " << round << " support " << support << ": log2 failure exceeds budget by " << slack;
                errors.push_back(ss.str());
            }
        }
    }

    const auto& finalState = rounds.rbegin()->second;
    const auto distance = systematicDistance(finalState.k, finalState.parityN, finalState.thresholds);

    std::cout << std::setprecision(12)
              << "ok=" << (errors.empty() ? "true" : "false") << '\n'
              << "rounds=" << rounds.size() << '\n'
              << "final_round=" << rounds.rbegin()->first << '\n'
              << "final_k=" << finalState.k << '\n'
              << "final_parity_n=" << finalState.parityN << '\n'
              << "parity_expansion=" << parityExpansion << '\n'
              << "final_distance=" << distance.distance << '\n'
              << "final_relative_distance=" << static_cast<double>(distance.distance) / static_cast<double>(finalState.k + finalState.parityN)
              << '\n'
              << "final_worst_distance_support=" << distance.support << '\n'
              << "worst_log2_slack=" << worstSlack << '\n'
              << "worst_slack_round=" << worstRound << '\n'
              << "worst_slack_support=" << worstSupport << '\n';

    if (!errors.empty()) {
        std::cout << "errors:\n";
        for (const auto& error : errors) {
            std::cout << "- " << error << '\n';
        }
        return 1;
    }
    return 0;
}

int runGenerate(const Options& opts) {
    if (opts.k0 != 1) {
        throw std::runtime_error("this tool currently assumes k0=1");
    }
    if (opts.depth < 0 || opts.parityExpansion <= 0) {
        throw std::runtime_error("--depth and --parity-expansion are required for generation");
    }

    std::uint64_t k = opts.k0;
    std::uint64_t n = static_cast<std::uint64_t>(opts.parityExpansion) * k;
    std::vector<int> thresholds(static_cast<std::size_t>(k + 1), static_cast<int>(n + 1));
    thresholds[0] = static_cast<int>(n + 1);
    thresholds[1] = 1;

    std::vector<CertRow> certRows;
    std::vector<RoundState> roundStates;
    double pendingSlack = 0.0;
    int pendingWorstSupport = 1;

    if (opts.compare) {
        std::cout << "round,k,parity_n,total_n,systematic_distance,best_support,"
                  << "systematic_relative,non_systematic_relative,dense_heuristic_relative\n";
    } else {
        std::cout << "round,k,n,min_distance,best_support,relative_distance\n";
    }

    for (int round = 0; round <= opts.depth; ++round) {
        const auto distance = systematicDistance(k, n, thresholds);
        const auto relative = static_cast<double>(distance.distance) / static_cast<double>(k + n);

        certRows.push_back(CertRow{
            round,
            k,
            n,
            distance.support,
            thresholds[static_cast<std::size_t>(distance.support)],
            distance.distance,
            relative,
            pendingSlack,
            pendingWorstSupport,
        });
        roundStates.push_back(RoundState{k, n, thresholds});

        if (opts.compare) {
            const auto totalExpansion = opts.parityExpansion + 1;
            const auto oldDelta = basefoldGlobalDistance(round, totalExpansion, opts.fieldBits, opts.securityBits, opts.k0);
            const auto parityDelta = basefoldGlobalDistance(round, opts.parityExpansion, opts.fieldBits, opts.securityBits, opts.k0);
            const auto denseDelta = (1.0 + static_cast<double>(opts.parityExpansion) * parityDelta) /
                                    static_cast<double>(totalExpansion);
            std::cout << round << ',' << k << ',' << n << ',' << (k + n) << ',' << distance.distance << ','
                      << distance.support << ',' << std::fixed << std::setprecision(8) << relative << ',' << oldDelta
                      << ',' << denseDelta << '\n';
        } else {
            std::cout << round << ',' << k << ',' << n << ',' << distance.distance << ',' << distance.support << ','
                      << std::fixed << std::setprecision(8) << relative << '\n';
        }

        if (round == opts.depth) {
            break;
        }

        const auto prevThresholds = opts.monotoneThresholds ? prefixMaxThresholds(thresholds) : thresholds;
        auto next = nextThresholds(
            k,
            opts.parityExpansion,
            thresholds,
            opts.fieldBits,
            opts.securityBits,
            opts.monotoneThresholds);
        if (opts.monotoneThresholds) {
            next = prefixMaxThresholds(next);
        }
        const auto report = theoremConditionReport(k, n, prevThresholds, next, opts.fieldBits, opts.securityBits);
        pendingSlack = report.first;
        pendingWorstSupport = report.second;
        thresholds = std::move(next);
        k *= 2;
        n *= 2;
    }

    if (!certRows.empty()) {
        certRows[0].worstLog2Slack = 0.0;
        certRows[0].worstSupport = certRows[0].support;
    }

    if (opts.dumpFinalProfile) {
        const auto& finalState = roundStates.back();
        std::cout << "support,zero_threshold,systematic_distance,relative_distance\n";
        for (std::uint64_t support = 1; support <= finalState.k; ++support) {
            const auto distance =
                static_cast<int>(support + finalState.parityN - static_cast<std::uint64_t>(finalState.thresholds[static_cast<std::size_t>(support)] - 1));
            std::cout << support << ',' << finalState.thresholds[static_cast<std::size_t>(support)] << ',' << distance
                      << ',' << std::fixed << std::setprecision(8)
                      << static_cast<double>(distance) / static_cast<double>(finalState.k + finalState.parityN) << '\n';
        }
    }

    if (!opts.certificatePath.empty()) {
        writeCertificate(opts.certificatePath, certRows);
    }
    if (!opts.fullThresholdsPath.empty()) {
        writeThresholds(opts.fullThresholdsPath, roundStates);
    }
    return 0;
}

std::string requireValue(int& i, int argc, char** argv, std::string_view flag) {
    if (i + 1 >= argc) {
        throw std::runtime_error(std::string(flag) + " requires a value");
    }
    return argv[++i];
}

Options parseOptions(int argc, char** argv) {
    Options opts;
    for (int i = 1; i < argc; ++i) {
        const std::string arg = argv[i];
        if (arg == "--k0") {
            opts.k0 = static_cast<std::uint64_t>(std::stoull(requireValue(i, argc, argv, arg)));
        } else if (arg == "--depth") {
            opts.depth = std::stoi(requireValue(i, argc, argv, arg));
        } else if (arg == "--parity-expansion") {
            opts.parityExpansion = std::stoi(requireValue(i, argc, argv, arg));
        } else if (arg == "--field-bits") {
            opts.fieldBits = std::stod(requireValue(i, argc, argv, arg));
        } else if (arg == "--security-bits") {
            opts.securityBits = std::stod(requireValue(i, argc, argv, arg));
        } else if (arg == "--compare") {
            opts.compare = true;
        } else if (arg == "--monotone-thresholds") {
            opts.monotoneThresholds = true;
        } else if (arg == "--dump-final-profile") {
            opts.dumpFinalProfile = true;
        } else if (arg == "--certificate-path") {
            opts.certificatePath = requireValue(i, argc, argv, arg);
        } else if (arg == "--full-thresholds-path") {
            opts.fullThresholdsPath = requireValue(i, argc, argv, arg);
        } else if (arg == "--verify") {
            opts.verifyPath = requireValue(i, argc, argv, arg);
        } else if (arg == "--ideal-first-moment") {
            opts.idealFirstMoment = true;
        } else if (arg == "--sample-rfc-first-moment") {
            opts.sampleRfcFirstMoment = true;
        } else if (arg == "--sample-rfc-product-first-moment") {
            opts.sampleRfcProductFirstMoment = true;
        } else if (arg == "--sample-rfc-one-step") {
            opts.sampleRfcOneStep = true;
        } else if (arg == "--total-expansion") {
            opts.totalExpansion = std::stoi(requireValue(i, argc, argv, arg));
        } else if (arg == "--prime") {
            opts.prime = std::stoi(requireValue(i, argc, argv, arg));
        } else if (arg == "--samples") {
            opts.samples = std::stoi(requireValue(i, argc, argv, arg));
        } else if (arg == "--seed") {
            opts.seed = static_cast<std::uint64_t>(std::stoull(requireValue(i, argc, argv, arg)));
        } else if (arg == "--spectrum-path") {
            opts.spectrumPath = requireValue(i, argc, argv, arg);
        } else if (arg == "--support-spectrum-path") {
            opts.supportSpectrumPath = requireValue(i, argc, argv, arg);
        } else if (arg == "--category-path") {
            opts.categoryPath = requireValue(i, argc, argv, arg);
        } else if (arg == "--help" || arg == "-h") {
            std::cout
                << "systematic_rfc_cert options:\n"
                << "  --depth N --parity-expansion N     generate thresholds\n"
                << "  --verify path                      verify threshold CSV\n"
                << "  --ideal-first-moment               print ideal random-code first-moment baselines\n"
                << "  --sample-rfc-first-moment          sample tiny-field RFC first-moment spectra\n"
                << "  --sample-rfc-product-first-moment  sample one-tree laws and product them to expansion\n"
                << "  --sample-rfc-one-step              sample child-pair one-step RFC first-moment spectra\n"
                << "  --total-expansion N                total expansion for first-moment baselines\n"
                << "  --prime P --samples N --seed N     sampler parameters\n"
                << "  --spectrum-path path               write sampled spectrum CSV\n"
                << "  --support-spectrum-path path       write sampled support-stratified spectrum CSV\n"
                << "  --category-path path               write sampled one-step category CSV\n"
                << "  --field-bits B                     field size log2, default 128\n"
                << "  --security-bits B                  per-transition security bits\n"
                << "  --compare                          print non-systematic comparison columns\n"
                << "  --monotone-thresholds              enforce prefix-max thresholds\n"
                << "  --certificate-path path            write summary CSV\n"
                << "  --full-thresholds-path path        write threshold CSV\n";
            std::exit(0);
        } else {
            throw std::runtime_error("unknown argument: " + arg);
        }
    }
    return opts;
}

} // namespace

int main(int argc, char** argv) {
    try {
        const auto opts = parseOptions(argc, argv);
        if (!opts.verifyPath.empty()) {
            return runVerify(opts);
        }
        if (opts.idealFirstMoment) {
            return runIdealFirstMoment(opts);
        }
        if (opts.sampleRfcFirstMoment) {
            return runSampleRfcFirstMoment(opts);
        }
        if (opts.sampleRfcProductFirstMoment) {
            return runSampleRfcProductFirstMoment(opts);
        }
        if (opts.sampleRfcOneStep) {
            return runSampleRfcOneStep(opts);
        }
        return runGenerate(opts);
    } catch (const std::exception& e) {
        std::cerr << "error: " << e.what() << '\n';
        return 1;
    }
}
