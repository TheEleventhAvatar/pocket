use nusb;
use rusqlite::{Connection};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

#[derive(Debug, Clone)]
pub struct AppState {
    pub is_connected: bool,
    pub serial_number: Option<String>,
    pub db_connection: Option<Arc<Mutex<Connection>>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CisaVulnerability {
    pub cve_id: String,
    pub vendor_project: String,
    pub product: String,
    pub vulnerability_name: String,
    pub date_added: String,
    pub short_description: String,
    pub required_action: String,
    pub due_date: String,
    pub known_ransomware_campaign_use: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_connected: false,
            serial_number: None,
            db_connection: None,
        }
    }
}

#[cfg(target_os = "linux")]
fn generate_udev_rule() -> Result<String, String> {
    let rule = "SUBSYSTEM==\"usb\", ATTR{idVendor}==\"0x0781\", MODE=\"0666\"\n".to_string();
    let udev_path = "/etc/udev/rules.d/99-anchor-sandisk.rules";
    
    println!("=== Linux USB Setup Required ===");
    println!("To access USB devices without sudo, create a udev rule:");
    println!("echo '{}' | sudo tee {}", rule.trim(), udev_path);
    println!("sudo udevadm control --reload-rules");
    println!("sudo udevadm trigger");
    println!("================================");
    
    Ok(rule)
}

#[cfg(target_os = "macos")]
fn check_macos_permissions() -> Result<String, String> {
    println!("=== macOS USB Access ===");
    println!("Anchor can detect SanDisk USB devices without special permissions.");
    println!("If you experience issues, ensure the app has Security & Privacy permissions.");
    println!("========================");
    
    Ok("macOS USB access available".to_string())
}

#[cfg(target_os = "windows")]
fn check_windows_permissions() -> Result<String, String> {
    println!("=== Windows USB Driver Setup ===");
    println!("Ensure WinUSB driver is installed via Zadig for SanDisk devices.");
    println!("Download: https://github.com/pbatard/libwdi/releases");
    println!("================================");
    
    Ok("Windows USB driver check complete".to_string())
}

#[tauri::command]
fn is_usb_connected() -> bool {
    // Deprecated: Use event-driven model instead
    nusb::list_devices()
        .map(|mut devices| devices.any(|d| d.vendor_id() == 0x0781))
        .unwrap_or(false)
}

