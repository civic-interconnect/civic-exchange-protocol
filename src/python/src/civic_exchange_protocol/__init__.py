from importlib.metadata import PackageNotFoundError
from importlib.metadata import version as _version

try:
    # Distribution name from pyproject.toml
    __version__ = _version("civic-exchange-protocol")
except PackageNotFoundError:  # pragma: no cover - mainly for editable installs
    __version__ = "0.0.0"
