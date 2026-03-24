#!/usr/bin/env python3
"""
Visualize woof parallel speedup
Shows near-linear scaling up to available CPUs
"""

import sys
import subprocess
import json
import time

def run_benchmark(project_dir, threads):
    """Run woof with specified thread count and return time in ms"""
    cmd = [
        "./target/release/woofmt",
        "check", project_dir,
        "--threads", str(threads),
        "--quiet"
    ]
    
    times = []
    for _ in range(3):  # 3 iterations
        start = time.perf_counter()
        result = subprocess.run(cmd, capture_output=True)
        elapsed = (time.perf_counter() - start) * 1000
        if result.returncode in [0, 1]:  # 0 = no issues, 1 = issues found
            times.append(elapsed)
    
    return sum(times) / len(times) if times else None

def main():
    project_dir = sys.argv[1] if len(sys.argv) > 1 else "/tmp/woof_large_project"
    
    import os
    cpus = os.cpu_count() or 4
    
    print("=" * 60)
    print("Woof Parallel Speedup Visualization")
    print("=" * 60)
    print(f"Project: {project_dir}")
    print(f"Available CPUs: {cpus}")
    print()
    
    # Get baseline (1 thread)
    print("Running benchmarks...")
    print("  1 thread (baseline)...", end=" ", flush=True)
    baseline = run_benchmark(project_dir, 1)
    print(f"{baseline:.0f} ms")
    
    results = [(1, baseline)]
    
    # Test different thread counts
    for t in [2, 4, 8, 12, 16]:
        if t <= cpus:
            print(f"  {t} threads...", end=" ", flush=True)
            elapsed = run_benchmark(project_dir, t)
            if elapsed:
                results.append((t, elapsed))
                print(f"{elapsed:.0f} ms")
    
    print()
    print("=" * 60)
    print("Results")
    print("=" * 60)
    print()
    
    # Header
    print(f"{'CPUs':>6} | {'Time (ms)':>10} | {'Speedup':>8} | {'Efficiency':>10} | {'Bar'}")
    print("-" * 60)
    
    baseline_time = results[0][1]
    max_speedup = max(baseline_time / t for _, t in results)
    
    for cpus, elapsed in results:
        speedup = baseline_time / elapsed
        efficiency = (speedup / cpus) * 100
        
        # Bar chart
        bar_len = int(speedup * 8)
        bar = "█" * bar_len
        
        print(f"{cpus:>6} | {elapsed:>10.0f} | {speedup:>7.2f}x | {efficiency:>9.1f}% | {bar}")
    
    print()
    print("-" * 60)
    print("Ideal linear scaling:")
    
    for cpus, _ in results:
        bar_len = cpus * 8
        bar = "░" * min(bar_len, 56)
        print(f"{cpus:>6} | {'':>10} | {cpus:>7.2f}x | {'100.0%':>10} | {bar}")
    
    print()
    print("=" * 60)
    print("Legend:")
    print("  █ = Actual speedup")
    print("  ░ = Ideal linear speedup")
    print()
    
    # Summary
    max_speedup = max(baseline_time / t for _, t in results)
    best_config = max(results, key=lambda x: baseline_time / x[1])
    
    print(f"Best configuration: {best_config[0]} threads")
    print(f"Max speedup: {max_speedup:.2f}x")
    print()
    
    if max_speedup >= best_config[0] * 0.8:
        print("✓ Excellent parallel efficiency!")
    elif max_speedup >= best_config[0] * 0.6:
        print("✓ Good parallel efficiency")
    else:
        print("⚠ Parallel overhead significant")
        print("  Consider larger batch sizes or fewer threads")

if __name__ == "__main__":
    main()
