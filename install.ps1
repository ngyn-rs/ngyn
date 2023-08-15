#!/usr/bin/env pwsh
# Copyright 2018 the Deno authors. All rights reserved. MIT license.

$ErrorActionPreference = 'Stop'

$Project = "rustle"
$GithubRepo = "rustle-rs/rustle-rs"
$Target = 'x86_64-windows'
$Install = $env:BP_INSTALL
$BinDir = if ($Install) { "$Install" } else { "$Home\.$Project-bin" }
$Zip = "$BinDir\$Project.zip"
$Exe = "$BinDir\$Project.exe"

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Version = if ($v) { "v${v}" } elseif ($args.Length -eq 1) { $args.Get(0) } else { $null }

$Uri = "https://github.com/$GithubRepo/releases/download/${Version}/$Project-${Target}.zip"
if (!$Version) {
  $Uri = "https://github.com/$GithubRepo/releases/latest/download/${Project}-${Target}.zip"
}

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest -Uri $Uri -OutFile $Zip

Expand-Archive -Path $Zip -DestinationPath $BinDir -Force

Remove-Item $Zip

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

Write-Output "$Project was installed successfully to $Exe"
Write-Output "Run with '--help' to get started"
