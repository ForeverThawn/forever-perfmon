# Export-ModuleMember -Function Convert-Timestamp
# Export-ModuleMember -Function Format-BytesToString
Function Convert-Timestamp {
param (
    $timestamp,
    [switch]$Days,
    [switch]$Hours,
    [switch]$Minutes,
    [switch]$Seconds
)
    $totalSeconds = [int64]$timestamp
    $bDays = [math]::Floor($totalSeconds / 86400)
    $remainingSeconds = $totalSeconds % 86400
    $bHours = [math]::Floor($remainingSeconds / 3600)
    $remainingSeconds %= 3600
    $bMinutes = [math]::Floor($remainingSeconds / 60)
    $bSeconds = $remainingSeconds % 60
        
    if ($Days -eq $True) {
        return $bDays
    } elseif ($Hours -eq $True) {
        return $bHours
    } elseif ($Minutes -eq $True) {
        return $bMinutes
    } elseif ($Seconds -eq $True) {
        return $bSeconds
    } else {
        return "$bDays" + ':' + "$bHours" + ':' + "$bMinutes" + ':' + "$bSeconds"
    }
}

Function Format-BytesToString {
param(
    $byte_size
) 
    if ($byte_size -lt 1024) {
        return ('{0,15}  B ' -f [double]$byte_size)
    }
    elseif (($byte_size -lt 0x100000) -and ($byte_size -gt 1024)) { 
        $byte_size /= 0x400  # KB
        return ('{0,15:n3} KB ' -f $byte_size)
    }
    elseif (($byte_size -gt 0x100000) -and ($byte_size -lt 0x40000000)) {
        $byte_size /= 0x100000  # MB
        return ('{0,15:n3} MB ' -f $byte_size)
    }
    elseif (($byte_size -gt 0x40000000) -and ($byte_size -lt 0x10000000000)) {
        $byte_size /= 0x40000000  # GB
        return ('{0,15:n3} GB ' -f $byte_size)
    }
    elseif (($byte_size -gt 0x10000000000) -and ($byte_size -lt 0x3FFFFFFFFFFFC)) {
        $byte_size /= 0x10000000000  # TB
        return ('{0,15:n3} TB ' -f $byte_size)
    }
    elseif (($byte_size -gt 0x3FFFFFFFFFFFC) -and ($byte_size -lt 0x1000000000000000)) {
        $byte_size /= 0x3FFFFFFFFFFFC  # PB
        return ('{0,15:n3} PB ' -f $byte_size)
    }
}

Function Format-PercentageColor {
param (
    [Double]$Percentage,
    [Switch]$NoNewline
)
    if ($NoNewline) {
        if ($Percentage -lt 0.0) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor White -NoNewline
        }
        elseif ($Percentage -lt 12.5) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Cyan -NoNewline
        } 
        elseif (($Percentage -ge 12.5) -and ($Percentage -lt 25.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkCyan -NoNewline
        }
        elseif (($Percentage -ge 25.0) -and ($Percentage -lt 37.5)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Green -NoNewline
        }
        elseif (($Percentage -ge 37.5) -and ($Percentage -lt 50.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkGreen -NoNewline
        }
        elseif (($Percentage -ge 50.0) -and ($Percentage -lt 62.5)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Yellow -NoNewline
        }
        elseif (($Percentage -ge 62.5) -and ($Percentage -lt 75.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkYellow -NoNewline
        }
        elseif (($Percentage -ge 75.0) -and ($Percentage -lt 87.5)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Magenta -NoNewline
        }
        elseif (($Percentage -ge 87.5) -and ($Percentage -lt 100.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Red -NoNewline
        }
        elseif ($Percentage -ge 100.0) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkRed -NoNewline
        }
    }
    else {
        if ($Percentage -lt 0.0) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor White
        }
        elseif ($Percentage -lt 12.5) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Cyan
        } 
        elseif (($Percentage -ge 12.5) -and ($Percentage -lt 25.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkCyan
        }
        elseif (($Percentage -ge 25.0) -and ($Percentage -lt 37.5)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Green
        }
        elseif (($Percentage -ge 37.5) -and ($Percentage -lt 50.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkGreen
        }
        elseif (($Percentage -ge 50.0) -and ($Percentage -lt 62.5)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Yellow
        }
        elseif (($Percentage -ge 62.5) -and ($Percentage -lt 75.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkYellow
        }
        elseif (($Percentage -ge 75.0) -and ($Percentage -lt 87.5)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Magenta
        }
        elseif (($Percentage -ge 87.5) -and ($Percentage -lt 100.0)) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor Red
        }
        elseif ($Percentage -ge 100.0) {
            Write-Host ('{0,15:n3}  %' -f $Percentage) -ForegroundColor DarkRed
        }
    }
}

Function Format-PercentageColorSplit7NoNewline {
param (
    [Double]$Percentage
)
    if ($Percentage -lt 0.0) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor White -NoNewline
    }
    elseif ($Percentage -lt 12.5) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor Cyan -NoNewline
    } 
    elseif (($Percentage -ge 12.5) -and ($Percentage -lt 25.0)) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor DarkCyan -NoNewline
    }
    elseif (($Percentage -ge 25.0) -and ($Percentage -lt 37.5)) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor Green -NoNewline
    }
    elseif (($Percentage -ge 37.5) -and ($Percentage -lt 50.0)) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor DarkGreen -NoNewline
    }
    elseif (($Percentage -ge 50.0) -and ($Percentage -lt 62.5)) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor Yellow -NoNewline
    }
    elseif (($Percentage -ge 62.5) -and ($Percentage -lt 75.0)) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor DarkYellow -NoNewline
    }
    elseif (($Percentage -ge 75.0) -and ($Percentage -lt 87.5)) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor Magenta -NoNewline
    }
    elseif (($Percentage -ge 87.5) -and ($Percentage -lt 100.0)) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor Red -NoNewline
    }
    elseif ($Percentage -ge 100.0) {
        Write-Host ('{0,7:n3} %' -f $Percentage) -ForegroundColor DarkRed -NoNewline
    }

}

