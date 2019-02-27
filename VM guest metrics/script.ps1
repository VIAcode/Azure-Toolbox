
param([String]$ResourceGroupName = $(Read-Host -prompt "Enter name of resource group with virtual machines") )

$subscriptions = Get-AzureRmSubscription

if(![bool]$subscriptions)
{
	write-host "Could not find any subscriptions. Please, log in to Azure before launching the script." -ForegroundColor Yellow;
    Break
}

if($subscriptions.Count -gt 1)
{
	write-host 'Found more than one subscription. Please, select the subscription.' -ForegroundColor Yellow;
	for($i=0; $i -lt $subscriptions.Count; $i++)
	{
		echo "$i. $($subscriptions[$i].Name)"
	}
	do {
		write-host -nonewline "Your choise: "
		$inputString = read-host
		$value = $inputString -as [Int]
		$ok = ($value -ge 0) -and ($value -lt $subscriptions.Count)
		if ( -not $ok )
			{ write-host "Could not find such subscription. Try again." -ForegroundColor Yellow}
		}
	until ( $ok )
	Select-AzureRmSubscription -SubscriptionObject $subscriptions[$value]
}

$AllVMs = Get-AzureRmVM -ResourceGroupName $ResourceGroupName

if(![bool]$AllVMs -or $AllVMs.Count -lt 1)
{
    write-host 'No machines found. Script execution finished.' -ForegroundColor Yellow;
    Break
}

$Windows = $AllVMs | Where-Object {[bool]$_.OSProfile.WindowsConfiguration}

if(![bool]$Windows -or $Windows.Count -lt 1)
{
    write-host 'No Windows-based machines found. Check README for info about guest metrics for Linux machines. Script execution finished.' -ForegroundColor Yellow;
    Break
}

write-host "Following machines found and will be upgraded:" -ForegroundColor Yellow
foreach($vm in $Windows)
{
    write-host $vm.Name
}

write-host -nonewline "Continue? (Y/N)"
$response = read-host
if ( $response -ne "Y" ) { Break }

$names = "";

For($i=0; $i -lt ($Windows.Length-1); $i++){
	$names = $names + "`"" + $Windows[$i].Name + "`",`n"
}
	
$names = $names + "`"" + $Windows[$VMs.Length-1].Name + "`""

$content = Get-Content -Path "parameters.template.json" | Out-String

$content = $content.Replace("{0}", $names)
	
$template = ((Get-Item -Path ".\").FullName + "\extension.json")
$parameters = ((Get-Item -Path ".\").FullName + "\extension.parameters.json")

if(Test-Path -Path  $parameters)
{
    Clear-Content $parameters
}

$content >> $parameters
write-host "Parameters file: $parameters"
write-host "Deploying resources to Azure..."
New-AzureRmResourceGroupDeployment -Name "GuestExtensionsDeployment" -ResourceGroupName $ResourceGroupName -TemplateFile $template -TemplateParameterFile $parameters

Write-Host -NoNewLine 'Script execution finished';