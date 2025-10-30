# Deployment Guide - Excel till vCenter Server Deployment

Denna guide beskriver hela workflow från Excel-beställning till deployerad server i vCenter.

## 📋 Workflow Översikt

```
Excel Beställning (PL/Ark) 
    ↓
Rust Tool (excelparser) → YAML Inventory
    ↓
Ansible Playbook → Deploy Servers i vCenter
    ↓
Klar Server med nätverk, domänanslutning, etc.
```

## 🚀 Steg-för-Steg

### Steg 1: Få Excel-beställning

PL eller Arkitekt fyller i Excel-beställningen med:
- Beställarinformation (namn, e-post, telefon)
- Serverdata (namn, cluster, domain, template, roll, specs)
- Nätverkskonfiguration (IP, VLAN, gateway, DNS)

### Steg 2: Konvertera Excel till YAML

```bash
cd /path/to/excelparser

# Konvertera Excel till YAML inventory
cargo run --release -- -f Serverbeställning.xlsx -o inventory.yml

# Eller med timestamp
cargo run --release -- -f Serverbeställning.xlsx --file
```

Detta skapar en YAML-fil som Ansible kan läsa direkt.

### Steg 3: Konfigurera vCenter-inställningar

Skapa `group_vars/all.yml` eller använd `--extra-vars`:

```yaml
# group_vars/all.yml
vcenter_hostname: "vcenter.example.com"
vcenter_username: "administrator@vsphere.local"
vcenter_password: "YourPassword"
vcenter_datacenter: "Datacenter"
vcenter_cluster: "Cluster"
vcenter_datastore: "datastore1"
vcenter_folder: "Servers"
default_admin_password: "P@ssw0rd123!"
domain_admin_user: "domain\Administrator"
domain_admin_password: "DomainPassword"
```

### Steg 4: Deploya Servrar

```bash
# Deploya alla servrar från inventory
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e inventory_file=inventory.yml \
  -e vcenter_hostname=vcenter.example.com \
  -e vcenter_username=admin@vsphere.local \
  -e vcenter_password=YourPassword

# Deploya bara lab-servrar
ansible-playbook -i inventory.yml deploy-servers.yml \
  --limit lab \
  -e inventory_file=inventory.yml

# Med verbose output för debugging
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e inventory_file=inventory.yml \
  -vvv
```

## 🔧 Anpassning

### Custom Templates

Uppdatera `server_template` i Excel eller överskriv i playbook:

```yaml
# I deploy-servers.yml eller --extra-vars
server_template: "Windows-Server-2022-Custom-Template"
```

### Custom Folder Structure

Organisera VMs i olika folders:

```yaml
vcenter_folder: "{{ groups[inventory_hostname] }}/{{ server_role }}"
```

### Domain Join

Om domän anges i Excel deployeras automatiskt. Om inte, stannar servern i WORKGROUP.

## 📊 Inventory Struktur

Den genererade YAML-filen har följande struktur:

```yaml
all:
  children:
    lab:
      hosts:
        srv001:
          ansible_host: '172.27.206.10'
          ansible_user: Administrator
          ansible_connection: vmware_tools
          vars:
            name: srv001
            description: Application Server
            domain: winlab.lc
            template: Windows-Server-2022-Template
            role: AppServer
            vcpu: 4
            memory: 8192
            # ... nätverksinställningar
  vars:
    bestallare_namn: Håkan Pahlm
    bestallare_email: hakan.pahlm@company.se
    bestallare_telefon: '08-101010'
    kontaktperson_media: Stefan Åkerlund
```

## 🎯 Vad Playbook Gör

1. **Läser Inventory** - Laddar den genererade YAML-filen
2. **Skapar VMs** - Från templates i vCenter med korrekt specs
3. **Konfigurerar Nätverk** - IP, VLAN, gateway, DNS
4. **Konfigurerar Disks** - Primär + sekundär disk om angiven
5. **Domain Join** - Om domain anges i Excel
6. **Sätter Annotations** - Beställarinformation i vCenter
7. **Verifierar** - Väntar på VMware Tools och verifierar anslutning

## 🔐 Säkerhet

### Använd Ansible Vault för Lösenord

```bash
# Skapa vault file
ansible-vault create vault.yml

# Lägg till lösenord
vcenter_password: "YourPassword"
default_admin_password: "P@ssw0rd123!"
domain_admin_password: "DomainPassword"

# Kör med vault
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e @vault.yml \
  --ask-vault-pass
```

## 🐛 Troubleshooting

### Problem: VM skapas inte
- Kontrollera template finns i vCenter
- Verifiera cluster/datastore har resurser
- Kontrollera vCenter-credentials

### Problem: Nätverk fungerar inte
- Verifiera VLAN-namn matchar i vCenter
- Kontrollera IP-adresser är korrekta
- Verifiera gateway och DNS-inställningar

### Problem: Domain join misslyckas
- Kontrollera DNS-konfiguration
- Verifiera domain credentials
- Kontrollera nätverksanslutning till DC

## 📝 Exempel: Komplett Workflow

```bash
# 1. Konvertera Excel
cd excelparser
cargo run --release -- -f ../Serverbeställning.xlsx -o ../inventory.yml

# 2. Granska inventory (valfritt)
cat ../inventory.yml

# 3. Deploya servrar
cd ..
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e inventory_file=inventory.yml \
  -e vcenter_hostname=vcenter.company.local \
  -e vcenter_username=admin@vsphere.local \
  -e vcenter_password=$(cat .vcenter_pass) \
  -vvv

# 4. Verifiera i vCenter eller testa anslutning
ansible -i inventory.yml all -m win_ping
```

## 🎉 Fördelar med denna Workflow

- ✅ **Enkelt för PL/Ark** - Excel är bekant format
- ✅ **Automatiserat** - Inga manuella YAML-redigeringar
- ✅ **Säkert** - Validering och konsistent struktur
- ✅ **Spårbart** - Beställarinformation sparas i vCenter
- ✅ **Repeterbart** - Samma process varje gång
- ✅ **Snabb** - Automatisk deployment från Excel till server

## 📚 Relaterade Dokument

- `README.md` - Rust tool dokumentation
- `deploy-servers.yml` - Ansible playbook
- Ansible AD FS playbook - För referens på VMware Tools connection

---

**Lycka till med deployment! 🚀**

