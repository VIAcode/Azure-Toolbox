# Guest metrics
## Intro
In Microsoft Azure cloud you can create Virtual Machines. Linux-based or Windows-based. For these machines you can emit some performance counters to Azure and from Azure to SCOM using AzureMP. Recently there was a switch in API's that do all that work and a switch in approach.

Now all the non-host metrics can be emitted to Azure Monitor and if you want to collect metrics for VMs by AzureMP, you'll have to do switch to this approach.

To do so, first read the guides:
  What are guest metrics:
  How to enable guest metrics on Windows machines to send them to Monitor:
  How to enable guest metrics on Linux machines to send them to Monitor:
 
## What this repository provides
A way to ease your switch to Azure Monitor for Windows-based machines
