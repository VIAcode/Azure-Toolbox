# Guest metrics for Windows VMs 

## Intro
This solution deploys Guest metrics extensions to Azure **Windows Virtual Machines** that are monitored by SCOM **Azure Management Pack** version starting 1.7.0.0. 
Guest Metrics created for Virtual Machines by this extension will be emitted to Azure Monitor that is used by Azure Management Pack for delivering performance metrcis to SCOM. 

More information about Guest extenstions can be found in links below:

  + What are guest metrics: [link](https://docs.microsoft.com/en-us/azure/cost-management/azure-vm-extended-metrics)
  + How enable guest metrics on Windows machines to send them to Monitor: [link](https://docs.microsoft.com/en-us/azure/azure-monitor/platform/collect-custom-metrics-guestos-resource-manager-vm)
  + How enable guest metrics on Linux machines to send them to Monitor: [link](https://docs.microsoft.com/en-us/azure/azure-monitor/platform/collect-custom-metrics-linux-telegraf)
 
## What this repository provides
A way to ease your switch to Azure Monitor for Windows-based machines

## What this repository contains
script.ps1 — PowerShell script, requires [Azure PS Module](https://docs.microsoft.com/en-us/powershell/azure/install-az-ps?view=azps-1.4.0), updates all Windows machines in a single resource group

extension.json — ARM template that you can upload to Azure and use. Allows you to specify an array of VM's that script will try to update

extension.parameters.json — additional file that is required by script
parameters.template.json — additional file that is required by script

## How to use script
  1. Download all files to a single location
  2. Launch powershell and login to Azure
  3. Launch script and enter the resource group where Windows machines are located

## Limitations
  1. Turned off machines will not be updated
  2. Linux machines will not be updated
  3. Script will not execute if VMs and resource group are in different locations (like VMs are in East US and Resource Group is in North Europe) 

