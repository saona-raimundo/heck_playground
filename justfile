# use PowerShell instead of sh:
set shell := ["nu", "-c"]

# Build, serve and watch
serve:
	trunk serve --watch src --open 

# Publish to GitHub pages
publish:
	trunk build --release
	try { rm --recursive docs }
	cp --recursive dist docs
	# Replace relative paths by absolute paths 
	open docs\index.html | str replace --all `href="/` `href="./` | str replace --all `from '/` `from './` | str replace --all `'/heck_playground` `'./heck_playground` | save docs\index.html --force  

clean:
	cargo clean
	trunk clean