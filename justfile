# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

serve:
	trunk serve --watch src --open 
