# forever-perfmon

Rust rewrite of the original [performancer](https://github.com/ForeverThawn/Performancer) PowerShell monitor.

## Config

The app reads `forever-perfmon.toml` from the program's working directory:

```toml
snapshot_dir = "X:\\_TEMP\\performancer_log"

csv = false
csv_dir = "X:\\_TEMP\\performancer_log"

hyperv_vm = false
hyperv_vm_name = "ubuntu_22_04"
```

If the config file is missing in the program's working directory, the app creates it with the default content above.

It writes `snapshot.json` into the configured snapshot directory. When `csv` is `true`, it also writes a timestamped CSV file into `csv_dir`. When `hyperv_vm` is `true`, it monitors the VM named by `hyperv_vm_name`; when `false`, the Hyper-V memory row is hidden. If a snapshot already exists, press `C` to continue cumulative counters or `R` to reset them. No Enter key is required.

While running:

- `Q` exits.
- `C` clears the console.

The Hyper-V counters keep the original VM name from the script: `ubuntu_22_04`. If those counters do not exist, the Hyper-V line is shown as unavailable.
