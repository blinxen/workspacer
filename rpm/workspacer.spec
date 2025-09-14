# There are no tests yet
%bcond_with check

Name:           workspacer
Version:        0.4.0
Release:        1%{?dist}
Summary:        Terminal workspace manager used with terminal editors
# Apache-2.0 OR MIT
# MIT
License:        (Apache-2.0 OR MIT) AND MIT
URL:            https://github.com/blinxen/%{name}
Source:         %{url}/archive/%{version}/%{name}-%{version}.tar.gz

BuildRequires:  cargo-rpm-macros >= 24

%description
Terminal workspace manager used with terminal editors like neovim or helix

%files
%license LICENSE
%license LICENSE.dependencies
%doc README.md
%{_bindir}/workspacer

%prep
%autosetup -n %{name}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies
%cargo_build

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%changelog
* Sun Sep 14 2025 blinxen <h-k-81@hotmail.com> - 0.4.0-1
- Update to version 0.4.0
- Add fuzzy search for filtering workspaces
* Fri Aug 15 2025 blinxen <h-k-81@hotmail.com> - 0.3.0-1
- Initial package