Function Get-CounterSamplesTotal {
param(
    $CounterSamples
)
    if (-not $CounterSamples) {
        return 0
    }

    $sum = ($CounterSamples | Measure-Object -Property CookedValue -Sum).Sum
    if ($null -eq $sum) {
        return 0
    }

    return [math]::Round($sum, 3)
}

Function Get-NetworkCounterSamples {
param(
    $CounterSamples,
    [string]$CounterCategory,
    [ValidateSet('Received', 'Sent')]
    [string]$Direction
)
    $counterSuffix = if ($Direction -eq 'Received') { '\bytes received/sec' } else { '\bytes sent/sec' }
    $categoryPrefix = "\$($CounterCategory.ToLower())("

    return @($CounterSamples | Where-Object {
        $_.Path.ToLower().Contains($categoryPrefix) -and $_.Path.ToLower().EndsWith($counterSuffix)
    })
}

Function Get-NetworkInterfaceTotals {
param(
    $CounterSamples
)
    $receivedSuffix = '\bytes received/sec'
    $sentSuffix = '\bytes sent/sec'
    $categoryPrefix = '\network interface('
    $receivedTotal = 0.0
    $sentTotal = 0.0

    foreach ($sample in $CounterSamples) {
        $samplePath = $sample.Path.ToLower()
        if (-not $samplePath.Contains($categoryPrefix)) {
            continue
        }

        if ($samplePath.EndsWith($receivedSuffix)) {
            $receivedTotal += $sample.CookedValue
            continue
        }

        if ($samplePath.EndsWith($sentSuffix)) {
            $sentTotal += $sample.CookedValue
        }
    }

    return [pscustomobject]@{
        ReceivedBytesPerSecond = [math]::Round($receivedTotal, 3)
        SentBytesPerSecond     = [math]::Round($sentTotal, 3)
    }
}

Function Get-RoundedCounterValue {
param(
    $CounterSamples,
    [int]$Index
)
    return [math]::Round($CounterSamples[$Index].CookedValue, 3)
}

Function Get-TailscaleAdapterStatistics {
    try {
        $adapterStats = @(Get-NetAdapterStatistics -Name 'Tailscale*' -ErrorAction Stop)
        if ($adapterStats.Count -eq 0) {
            return $null
        }

        $receivedBytes = ($adapterStats | Measure-Object -Property ReceivedBytes -Sum).Sum
        $sentBytes = ($adapterStats | Measure-Object -Property SentBytes -Sum).Sum

        return [pscustomobject]@{
            ReceivedBytes = if ($null -eq $receivedBytes) { 0 } else { [double]$receivedBytes }
            SentBytes     = if ($null -eq $sentBytes) { 0 } else { [double]$sentBytes }
        }
    }
    catch {
        return $null
    }
}

Function Format-ElapsedRecordTime {
param(
    [int64]$Timestamp
)
    $days = [math]::Floor($Timestamp / 86400)
    $remainingSeconds = $Timestamp % 86400
    $hours = [math]::Floor($remainingSeconds / 3600)
    $remainingSeconds %= 3600
    $minutes = [math]::Floor($remainingSeconds / 60)
    $seconds = $remainingSeconds % 60

    return ('{0,4:n0}:{1,3:n0}:{2,3:n0}:{3,3:n0}' -f $days, $hours, $minutes, $seconds)
}

Function Get-ConfiguredDirectory {
param(
    [string]$ConfigFileName
)
    $configPath = Join-Path $PSScriptRoot $ConfigFileName
    $configuredDirectory = (Get-Content $configPath | Select-Object -First 1).Trim()
    if ([string]::IsNullOrWhiteSpace($configuredDirectory)) {
        throw "Config file '$ConfigFileName' is empty"
    }

    if (-not (Test-Path $configuredDirectory)) {
        New-Item -ItemType Directory -Path $configuredDirectory -Force | Out-Null
    }

    return (Resolve-Path $configuredDirectory).Path
}

