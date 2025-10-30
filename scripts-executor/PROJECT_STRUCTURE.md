# Project Structure

## Översikt

Detta är ett fristående projekt för att hantera Python/PowerShell-scripts i airgapped miljöer.

## Filstruktur

```
scripts-executor/
├── executor.rs              # Rust executor (preferred)
├── executor.go              # Go executor (fallback)
├── Cargo.toml              # Rust dependencies
├── go.mod                   # Go module
├── config.json.example     # Exempel på konfiguration
├── package-for-airgap.sh   # Script för att paketera för airgap
├── setup-dev-environment.sh # Setup script för utvecklingsmiljö
├── README.md               # Huvuddokumentation
├── WORKFLOW.md             # Detaljerad workflow-guide
├── LICENSE                 # MIT License
└── .gitignore             # Git ignore rules
```

## Utvecklingsstruktur (efter setup)

```
scripts-executor/
├── executor.rs/go          # Executor binary source
├── team-scripts/           # Teamets scripts (utvecklas här)
│   ├── deploy_servers.py
│   ├── validate_config.py
│   └── Generate-Report.ps1
├── python-env/            # Python virtual environment (skapas av setup)
│   ├── bin/
│   ├── lib/
│   └── ...
├── config.json            # Konfiguration (skapas från example)
└── ...
```

## Package-struktur (efter packaging)

```
scripts-airgap-package/
├── executor                # Static binary (Rust eller Go)
├── install.sh              # Installation script
├── config.json             # Konfiguration
├── team-scripts/           # Team scripts
│   └── ...
├── python-env.tar.gz       # Packaged Python environment
└── README_AIRGAP.md       # Installation guide (genereras)
```

## Komponenter

### Executor Binary
- **Rust version** (`executor.rs`) - Preferred, static binary
- **Go version** (`executor.go`) - Fallback, static binary
- Kör Python/PowerShell-scripts
- Hanterar virtual environments automatiskt

### Packaging Scripts
- **package-for-airgap.sh** - Paketerar allt för airgap
- **setup-dev-environment.sh** - Sätter upp utvecklingsmiljö

### Dokumentation
- **README.md** - Översikt och snabbstart
- **WORKFLOW.md** - Detaljerad workflow-guide
- **PROJECT_STRUCTURE.md** - Denna fil

## Separation från excelparser

Detta projekt är helt separerat från `excelparser`:
- Eget git repository
- Egen dokumentation
- Egen build process
- Kan användas med vilka scripts som helst

## Användning med andra projekt

Scripts-executor kan användas med:
- excelparser output (YAML inventory)
- Ansible playbooks
- Custom automation scripts
- Vilket projekt som helst som behöver script-execution i airgap

---

**Standalone projekt för airgap script execution** 🎯

