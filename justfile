# use PowerShell instead of sh:
set shell := ["nu", "-c"]

# Build, serve and watch
serve:
	trunk serve --watch src --open 

# Publish to GitHub pages
publish:
	trunk build
	cp --recursive dist docs