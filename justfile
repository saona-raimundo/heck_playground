# use PowerShell instead of sh:
set shell := ["nu", "-c"]

# Build, serve and watch
serve:
	trunk serve --watch src --open 

# Publish to GitHub pages
publish:
	trunk build
	try { rm --recursive docs }
	cp --recursive dist docs

clean:
	cargo clean
	trunk clean