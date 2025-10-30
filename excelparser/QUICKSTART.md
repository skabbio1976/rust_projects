# Quick Start - Excel till Server Deployment

Snabbguide för att deploya servrar från Excel-beställning.

## 🚀 Snabbstart (3 steg)

### 1. Konvertera Excel till YAML

```bash
cd excelparser
cargo run --release -- -f Serverbeställning.xlsx -o inventory.yml
```

### 2. Konfigurera vCenter (en gång)

Skapa `group_vars/all.yml`:
```yaml
vcenter_hostname: "vcenter.company.local"
vcenter_username: "admin@vsphere.local"
vcenter_password: "YourPassword"
vcenter_datacenter: "Datacenter"
vcenter_cluster: "Production"
vcenter_datastore: "datastore1"
vcenter_folder: "Servers"
default_admin_password: "P@ssw0rd123!"
```

### 3. Deploya Servrar

```bash
ansible-playbook -i inventory.yml deploy-servers.yml
```

**Klart!** 🎉

## 📋 Detaljerad Workflow

### Steg 1: Excel-beställning (PL/Ark)

PL eller Arkitekt fyller i Excel med:
- Beställarinformation (rad 2-5)
- Serverdata (från rad 8)
- Nätverkskonfiguration

**Ingen YAML-kunskap krävs!** ✅

### Steg 2: Konvertera (Drift)

```bash
# Bygg verktyget (första gången)
cd excelparser
cargo build --release

# Konvertera Excel → YAML
./target/release/excelparser -f ../Serverbeställning.xlsx -o inventory.yml
```

### Steg 3: Deploya (Drift)

```bash
# Med default-inställningar
ansible-playbook -i inventory.yml deploy-servers.yml

# Med custom vCenter
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e vcenter_hostname=vcenter.company.local \
  -e vcenter_username=admin@vsphere.local \
  -e vcenter_password=YourPassword

# Bara lab-servrar
ansible-playbook -i inventory.yml deploy-servers.yml --limit lab

# Med verbose output
ansible-playbook -i inventory.yml deploy-servers.yml -vvv
```

## ✅ Vad Händer Automatiskt?

1. ✅ VM skapas från template med korrekt specs
2. ✅ Nätverk konfigureras (IP, VLAN, gateway, DNS)
3. ✅ Disks konfigureras (C: och L: om angivet)
4. ✅ Domain join (om domain anges i Excel)
5. ✅ Beställarinformation sparas i vCenter annotations
6. ✅ VMware Tools väntas på och verifieras

## 🔧 Anpassning

### Custom Template per Server

I Excel, ändra Template-kolumnen eller överskriv i playbook:
```bash
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e server_template=Windows-Server-2022-Custom
```

### Custom Folder

```bash
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e vcenter_folder=Production/Servers
```

## 🐛 Felsökning

### Excel kunde inte parsas
- Kontrollera att kolumnnamn stämmer
- Verifiera att Name-kolumnen (D) inte är tom för serverrader

### VM skapas inte
- Kontrollera template finns i vCenter
- Verifiera cluster/datastore har resurser
- Kontrollera vCenter-credentials

### Nätverk fungerar inte
- Verifiera VLAN-namn matchar i vCenter
- Kontrollera IP-adresser är korrekta

## 📊 Exempel Output

```
PLAY [Deploy Servers from Excel Beställning] **************************

TASK [Set server variables from inventory] ****************************
ok: [srv001]

TASK [Create VM from template] ****************************************
changed: [srv001] => (item=srv001)

TASK [Wait for VMware Tools] *****************************************
ok: [srv001]

TASK [Configure static IP] ********************************************
changed: [srv001]

TASK [Join domain if specified] **************************************
changed: [srv001]

PLAY RECAP *************************************************************
srv001    : ok=6    changed=3    unreachable=0    failed=0
```

## 🎯 Fördelar

- ✅ **Enkelt för PL/Ark** - Excel är bekant
- ✅ **Inga YAML-fel** - Automatisk konvertering
- ✅ **Snabb deployment** - Allt automatiskt
- ✅ **Spårbart** - Beställarinfo i vCenter
- ✅ **Repeterbart** - Samma process varje gång

---

**För mer detaljer, se `DEPLOYMENT_GUIDE.md`**

