# forever-perfmon

Rust rewrite of the original 

[performancer]: https://github.com/ForeverThawn/Performancer

 PowerShell monitor.

## Config

The app reads `forever-perfmon.toml` from the repository root:

```toml
csv_dir = "X:\\_TEMP\\performancer_log"
csv_output = true
snapshot_dir = "X:\\_TEMP\\performancer_log"
hyperv_vm_name = "ubuntu_22_04"
```

It writes `snapshot.json` into the configured snapshot directory. When `csv_output` is `true`, it also writes a timestamped CSV file into `csv_dir`. If a snapshot already exists, press `C` to continue cumulative counters or `R` to reset them. No Enter key is required.

While running:

- `Q` exits.
- `C` clears the console.

The Hyper-V counters keep the original VM name from the script: `ubuntu_22_04`. If those counters do not exist, the Hyper-V line is shown as unavailable.