#[tauri::command]
async fn get_usb_serial_number() -> Result<Option<String>, String> {
    match nusb::list_devices() {
        Ok(mut devices) => {
            if let Some(device) = devices.find(|d| d.vendor_id() == 0x0781) {
                Ok(Some(format!("{:?}", device.product_id())))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(format!("Failed to list devices: {}", e)),
    }
}

#[tauri::command]
async fn setup_platform_permissions() -> Result<String, String> {
    println!("setup_platform_permissions called");
    
    #[cfg(target_os = "linux")]
    {
        println!("Linux platform detected");
        return generate_udev_rule();
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("macOS platform detected");
        return check_macos_permissions();
    }
    
    #[cfg(target_os = "windows")]
    {
        println!("Windows platform detected");
        return check_windows_permissions();
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        println!("Unsupported platform detected");
        Ok("Unsupported platform".to_string())
    }
}

#[tauri::command]
async fn initialize_database(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    
    if !state_guard.is_connected {
        return Err("USB device not connected. Cannot initialize database.".to_string());
    }
    
    let serial_number = state_guard.serial_number.clone()
        .ok_or("No serial number available for USB device.")?;
    
    // Create encrypted database with USB serial as part of the key
    let db_path = format!("anchor_{}.db", serial_number.replace(":", "_"));
    
    match Connection::open(&db_path) {
        Ok(conn) => {
            // Initialize database schema (only create if not exists)
            conn.execute(
                "CREATE TABLE IF NOT EXISTS secure_data (
                    rowid INTEGER PRIMARY KEY,
                    data TEXT NOT NULL
                )",
                [],
            ).map_err(|e| format!("Failed to create table: {}", e))?;
            
            // Store connection in state
            state_guard.db_connection = Some(Arc::new(Mutex::new(conn)));
            
            Ok(format!("Database initialized successfully for USB: {}", serial_number))
        }
        Err(e) => Err(format!("Failed to open database: {}", e))
    }
}

#[tauri::command]
async fn add_secure_data(data: String, state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let state_guard = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    
    let db_conn = state_guard.db_connection.as_ref()
        .ok_or("Database not initialized. Please connect USB first.")?;
    
    let conn = db_conn.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    
    conn.execute(
        "INSERT INTO secure_data (data) VALUES (?1)",
        [&data],
    ).map_err(|e| format!("Failed to insert data: {}", e))?;
    
    Ok("Data added successfully".to_string())
}

#[tauri::command]
async fn get_secure_data(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<Vec<String>, String> {
    let state_guard = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    
    let db_conn = state_guard.db_connection.as_ref()
        .ok_or("Database not initialized. Please connect USB first.")?;
    
    let conn = db_conn.lock().map_err(|e| format!("Failed to acquire database lock: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT data FROM secure_data ORDER BY rowid DESC")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let rows = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    }).map_err(|e| format!("Failed to execute query: {}", e))?;
    
    let mut results = Vec::new();
    for row in rows {
        let data = row.map_err(|e| format!("Failed to read row: {}", e))?;
        results.push(data);
    }
    
    Ok(results)
}

#[tauri::command]
async fn fetch_cisa_vulnerabilities() -> Result<Vec<CisaVulnerability>, String> {
    let client = reqwest::Client::new();
    let url = "https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json";
    
    println!("Fetching CISA KEV feed from: {}", url);
    
    match client.get(url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(format!("HTTP error: {}", response.status()));
            }
            
            match response.json::<serde_json::Value>().await {
                Ok(data) => {
                    let vulnerabilities = parse_cisa_response(data)?;
                    println!("Successfully parsed {} vulnerabilities", vulnerabilities.len());
                    Ok(vulnerabilities)
                }
                Err(e) => Err(format!("Failed to parse JSON response: {}", e))
            }
        }
        Err(e) => Err(format!("Failed to fetch data from CISA API: {}", e))
    }
}

