# Scripts Executor - Airgap Script Runner

Wrapper-executor som hanterar Python/PowerShell-scripts och deras dependencies i airgapped miljö.

## 🎯 Koncept

**Teamet utvecklar i internet-zon** → **Du paketerar** → **Airgap deployment**

### Utveckling (Internet-zon)
- ✅ Teamet utvecklar scripts med normal `pip install` / `Install-Module`
- ✅ Använder alla dependencies de behöver
- ✅ Testar lokalt med internet access
- ✅ Inga begränsningar under utveckling

### Deployment (Airgapped)
- ✅ Du paketerar allt en gång (med alla deps)
- ✅ Skapar self-contained package
- ✅ Kör scripts i airgap utan dependency-problem
- ✅ Single binary executor + packaged scripts

## 🏗️ Arkitektur

```
Development (Internet):
  Team scripts (Python/PS) + pip install
    ↓
Package Script:
  - Build executor binary (Rust/Go)
  - Package Python venv
  - Package PS modules
  - Create config
    ↓
Airgapped Deployment:
  executor binary + scripts + deps
    ↓
Runs scripts without internet
```

## 🚀 Snabbstart

### 1. Setup Development Environment

```bash
cd scripts-executor
./setup-dev-environment.sh
```

Detta skapar:
- `team-scripts/` - Där teamet utvecklar
- `python-env/` - Python virtual environment med dependencies
- Exempel scripts

### 2. Teamet Utvecklar

```bash
# Aktivera Python environment
source python-env/bin/activate  # Linux/Mac
# eller
python-env\Scripts\activate     # Windows

# Installera dependencies som vanligt
pip install requests pandas numpy

# Skriv scripts i team-scripts/
vim team-scripts/my_script.py

# Testa lokalt
python team-scripts/my_script.py
```

### 3. Du Paketerar

```bash
# När teamet är klar, paketera allt
./package-for-airgap.sh
```

Detta skapar `scripts-airgap-package.tar.gz` med:
- Executor binary (static, no deps)
- Team scripts
- Python environment (packaged)
- Config file

### 4. Deploy i Airgap

```bash
# I airgapped miljö
tar -xzf scripts-airgap-package.tar.gz
cd scripts-airgap-package
./install.sh

# Kör scripts
./executor config.json deploy-servers
./executor config.json validate-inventory input.yml
```

## 📋 Konfiguration

### config.json

```json
{
  "python_path": "python3",
  "python_env": "./python-env",
  "powershell_path": "pwsh",
  "scripts": [
    {
      "name": "deploy-servers",
      "type": "python",
      "script_path": "./team-scripts/deploy_servers.py",
      "python_deps": ["ansible", "pyvmomi"],
      "args": [],
      "working_dir": "./"
    },
    {
      "name": "validate-inventory",
      "type": "python",
      "script_path": "./team-scripts/validate_inventory.py",
      "python_deps": ["pyyaml"],
      "args": []
    },
    {
      "name": "generate-report",
      "type": "powershell",
      "script_path": "./team-scripts/Generate-Report.ps1",
      "ps_modules": ["ImportExcel"],
      "args": []
    }
  ]
}
```

## 💡 Fördelar

### För Teamet
- ✅ **Utvecklar bekvämt** - Med internet, inga begränsningar
- ✅ **Kan använda alla deps** - pip install som vanligt
- ✅ **Testar lokalt** - Ingen airgap-simulering behövs
- ✅ **Enkelt att lära** - Python/PS är bekant

### För Dig
- ✅ **Kontrollerar deployment** - Du paketerar vad som behövs
- ✅ **Single binary** - Executor är static, inga deps
- ✅ **Airgap-friendly** - Allt paketeras en gång
- ✅ **Code review** - Review team scripts innan packaging

### För Airgap
- ✅ **Self-contained** - Allt i ett package
- ✅ **Inga runtime deps** - Static binary executor
- ✅ **Enkelt deploy** - tar -xzf och kör
- ✅ **Förutsägbar** - Testat innan airgap

## 🔄 Workflow

### Typisk Utvecklingscykel

**Vecka 1: Teamet Utvecklar**
```bash
cd scripts-executor
source python-env/bin/activate
pip install nytt-bibliotek
vim team-scripts/nytt_script.py
python team-scripts/nytt_script.py  # Testa
git commit -m "Add new script"
```

**Vecka 2: Du Paketerar**
```bash
git pull
vim config.json  # Lägg till nytt script
./package-for-airgap.sh
# Testa package lokalt
```

**Vecka 3: Deploy i Airgap**
```bash
# Kopiera package till airgap
# Kör via executor
```

## 📚 Dokumentation

- `README.md` - Denna fil (översikt)
- `WORKFLOW.md` - Detaljerad workflow-guide
- `config.json.example` - Exempel på konfiguration

## 🛠️ Teknisk Info

### Executor Binary

Executor finns i två versioner:
- **Rust** (`executor.rs`) - Preferred, static binary med musl
- **Go** (`executor.go`) - Fallback, static binary med CGO_ENABLED=0

### Build

```bash
# Rust (preferred)
cargo build --release --target x86_64-unknown-linux-musl

# Go (fallback)
CGO_ENABLED=0 go build -ldflags="-w -s" -o executor executor.go
```

### Python Environment

Python virtual environment packeteras som tarball:
- Innehåller alla dependencies
- Extraheras i airgap-miljö
- Executor aktiverar automatiskt

## 🔐 Säkerhet

- ✅ Team scripts kan reviewas innan packaging
- ✅ Du kontrollerar vad som deployas
- ✅ Static binaries reducerar attack surface
- ✅ Paketerad miljö är förutsägbar

## 📊 Jämförelse

| Approach | Utveckling | Deployment | Airgap |
|----------|-----------|------------|--------|
| **Pure Python** | ✅ Enkelt | ❌ Dependency hell | ❌ Svårt |
| **Pure Compiled** | ❌ Svårt för team | ✅ Enkelt | ✅ Perfekt |
| **Hybrid (denna)** | ✅ Enkelt | ✅ Du paketerar | ✅ Perfekt |

## 🎯 Användningsfall

### Automation Scripts
- Server deployment scripts
- Config validation
- Report generation
- Data processing

### Infrastructure Tools
- Ansible wrappers
- Cloud API scripts
- Monitoring tools
- Backup scripts

## 📝 License

Detta projekt är utvecklat för intern användning.

## 👥 Bidrag

Teamet utvecklar scripts i `team-scripts/`, du paketerar och deployar.

---

**Detta ger bästa av båda världar: Teamet utvecklar bekvämt, du paketerar säkert för airgap!** 🎯