Function New-TailscaleTotalState {
param(
    $BaseBytes = $null,
    $CarryBytes = 0,
    $LastBytes = $null
)
    return @{
        BaseBytes  = if ($null -eq $BaseBytes) { $null } else { [double]$BaseBytes }
        CarryBytes = if ($null -eq $CarryBytes) { 0 } else { [double]$CarryBytes }
        LastBytes  = if ($null -eq $LastBytes) { $null } else { [double]$LastBytes }
    }
}

Function Get-TailscaleStateTotal {
param(
    $State
)
    if (($null -eq $State) -or ($null -eq $State.BaseBytes) -or ($null -eq $State.LastBytes)) {
        return $null
    }

    return ($State.CarryBytes + [math]::Max(0, ($State.LastBytes - $State.BaseBytes)))
}

Function Update-TailscaleTotalState {
param(
    $State,
    $AbsoluteBytes
)
    if ($null -eq $AbsoluteBytes) {
        return [pscustomobject]@{
            DeltaBytes  = $null
            LatestBytes = $null
        }
    }

    $previousTotal = Get-TailscaleStateTotal -State $State
    $absoluteBytesValue = [double]$AbsoluteBytes

    if ($null -eq $State.BaseBytes) {
        $State.BaseBytes = $absoluteBytesValue
        $State.LastBytes = $absoluteBytesValue
        return [pscustomobject]@{
            DeltaBytes  = 0
            LatestBytes = 0
        }
    }

    if (($null -ne $State.LastBytes) -and ($absoluteBytesValue -lt $State.LastBytes)) {
        $State.CarryBytes += [math]::Max(0, ($State.LastBytes - $State.BaseBytes))
        $State.BaseBytes = $absoluteBytesValue
    }

    $State.LastBytes = $absoluteBytesValue
    $currentTotal = Get-TailscaleStateTotal -State $State
    $deltaBytes = if ($null -eq $previousTotal) { 0 } else { [math]::Max(0, ($currentTotal - $previousTotal)) }

    return [pscustomobject]@{
        DeltaBytes  = $deltaBytes
        LatestBytes = $currentTotal
    }
}

Function Get-PerformancerSnapshot {
    if (-not (Test-Path $Script:snapshot_save_file)) {
        return $null
    }

    try {
        return (Get-Content -Path $Script:snapshot_save_file -Raw -ErrorAction Stop | ConvertFrom-Json)
    }
    catch {
        return $null
    }
}

Function Select-CounterResumeMode {
param(
    $Snapshot
)
    if ($null -eq $Snapshot) {
        return 'Reset'
    }

    Write-Host (" " * ([System.Console]::get_BufferWidth() - 1))
    Write-Host "Snapshot found: $($Snapshot.saved_at)" -ForegroundColor Green
    Write-Host "Press [C] to continue counters or [R] to reset counters." -ForegroundColor Yellow

    while ($true) {
        $selection = [System.Console]::ReadKey($true).Key
        if ($selection -eq 'C') {
            return 'Continue'
        }
        if ($selection -eq 'R') {
            return 'Reset'
        }
    }
}

Function Write-PerformancerSnapshot {
param(
    $Timestamp,
    $DiskReadSum,
    $DiskWriteSum,
    $NetworkReceivedSum,
    $NetworkSentSum,
    $TailscaleReceivedAbsoluteBytes,
    $TailscaleSentAbsoluteBytes
)
    $snapshot = [ordered]@{
        version                          = 5
        saved_at                         = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
        timestamp                        = $Timestamp
        disk_read_sum                    = $DiskReadSum
        disk_write_sum                   = $DiskWriteSum
        network_received_sum             = $NetworkReceivedSum
        network_sent_sum                 = $NetworkSentSum
        tailscale_received_absolute_bytes = $TailscaleReceivedAbsoluteBytes
        tailscale_sent_absolute_bytes     = $TailscaleSentAbsoluteBytes
    }

    try {
        $snapshot | ConvertTo-Json -Depth 3 | Set-Content -Path $Script:snapshot_save_file -Encoding UTF8 -ErrorAction Stop
    }
    catch {
        return
    }
}

$Script:csv_save_file = Get-Location
$Script:snapshot_save_file = $null