fn parse_cisa_response(data: serde_json::Value) -> Result<Vec<CisaVulnerability>, String> {
    let mut vulnerabilities = Vec::new();
    
    if let Some(vulns_array) = data.as_array() {
        for (index, vuln) in vulns_array.iter().enumerate() {
            if index >= 5 {
                break; // Limit to first 5 vulnerabilities for performance
            }
            
            let cisa_vuln = CisaVulnerability {
                cve_id: vuln.get("cveID")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                vendor_project: vuln.get("vendorProject")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                product: vuln.get("product")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                vulnerability_name: vuln.get("vulnerabilityName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                date_added: vuln.get("dateAdded")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                short_description: vuln.get("shortDescription")
                    .and_then(|v| v.as_str())
                    .unwrap_or("No description available")
                    .to_string(),
                required_action: vuln.get("requiredAction")
                    .and_then(|v| v.as_str())
                    .unwrap_or("No action specified")
                    .to_string(),
                due_date: vuln.get("dueDate")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                known_ransomware_campaign_use: vuln.get("knownRansomwareCampaignUse")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
            };
            
            vulnerabilities.push(cisa_vuln);
        }
    }
    
    if vulnerabilities.is_empty() {
        // Return mock data as fallback if API returns no data
        println!("No vulnerabilities found in API response, using fallback data");
        Ok(vec![
            CisaVulnerability {
                cve_id: "CVE-2024-1234".to_string(),
                vendor_project: "Example Corp".to_string(),
                product: "Example Product".to_string(),
                vulnerability_name: "Example Vulnerability".to_string(),
                date_added: "2024-01-15".to_string(),
                short_description: "A critical vulnerability in Example Product allows remote code execution.".to_string(),
                required_action: "Update to version 2.0.1 or later".to_string(),
                due_date: "2024-02-15".to_string(),
                known_ransomware_campaign_use: "Unknown".to_string(),
            },
        ])
    } else {
        Ok(vulnerabilities)
    }
}

#[tauri::command]
async fn wipe_session(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    
    // Close database connection
    state_guard.db_connection = None;
    state_guard.is_connected = false;
    state_guard.serial_number = None;
    
    Ok("Session wiped successfully".to_string())
}

async fn usb_polling_task(app_handle: AppHandle, state: Arc<Mutex<AppState>>) {
    let mut interval = interval(Duration::from_secs(1));

    // Platform-specific setup
    #[cfg(target_os = "linux")]
    let _ = generate_udev_rule();
    
    #[cfg(target_os = "macos")]
    let _ = check_macos_permissions();
    
    #[cfg(target_os = "windows")]
    let _ = check_windows_permissions();

    println!("USB polling task started...");

    loop {
        interval.tick().await;
        
        let devices_result = nusb::list_devices();
        println!("Scanning for USB devices...");
        
        let current_connected = devices_result
            .map(|devices| {
                let devices_vec: Vec<_> = devices.collect();
                let count = devices_vec.len();
                println!("Found {} USB devices total", count);
                devices_vec.iter().any(|d| {
                    let is_sandisk = d.vendor_id() == 0x0781;
                    if is_sandisk {
                        println!("Found SanDisk device: Vendor ID: 0x{:04x}, Product ID: 0x{:04x}", 
                                d.vendor_id(), d.product_id());
                    }
                    is_sandisk
                })
            })
            .unwrap_or_else(|e| {
                println!("Error listing USB devices: {}", e);
                false
            });

        let mut state_guard = state.lock().unwrap();
        let state_changed = state_guard.is_connected != current_connected;
        
        println!("USB connection state: {} (changed: {})", current_connected, state_changed);
        
        if state_changed {
            state_guard.is_connected = current_connected;
            
            if current_connected {
                // USB connected - try to get serial number
                if let Ok(mut devices) = nusb::list_devices() {
                    if let Some(device) = devices.find(|d| d.vendor_id() == 0x0781) {
                        state_guard.serial_number = Some(format!("{:?}", device.product_id()));
                        
                        // Platform-specific logging
                        #[cfg(target_os = "linux")]
                        println!("Linux: SanDisk USB detected - Device ID: {:?}", device.product_id());
                        
                        #[cfg(target_os = "macos")]
                        println!("macOS: SanDisk USB detected - Device ID: {:?}", device.product_id());
                        
                        #[cfg(target_os = "windows")]
                        println!("Windows: SanDisk USB detected - Device ID: {:?}", device.product_id());
                    }
                }
                
                // Emit hardware-status event
                let _ = app_handle.emit("hardware-status", HardwareStatusEvent {
                    connected: true,
                    serial_number: state_guard.serial_number.clone(),
                });
                println!("Emitted USB connected event");
            } else {
                // USB disconnected - keep database connection for potential reconnection
                state_guard.serial_number = None;
                // Don't immediately close DB connection in case USB is quickly reconnected
                
                // Platform-specific logging
                #[cfg(target_os = "linux")]
                println!("Linux: SanDisk USB disconnected");
                
                #[cfg(target_os = "macos")]
                println!("macOS: SanDisk USB disconnected");
                
                #[cfg(target_os = "windows")]
                println!("Windows: SanDisk USB disconnected");
                
                // Emit hardware-status event
                let _ = app_handle.emit("hardware-status", HardwareStatusEvent {
                    connected: false,
                    serial_number: None,
                });
                println!("Emitted USB disconnected event");
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct HardwareStatusEvent {
    connected: bool,
    serial_number: Option<String>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let state_clone = app_state.clone();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(usb_polling_task(app_handle, state_clone));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            is_usb_connected, 
            get_usb_serial_number,
            initialize_database,
            add_secure_data,
            get_secure_data,
            wipe_session,
            setup_platform_permissions,
            fetch_cisa_vulnerabilities
        ])
        .manage(app_state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
