# Guest metrics for Windows VMs 

## Intro
This solution deploys Guest metrics extensions on Azure **Windows Virtual Machines** that are monitored by SCOM [**Azure Management Pack**](https://www.microsoft.com/en-us/download/details.aspx?id=50013) 1.7.0.0.

Guest Metrics created for Virtual Machines by this extension will be emitted to Azure Monitor that is used by Azure Management Pack for delivering performance metrcis to SCOM. 

More information about Guest extenstions can be found in links below:
  + What are guest metrics: [link](https://docs.microsoft.com/en-us/azure/cost-management/azure-vm-extended-metrics)
  + How enable guest metrics on Windows machines to send them to Monitor: [link](https://docs.microsoft.com/en-us/azure/azure-monitor/platform/collect-custom-metrics-guestos-resource-manager-vm)
  + How enable guest metrics on Linux machines to send them to Monitor: [link](https://docs.microsoft.com/en-us/azure/azure-monitor/platform/collect-custom-metrics-linux-telegraf)
 
## What this repository provides
A way to ease your switch to Azure Monitor for Windows-based machines

## What this repository contains
+ extension.json — ARM template that you can upload to Azure and use. It allows to specify an array of VM's that script will update
+ script.ps1 — PowerShell script that updates all **Windows** machines in a single resource group
+ extension.parameters.json — additional file that is required by script
+ parameters.template.json — additional file that is required by script

## ARM Template Usage
Template allows apply Guest metrics extension for the selected Windows virtual machines in the resource group. 
Each VM name must be enumarated in **Windows VM Names** input box in the following format: 

                                                     `[vm1,vm2..vmn]` 

Note. Template will update default extenstion that is enabled by "Diagnostics settings-Enable Guest-Level monitoring" if it was enabled before.

### TemplateDeploymentSteps
1. Copy full information from extension.json file
2. Go to Templates in Azure portal and click [+Add]
3. In **General** tab type name and description: guest-azure-monitor-win-vms
4. In **Add template** tab paste contents of extension.json file
5. Deploy template. 
5. Go to Metrics tab for VM. New metric namespace  will be listed: `azure.vm.windows.guestmetrics`. 

Note. After template applied for single VM it takes up to 5 minutes for the new guest metric namespace to appear. 

## Script Usage
Script allows automate extension deployment on all Windows Virtual Machines for selected resource group.
### Prerequisites 
+ script.ps1 requires [Azure PS Module](https://docs.microsoft.com/en-us/powershell/azure/install-az-ps?view=azps-1.4.0) to be installed
+ Virtual Machines and Resource Group resided in the same location will be updated 

### ScriptDeploymentSteps
  1. Download all files to a single location
  2. Launch powershell and login to Azure
  3. Launch script and enter the resource group where Windows machines are located


## Limitations
  1. Turned off machines will not be updated
  2. Linux machines will not be updated
  3. Script will not be executed if VMs and resource group are in different locations (like VMs are in East US and Resource Group is in North Europe) 