Function Show-Performance {
param (
    [switch]$Force
)
    if ($Force) {
        $Host.UI.RawUI.BufferSize = New-Object System.Management.Automation.Host.Size(120, 50)
    }
    $max_length = [System.Console]::get_BufferWidth()
    if ($max_length -lt 110) {
        throw 'Please make sure your console window is at least 110 characters wide'
    }
    $blank_19 = ('{0,19}' -f '')
    $blank_24 = ('{0,24}' -f '')
    for ($i = 0; $i -lt 8; $i++) {
        Write-Host (" " * ($max_length - 1))
    }
    
    $counterList = @(
        '\Processor(_Total)\% Processor Time',   # 0
        '\Processor(_Total)\% User Time'         # 1
        '\Processor(_Total)\% Privileged Time'   # 2
        '\Processor(_Total)\% DPC Time'          # 3
        '\Memory\% Committed Bytes In Use',      # 4
        '\Memory\Available Bytes',               # 5
        '\Process(_total)\Working Set',          # 6 total
        '\Memory\Committed Bytes',               # 7 
        '\Memory\Commit Limit',                  # 8 total
        
        '\Hyper-V Dynamic Memory VM(ubuntu_22_04)\guest available memory' # 9
        '\Hyper-V Dynamic Memory VM(ubuntu_22_04)\physical memory'        # 10

        '\PhysicalDisk(_Total)\Disk Read Bytes/sec', # 11
        '\PhysicalDisk(_Total)\Disk Write Bytes/sec',# 12

        '\Network Interface(*)\Bytes Received/sec',  # 13
        '\Network Interface(*)\Bytes Sent/sec'       # 14



    )
    $timestamp = 0

    $disk_read_sum = 0
    $disk_write_sum = 0
    $network_received_sum = 0
    $network_sent_sum = 0
    Get-PerformancerConfig
    $snapshot = Get-PerformancerSnapshot
    $resumeMode = Select-CounterResumeMode -Snapshot $snapshot
    $tailscale_resume_with_snapshot = ($resumeMode -eq 'Continue') -and ($null -ne $snapshot)
    $tailscale_received_latest_state = New-TailscaleTotalState
    $tailscale_sent_latest_state = New-TailscaleTotalState
    $tailscale_received_delta_state = New-TailscaleTotalState
    $tailscale_sent_delta_state = New-TailscaleTotalState
    $tailscale_received_has_snapshot_baseline = $False
    $tailscale_sent_has_snapshot_baseline = $False

    if ($tailscale_resume_with_snapshot) {
        $timestamp = if ($null -eq $snapshot.timestamp) { 0 } else { [int]$snapshot.timestamp }
        $disk_read_sum = if ($null -eq $snapshot.disk_read_sum) { 0 } else { [double]$snapshot.disk_read_sum }
        $disk_write_sum = if ($null -eq $snapshot.disk_write_sum) { 0 } else { [double]$snapshot.disk_write_sum }
        $network_received_sum = if ($null -eq $snapshot.network_received_sum) { 0 } else { [double]$snapshot.network_received_sum }
        $network_sent_sum = if ($null -eq $snapshot.network_sent_sum) { 0 } else { [double]$snapshot.network_sent_sum }
        $tailscale_received_snapshot_absolute = if ($null -ne $snapshot.tailscale_received_absolute_bytes) { [double]$snapshot.tailscale_received_absolute_bytes } elseif ($null -ne $snapshot.tailscale_received_last_bytes) { [double]$snapshot.tailscale_received_last_bytes } else { $null }
        $tailscale_sent_snapshot_absolute = if ($null -ne $snapshot.tailscale_sent_absolute_bytes) { [double]$snapshot.tailscale_sent_absolute_bytes } elseif ($null -ne $snapshot.tailscale_sent_last_bytes) { [double]$snapshot.tailscale_sent_last_bytes } else { $null }
        $tailscale_received_has_snapshot_baseline = $null -ne $tailscale_received_snapshot_absolute
        $tailscale_sent_has_snapshot_baseline = $null -ne $tailscale_sent_snapshot_absolute
        $tailscale_received_latest_state = New-TailscaleTotalState -BaseBytes $tailscale_received_snapshot_absolute -CarryBytes 0 -LastBytes $tailscale_received_snapshot_absolute
        $tailscale_sent_latest_state = New-TailscaleTotalState -BaseBytes $tailscale_sent_snapshot_absolute -CarryBytes 0 -LastBytes $tailscale_sent_snapshot_absolute
    }

    $memory_total = (Get-CimInstance Win32_PhysicalMemory | Measure-Object -Property Capacity -Sum).Sum

    Clear-Host
    Write-ToCsvHead

    while ($true) {
        #_alloc_space_Get-Performance
        $max_height = [System.Console]::get_WindowHeight()
        $display_line_count = 12
        $cursor_start_height = $max_height - ($display_line_count + 1)
        [System.Console]::SetCursorPosition(0, $cursor_start_height)
        
        $originalCulture = [System.Threading.Thread]::CurrentThread.CurrentCulture
        [System.Threading.Thread]::CurrentThread.CurrentCulture = 'en-US'
        $currentTime = Get-Date -Format "MMM. dd, yyyy   HH:mm:ss"
        [System.Threading.Thread]::CurrentThread.CurrentCulture = $originalCulture
        
        try {
            $Script:counterData = Get-Counter -Counter $counterList -ErrorAction Ignore
        }
        catch {
            continue
        }
        
        $counterSamples = $counterData.CounterSamples
        $cpu_usage = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 0
        $cpu_user_usage = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 1
        $cpu_privileged_usage = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 2
        $cpu_dpc_usage = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 3
        $memory_avail = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 5
        $memory_used = $memory_total - $memory_avail
        $memory_usage_percentage = $memory_used / $memory_total * 100
        $disk_read = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 11
        $disk_write = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 12
        
        # $process_working_set = $counterValues[6]
        
        $memory_committed_percentage = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 4
        $memory_commited = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 7
        $memory_commit_limit = Get-RoundedCounterValue -CounterSamples $counterSamples -Index 8

        $record_time = Format-ElapsedRecordTime -Timestamp $timestamp

        Write-Host "Forever Performance Monitor 5                                                                        " -ForegroundColor Yellow
        Write-Host $currentTime -ForegroundColor Green -NoNewline
        Write-Host "                " -NoNewline
        Write-Host $record_time -ForegroundColor DarkCyan -NoNewline
        Write-Host ('                                                ')
        

        Write-Host "CPU      Usage   : " -ForegroundColor DarkYellow -NoNewline
        # Write-Host ('{0,15:n3}  %                                                                  ' -f $cpu_usage)
        Format-PercentageColor -Percentage $cpu_usage -NoNewline
        Write-Host ('    ') -NoNewline
        Write-Host ('[User]') -ForegroundColor Black -BackgroundColor DarkGray -NoNewline
        Format-PercentageColorSplit7NoNewline -Percentage $cpu_user_usage
        Write-Host ('    ') -NoNewline
        Write-Host ('[Privileged]') -ForegroundColor Black -BackgroundColor DarkGray -NoNewline
        Format-PercentageColorSplit7NoNewline -Percentage $cpu_privileged_usage
        Write-Host ('    ') -NoNewline
        Write-Host ('[Driver]') -ForegroundColor Black -BackgroundColor DarkGray -NoNewline
        Format-PercentageColorSplit7NoNewline -Percentage $cpu_dpc_usage
        Write-Host (' ')
        # Write-Host ('                                                                  ')
    

        $memory_used_formatted = Format-BytesToString($memory_used)
        $memory_total_formatted = Format-BytesToString($memory_total)

        Write-Host "Physical Memory  : " -ForegroundColor DarkYellow -NoNewline
        # Write-Host ('{0,15:n3}  % ' -f $memory_usage_percentage) -NoNewline
        Format-PercentageColor -Percentage $memory_usage_percentage -NoNewline
        Write-Host '  ' -NoNewline
        Write-Host ('{0,21:n} B ' -f $memory_used) -ForegroundColor DarkGray -NoNewline
        Write-Host ($memory_used_formatted) -NoNewline
        Write-Host '/' -ForegroundColor DarkGray -NoNewline
        Write-Host ($memory_total_formatted)


        $memory_commited_formatted = Format-BytesToString($($memory_commited))
        $memory_commit_limit_formatted = Format-BytesToString($($memory_commit_limit))

        Write-Host "Total    Commited: " -ForegroundColor DarkYellow -NoNewline
        # Write-Host ('{0,15:n3}  % ' -f $memory_committed_percentage) -NoNewline
        Format-PercentageColor -Percentage $memory_committed_percentage -NoNewline
        # Write-Host (Format-BytesToString($($process_working_set))) -NoNewline
        Write-Host '  ' -NoNewline
        Write-Host ('{0,21:n} B ' -f $memory_commited) -ForegroundColor DarkGray -NoNewline
        Write-Host ($memory_commited_formatted) -NoNewline
        Write-Host '/' -ForegroundColor DarkGray -NoNewline
        # Write-Host ('{0,21:n} B ' -f $memory_commit_limit) -ForegroundColor DarkGray
        Write-Host ($memory_commit_limit_formatted) 


        $disk_read_sum += $($disk_read)
        $disk_read_formatted = Format-BytesToString($($disk_read))
        $disk_read_sum_formatted = Format-BytesToString($disk_read_sum)

        Write-Host "Disk     Read    : " -ForegroundColor DarkYellow -NoNewline
        Write-Host ($disk_read_formatted) -NoNewline
        Write-Host ('{0,21:n}  B ' -f $($disk_read)) -ForegroundColor DarkGray -NoNewline
        Write-Host ($disk_read_sum_formatted) -ForegroundColor DarkCyan -NoNewline
        Write-Host 'Total                ' -ForegroundColor DarkCyan
        

        $disk_write_sum += $($disk_write)
        $disk_write_formatted = Format-BytesToString($($disk_write))
        $disk_write_sum_formatted = Format-BytesToString($disk_write_sum)

        Write-Host "Disk     Write   : " -ForegroundColor DarkYellow -NoNewline
        Write-Host ($disk_write_formatted) -NoNewline
        Write-Host ('{0,21:n}  B ' -f $($disk_write)) -ForegroundColor DarkGray -NoNewline
        Write-Host ($disk_write_sum_formatted) -ForegroundColor DarkCyan -NoNewline
        Write-Host 'Total                ' -ForegroundColor DarkCyan
    
        $network_totals = Get-NetworkInterfaceTotals -CounterSamples $counterSamples
        $network_received_value = $network_totals.ReceivedBytesPerSecond
        $network_received_sum += $network_received_value

        $network_sent_value = $network_totals.SentBytesPerSecond
        $network_sent_sum += $network_sent_value

        $tailscale_stats = Get-TailscaleAdapterStatistics
        $tailscale_received_absolute = if ($null -ne $tailscale_stats) { [double]$tailscale_stats.ReceivedBytes } else { $null }
        $tailscale_sent_absolute = if ($null -ne $tailscale_stats) { [double]$tailscale_stats.SentBytes } else { $null }
        $tailscale_received_delta_result = Update-TailscaleTotalState -State $tailscale_received_delta_state -AbsoluteBytes $tailscale_received_absolute
        $tailscale_sent_delta_result = Update-TailscaleTotalState -State $tailscale_sent_delta_state -AbsoluteBytes $tailscale_sent_absolute
        $tailscale_received_delta = $tailscale_received_delta_result.DeltaBytes
        $tailscale_sent_delta = $tailscale_sent_delta_result.DeltaBytes
        if ($tailscale_resume_with_snapshot -and $tailscale_received_has_snapshot_baseline) {
            $tailscale_received_latest_result = Update-TailscaleTotalState -State $tailscale_received_latest_state -AbsoluteBytes $tailscale_received_absolute
            $tailscale_received_latest = $tailscale_received_latest_result.LatestBytes
        }
        else {
            $tailscale_received_latest = $tailscale_received_absolute
        }

        if ($tailscale_resume_with_snapshot -and $tailscale_sent_has_snapshot_baseline) {
            $tailscale_sent_latest_result = Update-TailscaleTotalState -State $tailscale_sent_latest_state -AbsoluteBytes $tailscale_sent_absolute
            $tailscale_sent_latest = $tailscale_sent_latest_result.LatestBytes
        }
        else {
            $tailscale_sent_latest = $tailscale_sent_absolute
        }
        $tailscale_received_sum = $tailscale_received_absolute
        $tailscale_sent_sum = $tailscale_sent_absolute

        $network_received_value_formatted = Format-BytesToString($network_received_value)
        $network_received_sum_formatted = Format-BytesToString($network_received_sum)
        $network_sent_value_formatted = Format-BytesToString($network_sent_value)
        $network_sent_sum_formatted = Format-BytesToString($network_sent_sum)
        $tailscale_received_delta_formatted = if ($null -ne $tailscale_received_delta) { Format-BytesToString($tailscale_received_delta) } else { $blank_19 }
        $tailscale_received_value_formatted = if ($null -ne $tailscale_received_latest) { Format-BytesToString($tailscale_received_latest) } else { $blank_19 }
        $tailscale_received_value_raw_formatted = if ($null -ne $tailscale_received_latest) { ('{0,17} Latest' -f $tailscale_received_value_formatted.Trim()) } else { $blank_24 }
        $tailscale_received_sum_formatted = if ($null -ne $tailscale_received_sum) { Format-BytesToString($tailscale_received_sum) } else { $blank_19 }
        $tailscale_sent_delta_formatted = if ($null -ne $tailscale_sent_delta) { Format-BytesToString($tailscale_sent_delta) } else { $blank_19 }
        $tailscale_sent_value_formatted = if ($null -ne $tailscale_sent_latest) { Format-BytesToString($tailscale_sent_latest) } else { $blank_19 }
        $tailscale_sent_value_raw_formatted = if ($null -ne $tailscale_sent_latest) { ('{0,17} Latest' -f $tailscale_sent_value_formatted.Trim()) } else { $blank_24 }
        $tailscale_sent_sum_formatted = if ($null -ne $tailscale_sent_sum) { Format-BytesToString($tailscale_sent_sum) } else { $blank_19 }
        
        Write-Host "Network  Received: " -ForegroundColor DarkYellow -NoNewline
        Write-Host ($network_received_value_formatted) -NoNewline
        Write-Host ('{0,21:n}  B ' -f $network_received_value) -ForegroundColor DarkGray -NoNewline
        Write-Host ($network_received_sum_formatted) -ForegroundColor DarkCyan -NoNewline
        Write-Host 'Total                ' -ForegroundColor DarkCyan
    
        Write-Host "Network  Sent    : " -ForegroundColor DarkYellow -NoNewline
        Write-Host ($network_sent_value_formatted) -NoNewline
        Write-Host ('{0,21:n}  B ' -f $network_sent_value) -ForegroundColor DarkGray -NoNewline
        Write-Host ($network_sent_sum_formatted) -ForegroundColor DarkCyan -NoNewline
        Write-Host 'Total                ' -ForegroundColor DarkCyan

        $Script:hyperV_memory_allocating = $False
        $hyperV_memory_avail = (Get-RoundedCounterValue -CounterSamples $counterSamples -Index 9) * 1048576
        $hyperV_memory_total = (Get-RoundedCounterValue -CounterSamples $counterSamples -Index 10) * 1048576
        if ($Script:hyperV_memory_total_old -ne $hyperV_memory_total) {
            $Script:hyperV_memory_total_old = $hyperV_memory_total
            $Script:hyperV_memory_allocating = $True
        }
        $hyperV_memory_usage = $hyperV_memory_total - $hyperV_memory_avail
        $hyperV_memory_usage_percentage = $hyperV_memory_usage / $hyperV_memory_total * 100

        Write-Host "Hyper-V  Memory  : " -ForegroundColor DarkYellow -NoNewline
        Format-PercentageColor -Percentage $hyperV_memory_usage_percentage -NoNewline
        # Write-Host ('{0,15:n3}  % ' -f $hyperV_memory_usage_percentage) -NoNewline
        if ($Script:hyperV_memory_allocating -eq $True) {
            Write-Host '       [ALLOCATING]       ' -ForegroundColor Green -NoNewline
        }
        else {
            Write-Host '                          ' -NoNewline
        }

        $hyperV_memory_usage_formatted = Format-BytesToString($hyperV_memory_usage)
        $hyperV_memory_total_formatted = Format-BytesToString($hyperV_memory_total)

        Write-Host ($hyperV_memory_usage_formatted) -NoNewline
        Write-Host '/' -ForegroundColor DarkGray -NoNewline
        Write-Host ($hyperV_memory_total_formatted)

        Write-Host "Tailscale Received:" -ForegroundColor DarkYellow -NoNewline
        Write-Host ($tailscale_received_delta_formatted) -ForegroundColor White -NoNewline
        Write-Host $tailscale_received_value_raw_formatted -ForegroundColor DarkCyan -NoNewline
        Write-Host ($tailscale_received_sum_formatted) -ForegroundColor Green -NoNewline
        Write-Host 'Total              ' -ForegroundColor Green

        Write-Host "Tailscale Sent    :" -ForegroundColor DarkYellow -NoNewline
        Write-Host ($tailscale_sent_delta_formatted) -ForegroundColor White -NoNewline
        Write-Host $tailscale_sent_value_raw_formatted -ForegroundColor DarkCyan -NoNewline
        Write-Host ($tailscale_sent_sum_formatted) -ForegroundColor Green -NoNewline
        Write-Host 'Total              ' -ForegroundColor Green
        
        <# param($current_time,
    $record_time,
    $cpu_usage,
    $cpu_user_usage,
    $cpu_privileged_usage,
    $cpu_dpc_usage,
    $memory_usage_percentage, 
    $memory_used,
    $memory_total,
    $memory_committed_percentage,
    $memory_commited,
    $memory_commit_limit,
    $disk_read,
    $disk_write,
    $network_received_value,
    $network_sent_value,
    $disk_read_sum,
    $disk_write_sum,
    $network_received_sum,
    $network_sent_sum,
    $hyperV_memory_usage,
    $hyperV_memory_avail,
    $hyperV_memory_total
) #>
        Write-ToCsv($currentTime,
            $record_time,
            $cpu_usage,
            $cpu_user_usage,
            $cpu_privileged_usage,
            $cpu_dpc_usage,
            $memory_usage_percentage, 
            $memory_used_formatted,
            $memory_total_formatted,
            $memory_committed_percentage,
            $memory_commited_formatted,
            $memory_commit_limit_formatted,
            $disk_read_formatted,
            $disk_write_formatted,
            $network_received_value_formatted,
            $network_sent_value_formatted,
            $tailscale_received_value_formatted,
            $tailscale_sent_value_formatted,
            $disk_read_sum_formatted,
            $disk_write_sum_formatted,
            $network_received_sum_formatted,
            $network_sent_sum_formatted,
            $tailscale_received_sum_formatted,
            $tailscale_sent_sum_formatted,
            $hyperV_memory_usage_percentage,
            $hyperV_memory_usage_formatted,
            $hyperV_memory_total_formatted
        )

        $timestamp += 1
        Write-PerformancerSnapshot -Timestamp $timestamp -DiskReadSum $disk_read_sum -DiskWriteSum $disk_write_sum -NetworkReceivedSum $network_received_sum -NetworkSentSum $network_sent_sum -TailscaleReceivedAbsoluteBytes $tailscale_received_absolute -TailscaleSentAbsoluteBytes $tailscale_sent_absolute
        
        if ([System.Console]::KeyAvailable) {
            $pressedKey = [System.Console]::ReadKey($true).Key
            if ($pressedKey -eq 'q') {
                break
            }
            if ($pressedKey -eq 'c') {
                [System.Console]::SetCursorPosition(0, $cursor_start_height)
                for ($i = 0; $i -lt $display_line_count; $i++) {
                    Write-Host (" " * ($max_length - 1))
                }
                [System.Console]::SetCursorPosition(0, $cursor_start_height)
            }
        }
        
    }
}

