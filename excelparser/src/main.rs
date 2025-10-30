use calamine::{open_workbook_auto, Reader};
use clap::{Arg, Command};
use serde::Serialize;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};
use chrono::Local;

#[derive(Debug, Serialize)]
struct AnsibleHost {
    ansible_host: String,
    ansible_user: Option<String>,
    ansible_password: Option<String>,
    ansible_ssh_private_key_file: Option<String>,
    ansible_connection: Option<String>,
    ansible_port: Option<u16>,
    #[serde(flatten)]
    vars: HashMap<String, serde_yml::Value>,
}

#[derive(Debug, Serialize)]
struct AnsibleInventory {
    all: AnsibleGroups,
}

#[derive(Debug, Serialize)]
struct AnsibleGroups {
    children: HashMap<String, AnsibleGroup>,
    vars: Option<HashMap<String, serde_yml::Value>>,
}

#[derive(Debug, Serialize)]
struct AnsibleGroup {
    hosts: HashMap<String, AnsibleHost>,
    vars: Option<HashMap<String, serde_yml::Value>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("excelparser")
        .version("1.0")
        .author("Johan Kallio")
        .about("Parsar serverbeställning.xlsx till host.yml")
        .arg(Arg::new("xlsxfile")
            .short('f')
            .long("xlsxfile")
            .value_name("XLSX")
            .help("Input Excel-fil")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output-yaml")
            .value_name("YAML")
            .help("Output YAML-fil"))
        .arg(Arg::new("file")
            .long("file")
            .help("Spara YAML till fil även utan -o")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let xlsx_path = matches.get_one::<String>("xlsxfile").unwrap();
    let output_path = matches.get_one::<String>("output").map_or_else(
        || {
            let timestamp = Local::now().format("%Y%m%d%H%M").to_string();
            PathBuf::from(format!("{timestamp}_serverbeställning.yml"))
        },
        PathBuf::from,
    );

    let mut workbook = open_workbook_auto(xlsx_path)?;
    let range = workbook
        .worksheet_range("Blad1")
        .expect("Kunde inte hitta bladet 'Blad1'");

    // Läs generell information från B2-B5 (rad 1-4, kolumn 1)
    let mut general_vars: HashMap<String, serde_yml::Value> = HashMap::new();
    let rows_vec: Vec<_> = range.rows().collect();
    
    if let Some(row1) = rows_vec.get(1) {
        if let Some(cell) = row1.get(1) {
            let value = cell.to_string();
            if value.contains("Beställarens Namn:") {
                let name = value.replace("Beställarens Namn:", "").trim().to_string();
                general_vars.insert("bestallare_namn".to_string(), serde_yml::Value::String(name));
            }
        }
    }
    
    if let Some(row2) = rows_vec.get(2) {
        if let Some(cell) = row2.get(1) {
            let value = cell.to_string();
            if value.contains("E-Post:") {
                let email = value.replace("E-Post:", "").trim().to_string();
                general_vars.insert("bestallare_email".to_string(), serde_yml::Value::String(email));
            }
        }
    }
    
    if let Some(row3) = rows_vec.get(3) {
        if let Some(cell) = row3.get(1) {
            let value = cell.to_string();
            if value.contains("Telefon:") {
                let phone = value.replace("Telefon:", "").trim().to_string();
                general_vars.insert("bestallare_telefon".to_string(), serde_yml::Value::String(phone));
            }
        }
    }
    
    if let Some(row4) = rows_vec.get(4) {
        if let Some(cell) = row4.get(1) {
            let value = cell.to_string();
            if value.contains("Kontaktperson media:") {
                let contact = value.replace("Kontaktperson media:", "").trim().to_string();
                general_vars.insert("kontaktperson_media".to_string(), serde_yml::Value::String(contact));
            }
        }
    }

    // Läs från rad 7 (index 7) där headers finns
    let mut rows = rows_vec.into_iter().skip(7);
    
    // Hoppa över header-raden
    rows.next();

    let mut groups: HashMap<String, AnsibleGroup> = HashMap::new();

    for row in rows {
        // Kolla om raden innehåller serverdata (Name-kolumnen inte är null eller tom)
        if let Some(name_cell) = row.get(3) {
            let name = name_cell.to_string();
            if !name.trim().is_empty() && name != "Name" {
                let host = parse_server_row(row)?;
                let group_name = determine_group(&host);
                
                groups.entry(group_name.clone()).or_insert(AnsibleGroup {
                    hosts: HashMap::new(),
                    vars: None,
                });
                
                let hostname = host.vars.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                groups.get_mut(&group_name).unwrap()
                    .hosts.insert(hostname, host);
            }
        }
    }

    let inventory = AnsibleInventory {
        all: AnsibleGroups {
            children: groups,
            vars: Some(general_vars),
        },
    };

    let yaml = serde_yml::to_string(&inventory)?;

    if matches.get_flag("file") || matches.contains_id("output") {
        let mut file = File::create(output_path)?;
        file.write_all(yaml.as_bytes())?;
        println!("✅ YAML skapad!");
    } else {
        println!("{yaml}");
    }

    Ok(())
}

fn parse_server_row<T>(row: &[T]) -> Result<AnsibleHost, Box<dyn std::error::Error>> 
where
    T: std::fmt::Display,
{
    let mut vars: HashMap<String, serde_yml::Value> = HashMap::new();
    
    // Mappa kolumner baserat på Excel-strukturen S
    if let Some(description) = row.get(2) {
        vars.insert("description".to_string(), serde_yml::Value::String(description.to_string()));
    }
    
    if let Some(name) = row.get(3) {
        vars.insert("name".to_string(), serde_yml::Value::String(name.to_string()));
    }
    
    if let Some(cluster) = row.get(4) {
        vars.insert("cluster".to_string(), serde_yml::Value::String(cluster.to_string()));
    }
    
    if let Some(domain) = row.get(6) {
        vars.insert("domain".to_string(), serde_yml::Value::String(domain.to_string()));
    }
    
    if let Some(template) = row.get(8) {
        vars.insert("template".to_string(), serde_yml::Value::String(template.to_string()));
    }
    
    if let Some(role) = row.get(9) {
        vars.insert("role".to_string(), serde_yml::Value::String(role.to_string()));
    }
    
    if let Some(vcpu) = row.get(11) {
        if let Ok(vcpu_num) = vcpu.to_string().parse::<i32>() {
            vars.insert("vcpu".to_string(), serde_yml::Value::Number(vcpu_num.into()));
        }
    }
    
    if let Some(memory) = row.get(12) {
        if let Ok(memory_num) = memory.to_string().parse::<i32>() {
            vars.insert("memory".to_string(), serde_yml::Value::Number(memory_num.into()));
        }
    }
    
    if let Some(hdd1) = row.get(13) {
        if let Ok(hdd1_num) = hdd1.to_string().parse::<i32>() {
            vars.insert("harddrive1_c".to_string(), serde_yml::Value::Number(hdd1_num.into()));
        }
    }
    
    if let Some(hdd2) = row.get(14) {
        if let Ok(hdd2_num) = hdd2.to_string().parse::<i32>() {
            vars.insert("harddrive2_l".to_string(), serde_yml::Value::Number(hdd2_num.into()));
        }
    }
    
    // Nätverksinställningar
    if let Some(vlan1) = row.get(23) {
        vars.insert("vlan1".to_string(), serde_yml::Value::String(vlan1.to_string()));
    }
    
    if let Some(ip1) = row.get(25) {
        vars.insert("ip1".to_string(), serde_yml::Value::String(ip1.to_string()));
    }
    
    if let Some(subnet1) = row.get(26) {
        vars.insert("subnet_mask1".to_string(), serde_yml::Value::String(subnet1.to_string()));
    }
    
    if let Some(gateway1) = row.get(27) {
        vars.insert("gateway1".to_string(), serde_yml::Value::String(gateway1.to_string()));
    }
    
    if let Some(dns1) = row.get(28) {
        vars.insert("dns1".to_string(), serde_yml::Value::String(dns1.to_string()));
    }
    
    // Andra VLAN om det finns
    if let Some(vlan2) = row.get(30) {
        if !vlan2.to_string().is_empty() {
            vars.insert("vlan2".to_string(), serde_yml::Value::String(vlan2.to_string()));
        }
    }
    
    if let Some(ip2) = row.get(32) {
        if !ip2.to_string().is_empty() {
            vars.insert("ip2".to_string(), serde_yml::Value::String(ip2.to_string()));
        }
    }
    
    if let Some(subnet2) = row.get(33) {
        if !subnet2.to_string().is_empty() {
            vars.insert("subnet_mask2".to_string(), serde_yml::Value::String(subnet2.to_string()));
        }
    }
    
    // Använd första IP-adressen som ansible_host
    let ansible_host = vars.get("ip1")
        .and_then(|v| v.as_str())
        .unwrap_or("localhost")
        .to_string();
    
    Ok(AnsibleHost {
        ansible_host,
        ansible_user: Some("Administrator".to_string()),
        ansible_password: None,
        ansible_ssh_private_key_file: None,
        ansible_connection: Some("vmware_tools".to_string()),
        ansible_port: None,
        vars,
    })
}

fn determine_group(host: &AnsibleHost) -> String {
    // Bestäm grupp baserat på domain eller cluster
    if let Some(domain) = host.vars.get("domain") {
        if let Some(domain_str) = domain.as_str() {
            if domain_str.contains("prod") {
                return "production".to_string();
            } else if domain_str.contains("uac") {
                return "uac".to_string();
            } else if domain_str.contains("lab") {
                return "lab".to_string();
            }
        }
    }
    
    if let Some(cluster) = host.vars.get("cluster") {
        if let Some(cluster_str) = cluster.as_str() {
            if cluster_str.contains("prod") {
                return "production".to_string();
            } else if cluster_str.contains("uac") {
                return "uac".to_string();
            } else if cluster_str.contains("lab") {
                return "lab".to_string();
            }
        }
    }
    
    "default".to_string()
}
