# armtemplatevalidator

Tool for Azure ARM Template Validation.

Example usage:
``` bash
./armtemplatevalidator azuredeploy.json 
The template "azuredeploy.json" is Invalid!
65:        "Owner": "[parameters(owner')]",
67:        "Scenario": "[parameters('tagName)]"

```

or:
``` bash
./armtemplatevalidator ./new_file/*
The template "./new_file/azuredeploy2.json" is Invalid!
63:      "name":"[parameters'keyVaultName')",
66:        "Creator":"[parameters'creator')]",
72:        "enabledForDeployment":"[parameters('enableVaultForDeployment')",
75:        "accessPolicies":"parameters('accessPolicies')]",

```

or:
``` bash
./armtemplatevalidator ./*
All Temaplates Are Valid!
```
