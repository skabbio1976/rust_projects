# Excel Parser för Ansible Inventory

En Rust-applikation som konverterar Excel-filer med serverbeställningar till Ansible inventory-filer i YAML-format.

## Översikt

Denna parser läser serverbeställningar från Excel-filer och genererar strukturerade Ansible inventory-filer som kan användas direkt med Ansible-playbooks för att hantera virtuella servrar i vCenter.

## Funktioner

- ✅ Läser serverdata från Excel-filer (`.xlsx`)
- ✅ Automatisk gruppering baserat på domain/cluster (lab, uac, production)
- ✅ Extraherar beställarinformation (namn, e-post, telefon, kontaktperson)
- ✅ Genererar Ansible-kompatibel YAML-struktur
- ✅ Konfigurerad för VMware Tools-anslutning
- ✅ Komplett nätverkskonfiguration (IP, VLAN, gateway, DNS)
- ✅ Server-specifikationer (vCPU, minne, hårddiskar, roller)

## Installation

### Förutsättningar
- Rust (senaste stabila version)
- Excel-fil med serverbeställningar

### Byggning
```bash
cargo build --release
```

## Användning

### Grundläggande användning
```bash
# Visa YAML på skärmen
cargo run -- -f src/Serverbeställningxlsx.xlsx

# Spara till fil med tidsstämpel
cargo run -- -f src/Serverbeställningxlsx.xlsx --file

# Spara till specifik fil
cargo run -- -f src/Serverbeställningxlsx.xlsx -o hosts.yml
```

### Kommandoradsalternativ
- `-f, --xlsxfile <XLSX>` - Input Excel-fil (obligatorisk)
- `-o, --output-yaml <YAML>` - Output YAML-fil
- `--file` - Spara YAML till fil även utan -o flaggan

## Excel-filstruktur

Parsern förväntar sig en Excel-fil med följande struktur. **Viktigt**: Parsern identifierar serverrader genom att kolla om Name-kolumnen (kolumn D) inte är tom eller innehåller "Name" (header).

### Beställarinformation (B2-B5)
- **B2**: Beställarens Namn: [Namn]
- **B3**: E-Post: [E-postadress]
- **B4**: Telefon: [Telefonnummer]
- **B5**: Kontaktperson media: [Kontaktperson]

### Serverdata (från rad 8)
- **Kolumn C**: Description
- **Kolumn D**: Name (hostname) - *Används för att identifiera serverrader*
- **Kolumn E**: Cluster
- **Kolumn G**: Domain
- **Kolumn I**: Template
- **Kolumn J**: Roll
- **Kolumn L**: vCPU
- **Kolumn M**: Memory
- **Kolumn N**: Harddrive1 (C:)
- **Kolumn O**: Harddrive2 (L:)
- **Kolumn X**: VLAN1
- **Kolumn Z**: IP1
- **Kolumn AA**: SubnetMask1
- **Kolumn AB**: Gateway1
- **Kolumn AC**: DNS1_1
- **Kolumn AD**: DNS1_2

## Genererad YAML-struktur

```yaml
all:
  children:
    lab:
      hosts:
        srv001:
          ansible_host: '172.27.206.10'
          ansible_user: Administrator
          ansible_connection: vmware_tools
          # ... server-specifik variabler
    uac:
      hosts:
        srv003:
          # ... server-specifik variabler
    production:
      hosts:
        srv004:
          # ... server-specifik variabler
  vars:
    bestallare_namn: Håkan Pahlm
    bestallare_email: hakan.pahlm@somecompany.se
    bestallare_telefon: '08-101010'
    kontaktperson_media: Stefan Åkerlund 08-101010
```

## Användning med Ansible

### Komplett Deployment Workflow

**Steg 1:** Konvertera Excel till YAML
```bash
cargo run --release -- -f Serverbeställning.xlsx -o inventory.yml
```

**Steg 2:** Deploya servrar i vCenter
```bash
ansible-playbook -i inventory.yml deploy-servers.yml \
  -e vcenter_hostname=vcenter.company.local \
  -e vcenter_username=admin@vsphere.local \
  -e vcenter_password=YourPassword
```

**Det är allt!** Servrarna deployas automatiskt med:
- ✅ Korrekt specs (vCPU, minne, diskar)
- ✅ Nätverkskonfiguration (IP, VLAN, gateway, DNS)
- ✅ Domain join (om angivet)
- ✅ Beställarinformation i vCenter annotations

Se `DEPLOYMENT_GUIDE.md` och `QUICKSTART.md` för detaljerad dokumentation.

### Grundläggande användning
```bash
# Använd den genererade inventory-filen
ansible-playbook -i inventory.yml deploy-servers.yml

# Kör mot specifik grupp
ansible-playbook -i inventory.yml deploy-servers.yml --limit lab
```

### Exempel på Ansible-playbook
```yaml
---
- name: Deploy servers
  hosts: all
  gather_facts: false
  tasks:
    - name: Display server information
      debug:
        msg: |
          Server: {{ inventory_hostname }}
          Beställare: {{ bestallare_namn }}
          E-post: {{ bestallare_email }}
          IP: {{ ip1 }}
          Roll: {{ role }}
```

### vCenter-annotations
```yaml
- name: Set vCenter annotations
  vmware_guest:
    name: "{{ inventory_hostname }}"
    annotations:
      - "Beställare: {{ bestallare_namn }}"
      - "E-post: {{ bestallare_email }}"
      - "Telefon: {{ bestallare_telefon }}"
      - "Kontakt: {{ kontaktperson_media }}"
```

## Gruppering

Servrar grupperas automatiskt baserat på domain/cluster:
- **lab**: Servrar med `winlab.lc` domain eller `lab_cluster` cluster
- **uac**: Servrar med `uac.lab.se` domain eller `uac_cluster` cluster  
- **production**: Servrar med `prod.lab.se` domain eller `prod_cluster` cluster

## Variabler

### Globala variabler (all.vars)
- `bestallare_namn` - Beställarens namn
- `bestallare_email` - Beställarens e-post
- `bestallare_telefon` - Beställarens telefon
- `kontaktperson_media` - Kontaktperson för media

### Host-specifika variabler
- `ansible_host` - IP-adress för anslutning
- `ansible_user` - Användarnamn (Administrator)
- `ansible_connection` - Anslutningstyp (vmware_tools)
- `name` - Hostname
- `description` - Serverbeskrivning
- `cluster` - vCenter cluster
- `domain` - Windows domain
- `template` - VM template
- `role` - Serverroll
- `vcpu` - Antal vCPU
- `memory` - Minne (GB)
- `harddrive1_c` - C: diskstorlek (GB)
- `harddrive2_l` - L: diskstorlek (GB)
- `vlan1` - Primärt VLAN
- `ip1` - Primär IP-adress
- `subnet_mask1` - Primär nätmask
- `gateway1` - Primär gateway
- `dns1` - Primär DNS-server
- `vlan2` - Sekundärt VLAN (om tillgängligt)
- `ip2` - Sekundär IP-adress (om tillgängligt)
- `subnet_mask2` - Sekundär nätmask (om tillgängligt)

## Utveckling

### Byggning för utveckling
```bash
cargo build
```

### Testning
```bash
cargo test
```

### Kontrollera kod
```bash
cargo check
cargo clippy
```

## Beroenden

- `calamine` - Excel-filläsning
- `clap` - Kommandoradsargument
- `serde` - Serialisering
- `serde_yml` - YAML-generering
- `chrono` - Tidsstämplar

## Licens

Detta projekt är utvecklat för intern användning.

## Support

För frågor eller problem, kontakta Johan Kallio.