Function Get-PerformancerConfig {
    $Script:csv_save_path = Get-ConfiguredDirectory -ConfigFileName 'csv_path.cfg'
    $Script:snapshot_save_path = Get-ConfiguredDirectory -ConfigFileName 'snapshot_path.cfg'
    $Script:snapshot_save_file = Join-Path $Script:snapshot_save_path 'performancer_snapshot.json'
}



Function Write-ToCsvHead {
    $write_tmp = "Forever Performance Monitor 5,Record Time,CPU Usage,CPU User,CPU Privileged,CPU Driver,Physical Memory Usage,Physical Memory,Max Memory Size,Committed Usage,Committed Size,Max Committed Size,Disk Read Rate,Disk Write Rate,Network Receiving Rate,Network Sending Rate,Tailscale Received Rate,Tailscale Sent Rate,Total Disk Read,Total Disk Write,Total Network Received,Total Network Sent,Total Tailscale Received,Total Tailscale Sent,Hyper-V Memory Usage,Hyper-V Memory Available,Hyper-V Memory Total,`n"
    
    $timestamp = Get-Date -Format 'yyyy-MM-dd_HH-mm-ss'
    $Script:csv_save_file = "$Script:csv_save_path\performancer_$timestamp.csv"
    Set-Content -Path $Script:csv_save_file -Value $write_tmp
}

