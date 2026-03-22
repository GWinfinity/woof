#!/usr/bin/env python3
"""
Visualize woof scaling up to 256 cores
Shows the gap between linear and actual scaling
"""

import math
import sys

def amdahl_speedup(num_cores, serial_fraction=0.02):
    """Amdahl's Law: S = 1 / (s + (1-s)/N)"""
    return 1.0 / (serial_fraction + (1.0 - serial_fraction) / num_cores)

def realistic_speedup(num_cores, file_count, complexity=1.0):
    """More realistic model with contention and imbalance"""
    # Amdahl base
    s = amdahl_speedup(num_cores)
    
    # Contention penalty (increases with cores)
    contention = 0.95 ** math.log2(max(num_cores, 1))
    
    # Load imbalance (decreases with more files)
    files_per_core = file_count / max(num_cores, 1)
    imbalance = max(0, 0.15 - 0.05 * math.log10(max(files_per_core, 1)))
    
    return s * contention * (1.0 - imbalance)

def woof_optimized_speedup(num_cores, file_count):
    """woof with hierarchical scheduling optimizations"""
    # Reduced serial fraction due to lock-free structures
    serial = 0.005 if num_cores <= 64 else 0.008  # slightly higher at 256
    
    # Better contention handling with hierarchical schedulers
    if num_cores <= 64:
        contention = 0.98 ** math.log2(max(num_cores, 1))
    else:
        # Hierarchical schedulers reduce contention for 256 cores
        contention = 0.98 ** (math.log2(max(num_cores, 1)) * 0.7)
    
    # Load imbalance
    files_per_core = file_count / max(num_cores, 1)
    imbalance = max(0, 0.1 - 0.03 * math.log10(max(files_per_core, 1)))
    
    s = 1.0 / (serial + (1.0 - serial) / num_cores)
    return s * contention * (1.0 - imbalance)

def draw_chart():
    cores = [1, 2, 4, 8, 16, 32, 64, 128, 256]
    file_count = 100000  # 100K files
    
    print("=" * 90)
    print("woof Scaling Visualization: 1 to 256 Cores (100K files)")
    print("=" * 90)
    print()
    
    # Header
    print(f"{'Cores':>6} | {'Linear':>8} | {'Amdahl':>8} | {'Realistic':>9} | {'woof opt':>9} | {'Chart'}")
    print("-" * 90)
    
    max_speedup = 256  # Linear max
    
    for c in cores:
        linear = c
        amdahl = amdahl_speedup(c)
        realistic = realistic_speedup(c, file_count)
        woof = woof_optimized_speedup(c, file_count)
        
        # Create bar chart
        bar_len = 40
        linear_bar = int((linear / max_speedup) * bar_len)
        amdahl_bar = int((amdahl / max_speedup) * bar_len)
        realistic_bar = int((realistic / max_speedup) * bar_len)
        woof_bar = int((woof / max_speedup) * bar_len)
        
        print(f"{c:>6} | {linear:>7.1f}x | {amdahl:>7.1f}x | {realistic:>8.1f}x | {woof:>8.1f}x |", end="")
        
        # Draw bars
        chart = ""
        for i in range(bar_len):
            if i < woof_bar:
                chart += "█"  # woof optimized
            elif i < realistic_bar:
                chart += "▓"  # Realistic
            elif i < amdahl_bar:
                chart += "▒"  # Amdahl
            elif i < linear_bar:
                chart += "░"  # Linear
            else:
                chart += " "
        print(f" {chart}")
    
    print()
    print("Legend:")
    print("  █ = woof (optimized with hierarchical scheduling)")
    print("  ▓ = Realistic (with contention and imbalance)")
    print("  ▒ = Amdahl's Law (theoretical with serial fraction)")
    print("  ░ = Perfect linear scaling")
    print()
    
    # Summary
    woof_256 = woof_optimized_speedup(256, file_count)
    linear_256 = 256
    efficiency = (woof_256 / linear_256) * 100
    
    print(f"256-Core Summary:")
    print(f"  Linear scaling:     256.0x")
    print(f"  woof (predicted):   {woof_256:.1f}x")
    print(f"  Efficiency:         {efficiency:.1f}%")
    print()
    
    # Efficiency by core count
    print("Efficiency Analysis:")
    print(f"{'Cores':>8} {'woof':>10} {'vs Linear':>12}")
    print("-" * 35)
    for c in [4, 8, 16, 32, 64, 128, 256]:
        woof_s = woof_optimized_speedup(c, file_count)
        eff = (woof_s / c) * 100
        gap = c - woof_s
        print(f"{c:>8} {eff:>9.1f}% {gap:>10.1f}x")
    
    print()
    print("Key Insights:")
    print("  • Up to 16 cores: Near-linear scaling (70-90% efficiency)")
    print("  • 16-64 cores: Good scaling (50-70% efficiency)")
    print("  • 64-256 cores: Diminishing returns (30-50% efficiency)")
    print("  • Hierarchical scheduling extends efficient scaling to 256 cores")
    print()

def print_recommendations():
    print("=" * 90)
    print("Recommendations for 256-Core Deployment")
    print("=" * 90)
    print()
    
    scenarios = [
        ("CI/CD (dedicated)", 256, 500000, "2.6m", "1.7m"),
        ("CI/CD (shared)", 128, 500000, "1.7m", "1.1m"),
        ("Monorepo analysis", 256, 1000000, "5.2m", "3.5m"),
        ("Nightly lint", 256, 2000000, "10.4m", "6.8m"),
    ]
    
    print(f"{'Scenario':<20} {'Cores':>8} {'Files':>10} {'64-core':>10} {'256-core':>10} {'Speedup':>10}")
    print("-" * 90)
    
    for name, cores, files, t64, t256 in scenarios:
        # Parse times
        def to_seconds(t):
            if 'm' in t:
                return float(t.replace('m', '')) * 60
            elif 's' in t:
                return float(t.replace('s', ''))
            return float(t)
        
        s64 = to_seconds(t64)
        s256 = to_seconds(t256)
        speedup = s64 / s256
        
        print(f"{name:<20} {cores:>8} {files:>10} {t64:>10} {t256:>10} {speedup:>9.2f}x")
    
    print()
    print("Best Practices for 256 Cores:")
    print()
    print("  1. File Count")
    print("     Minimum: 10,000 files (to amortize scheduling overhead)")
    print("     Optimal: 100,000+ files (best efficiency)")
    print()
    print("  2. Batch Size")
    print("     Auto-tuned: 200-500 files per batch")
    print("     Larger batches reduce scheduling overhead")
    print()
    print("  3. NUMA Awareness")
    print("     Pin schedulers to NUMA nodes")
    print("     Use numactl --interleave=all for memory allocation")
    print()
    print("  4. Monitoring")
    print("     Watch for load imbalance (some schedulers finishing early)")
    print("     Adjust batch size if imbalance > 20%")
    print()

if __name__ == "__main__":
    draw_chart()
    print_recommendations()
