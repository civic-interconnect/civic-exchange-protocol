# DEVELOPER

Update CHANGELOG.md first (2 places: beginning and end).
Update versions in provenance/ptag.json (about 4 places).

### 1. Prepare the release
```shell
git add .
git commit -m "Prep vx.y.z"
git push -u origin main
```

After all GitHub actions pass, continue. 

### 2. Tag the release
```shell
git tag vx.y.z -m "x.y.z"
git push origin vx.y.z
```

A GitHub Action will **build**, **publish to PyPI** (Trusted Publishing), **create a GitHub Release** with artifacts, and **deploy versioned docs** with `mike`.

### 3. Verify
- The tag appears at: https://github.com/civic-interconnect/<repo>/tags
```
