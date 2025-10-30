# Workflow Guide - Scripts Executor

Praktisk guide för hur teamet utvecklar och du paketerar för airgap.

## 🎯 Konceptet i Korthet

**Teamet utvecklar** → **Du paketerar** → **Airgap deployment**

## 📋 Steg-för-Steg Workflow

### Steg 1: Setup Development Environment (En gång)

```bash
cd scripts-executor
./setup-dev-environment.sh
```

Detta skapar:
- `team-scripts/` - Där teamet utvecklar
- `python-env/` - Python virtual environment
- Exempel scripts

### Steg 2: Teamet Utvecklar (Internet-zon)

```bash
# Aktivera Python environment
source python-env/bin/activate

# Installera dependencies (som vanligt!)
pip install requests pandas numpy ansible pyvmomi

# Skriv scripts
vim team-scripts/deploy_servers.py
vim team-scripts/validate_config.py

# Testa lokalt
python team-scripts/deploy_servers.py --help
```

**Teamet behöver INTE tänka på airgap - de utvecklar normalt!**

### Steg 3: Du Uppdaterar Config

När teamet lägger till nya scripts, uppdatera `config.json`:

```json
{
  "scripts": [
    {
      "name": "deploy-servers",
      "type": "python",
      "script_path": "./team-scripts/deploy_servers.py",
      "python_deps": ["ansible", "pyvmomi"],
      "args": []
    },
    {
      "name": "validate-config",
      "type": "python",
      "script_path": "./team-scripts/validate_config.py",
      "python_deps": ["pyyaml"],
      "args": []
    }
  ]
}
```

### Steg 4: Du Paketerar (Innan Airgap)

```bash
# När teamet är klar, paketera allt
./package-for-airgap.sh
```

Detta:
1. Bygger executor binary (Rust eller Go)
2. Packeterar Python environment med alla dependencies
3. Packeterar team scripts
4. Skapar install script
5. Skapar tarball

**Resultat:** `scripts-airgap-package.tar.gz` - Komplett, self-contained!

### Steg 5: Deploy i Airgap

```bash
# Kopiera till airgapped miljö (USB, GitLab, etc.)
cp scripts-airgap-package.tar.gz /path/to/airgap/

# I airgapped miljö
tar -xzf scripts-airgap-package.tar.gz
cd scripts-airgap-package
./install.sh

# Kör scripts
./executor config.json deploy-servers
./executor config.json validate-config inventory.yml
```

## 🔄 Typisk Utvecklingscykel

### Vecka 1: Teamet Utvecklar
```bash
# Teamet jobbar i internet-zon
cd scripts-executor
source python-env/bin/activate
pip install nytt-bibliotek  # Installera vad de behöver
vim team-scripts/nytt_script.py
python team-scripts/nytt_script.py  # Testa
git commit -m "Add new script"
```

### Vecka 2: Du Paketerar
```bash
# Du hämtar senaste ändringar
git pull

# Uppdatera config.json med nya scripts
vim config.json

# Paketera
./package-for-airgap.sh

# Testa package lokalt (valfritt)
tar -xzf scripts-airgap-package.tar.gz -C /tmp/test
cd /tmp/test/scripts-airgap-package
./install.sh
./executor config.json nytt-script
```

### Vecka 3: Deploy i Airgap
```bash
# Deploy package till airgapped miljö
# Kör scripts via executor
```

## 💡 Best Practices

### För Teamet

1. **Använd virtual environment**
   ```bash
   source python-env/bin/activate
   ```

2. **Dokumentera dependencies**
   ```bash
   pip freeze > requirements.txt
   ```

3. **Testa scripts lokalt**
   ```bash
   python team-scripts/my_script.py
   ```

4. **Commit regelbundet**
   ```bash
   git add team-scripts/
   git commit -m "Add new automation script"
   ```

### För Dig

1. **Review team scripts** innan packaging
2. **Uppdatera config.json** när nya scripts läggs till
3. **Testa package** innan airgap deployment
4. **Versionera packages** (lägg till datum/version i filnamn)

## 🎯 Fördelar med denna Approach

### För Teamet
- ✅ Utvecklar bekvämt med internet
- ✅ Kan använda alla dependencies de vill
- ✅ Inga begränsningar under utveckling
- ✅ Testar lokalt som vanligt

### För Dig
- ✅ Kontrollerar vad som deployas
- ✅ Paketerar en gång med alla deps
- ✅ Airgap-deployment är enkelt
- ✅ Single binary executor (ingen dependency hell)

### För Airgap
- ✅ Self-contained package
- ✅ Inga runtime dependencies
- ✅ Enkelt att deploya
- ✅ Förutsägbar deployment

## 📊 Jämförelse

| Approach | Utveckling | Deployment | Airgap |
|----------|-----------|------------|--------|
| **Pure Python** | ✅ Enkelt | ❌ Dependency hell | ❌ Svårt |
| **Pure Compiled** | ❌ Svårt för team | ✅ Enkelt | ✅ Perfekt |
| **Hybrid (denna)** | ✅ Enkelt | ✅ Du paketerar | ✅ Perfekt |

## 🚀 Nästa Steg

1. Testa setup script
2. Låt teamet utveckla några scripts
3. Paketera och testa lokalt
4. Deploy i airgap och verifiera

---

**Detta ger bästa av båda världar: Teamet utvecklar bekvämt, du paketerar säkert!** 🎯