Function Write-ToCsv {
param(
    $current_time,
    $record_time,
    $cpu_usage,
    $cpu_user_usage,
    $cpu_privileged_usage,
    $cpu_dpc_usage,
    $memory_usage_percentage, 
    $memory_used,
    $memory_total,
    $memory_committed_percentage,
    $memory_commited,
    $memory_commit_limit,
    $disk_read,
    $disk_write,
    $network_received_value,
    $network_sent_value,
    $tailscale_received_value,
    $tailscale_sent_value,
    $disk_read_sum,
    $disk_write_sum,
    $network_received_sum,
    $network_sent_sum,
    $tailscale_received_sum,
    $tailscale_sent_sum,
    $hyperV_memory_usage_percentage,
    $hyperV_memory_usage,
    $hyperV_memory_total
)
$write_tmp = @(
    $current_time,
    $record_time,
    $cpu_usage,
    $cpu_user_usage,
    $cpu_privileged_usage,
    $cpu_dpc_usage,
    $memory_usage_percentage,
    $memory_used,
    $memory_total,
    $memory_committed_percentage,
    $memory_commited,
    $memory_commit_limit,
    $disk_read,
    $disk_write,
    $network_received_value,
    $network_sent_value,
    $tailscale_received_value,
    $tailscale_sent_value,
    $disk_read_sum,
    $disk_write_sum,
    $network_received_sum,
    $network_sent_sum,
    $tailscale_received_sum,
    $tailscale_sent_sum,
    $hyperV_memory_usage_percentage,
    $hyperV_memory_usage,
    $hyperV_memory_total
) | Join-String -Separator '","'

    # $write_tmp = "$current_time,"
    # $write_tmp += "$record_time,"
    # $write_tmp += "$cpu_usage,"
    # $write_tmp += "$cpu_user_usage,"
    # $write_tmp += "$cpu_privileged_usage,"
    # $write_tmp += "$cpu_dpc_usage,"
    # $write_tmp += "$memory_usage_percentage,"
    # $write_tmp += "$memory_used,"
    # $write_tmp += "$memory_total,"
    # $write_tmp += "$memory_committed_percentage,"
    # $write_tmp += "$memory_commited,"
    # $write_tmp += "$memory_commit_limit,"
    # $write_tmp += "$disk_read,"
    # $write_tmp += "$disk_write,"
    # $write_tmp += "$network_received_value,"
    # $write_tmp += "$network_sent_value,"
    # $write_tmp += "$disk_read_sum,"
    # $write_tmp += "$disk_write_sum,"
    # $write_tmp += "$network_received_sum,"
    # $write_tmp += "$network_sent_sum,"
    # $write_tmp += "$hyperV_memory_usage,"
    # $write_tmp += "$hyperV_memory_avail,"
    # $write_tmp += "$hyperV_memory_total,"
    # $write_tmp += "`n"
    try { Add-Content -Path $Script:csv_save_file -Value "`"$write_tmp`"" -Force }
    catch { Continue }
}
