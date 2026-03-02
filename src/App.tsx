import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface HardwareStatusEvent {
  connected: boolean;
  serial_number?: string;
}

interface SecureData {
  data: string;
}

interface CisaVulnerability {
  cve_id: string;
  vendor_project: string;
  product: string;
  vulnerability_name: string;
  date_added: string;
  short_description: string;
  required_action: string;
  due_date: string;
  known_ransomware_campaign_use: string;
}

function App() {
  const [isUnlocked, setIsUnlocked] = useState(false);
  const [usbSerial, setUsbSerial] = useState<string | null>(null);
  const [dbInitialized, setDbInitialized] = useState(false);
  const [secureData, setSecureData] = useState<SecureData[]>([]);
  const [vulnerabilities, setVulnerabilities] = useState<CisaVulnerability[]>([]);
  const [newData, setNewData] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const setupHardwareListener = async () => {
      console.log("Setting up hardware listener...");
      const unlisten = await listen<HardwareStatusEvent>("hardware-status", (event) => {
        console.log("Hardware status event received:", event);
        const { connected, serial_number } = event.payload;
        
        if (connected) {
          console.log("USB Connected - Unlocking app. Serial:", serial_number);
          setIsUnlocked(true);
          setUsbSerial(serial_number || null);
          setDbInitialized(false);
          setSecureData([]);
          setVulnerabilities([]); // Clear threat intelligence on reconnect
          // Check for existing database after a short delay
          setTimeout(() => checkExistingDatabase(), 500);
        } else {
          console.log("USB Disconnected - Locking app");
          setIsUnlocked(false);
          setUsbSerial(null);
          setDbInitialized(false);
          setSecureData([]);
          setVulnerabilities([]); // Clear threat intelligence on disconnect
          setNewData("");
        }
        setError(null);
      });

      // Get platform-specific setup info
      try {
        // Test with a different command first
        const info = await invoke<string>("is_usb_connected");
        console.log("Test command result:", info);
      } catch (err) {
        console.log("Test command error:", err);
        try {
          const info = await invoke<string>("setup_platform_permissions");
          console.log("Platform info:", info);
        } catch (err2) {
          console.log("Platform info error:", err2);
        }
      }

      return unlisten;
    };

    setupHardwareListener();
  }, []);

  const initializeDatabase = async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await invoke<string>("initialize_database");
      console.log(result);
      setDbInitialized(true);
      await loadSecureData();
    } catch (err) {
      setError(`Failed to initialize database: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  // Auto-load data if database was previously initialized
  const checkExistingDatabase = async () => {
    if (usbSerial && !dbInitialized) {
      try {
        await loadSecureData();
        setDbInitialized(true);
      } catch (err) {
        // Database doesn't exist yet, that's okay
        console.log("No existing database found");
      }
    }
  };

  const loadSecureData = async () => {
    try {
      const data = await invoke<string[]>("get_secure_data");
      setSecureData(data.map(item => ({ data: item })));
    } catch (err) {
      setError(`Failed to load data: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const addSecureData = async () => {
    if (!newData.trim()) return;
    
    const dataToAdd = newData.trim();
    setNewData(""); // Clear input immediately
    
    try {
      setLoading(true);
      setError(null);
      await invoke<string>("add_secure_data", { data: dataToAdd });
      await loadSecureData();
    } catch (err) {
      setError(`Failed to add data: ${err}`);
      setNewData(dataToAdd); // Restore data on error
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && newData.trim()) {
      addSecureData();
    }
  };

  const fetchVulnerabilities = async () => {
    try {
      setLoading(true);
      setError(null);
      const vulns = await invoke<CisaVulnerability[]>("fetch_cisa_vulnerabilities");
      setVulnerabilities(vulns);
    } catch (err) {
      setError(`Failed to fetch threat intelligence: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const SecurityDashboard = () => (
    <div className="bg-black border border-green-500 rounded-lg p-4 font-mono">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-green-400 font-bold text-lg">THREAT INTELLIGENCE</h2>
        <div className="flex items-center space-x-2">
          <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
          <span className="text-green-400 text-xs">LIVE</span>
        </div>
      </div>
      
      {vulnerabilities.length === 0 ? (
        <div className="text-center py-8">
          <button
            onClick={fetchVulnerabilities}
            disabled={loading}
            className="bg-green-600 text-black px-6 py-3 rounded border border-green-400 hover:bg-green-500 transition-colors disabled:opacity-50 font-mono text-sm"
          >
            {loading ? "FETCHING..." : "FETCH CISA KEV FEED"}
          </button>
        </div>
      ) : (
        <div className="space-y-3">
          <div className="text-xs text-gray-400 mb-4">
            LATEST 5 CVEs • CISA KNOWN EXPLOITED VULNERABILITIES
          </div>
          {vulnerabilities.map((vuln, index) => (
            <div key={index} className="border border-gray-700 rounded p-3 bg-gray-900">
              <div className="flex items-start justify-between mb-2">
                <div className="flex items-center space-x-2">
                  <span className="text-red-400 font-bold text-xs">CVE</span>
                  <span className="text-green-400 font-mono text-sm">{vuln.cve_id}</span>
                </div>
                <div className="text-xs text-gray-500">{vuln.date_added}</div>
              </div>
              
              <div className="text-yellow-300 text-xs mb-2 font-mono">{vuln.vulnerability_name}</div>
              
              <div className="text-gray-400 text-xs mb-2">{vuln.short_description}</div>
              
              <div className="grid grid-cols-2 gap-4 text-xs">
                <div>
                  <span className="text-blue-400">VENDOR:</span>
                  <span className="text-gray-300 ml-2">{vuln.vendor_project}</span>
                </div>
                <div>
                  <span className="text-blue-400">PRODUCT:</span>
                  <span className="text-gray-300 ml-2">{vuln.product}</span>
                </div>
              </div>
              
              <div className="border-t border-gray-700 pt-2 mt-2">
                <div className="text-orange-400 text-xs mb-1">REQUIRED ACTION:</div>
                <div className="text-gray-300 text-xs">{vuln.required_action}</div>
              </div>
              
              {vuln.due_date && (
                <div className="text-xs text-gray-500">
                  DUE: <span className="text-yellow-400">{vuln.due_date}</span>
                </div>
              )}
            </div>
          ))}
          
          <div className="mt-4 pt-4 border-t border-gray-700">
            <button
              onClick={fetchVulnerabilities}
              disabled={loading}
              className="w-full bg-green-600 text-black px-4 py-2 rounded border border-green-400 hover:bg-green-500 transition-colors disabled:opacity-50 font-mono text-xs"
            >
              {loading ? "REFRESHING..." : "REFRESH THREAT DATA"}
            </button>
          </div>
        </div>
      )}
    </div>
  );

  const LockedView = () => (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center">
      <div className="text-center space-y-6">
        <h1 className="text-3xl font-bold">**LOCKED**</h1>
        <h2 className="text-xl font-semibold">Platform Setup Required</h2>
        
        <div className="bg-gray-800 rounded-lg p-4 max-w-md">
          <p className="text-sm text-gray-500 mb-4">
            Your secure database is encrypted and inaccessible. Complete the platform setup below, then insert your SanDisk USB device.
          </p>
          
          <div className="space-y-4 text-left">
            <div className="border-l-4 border-blue-500 pl-4">
              <h3 className="font-semibold text-blue-400">Windows Users</h3>
              <p className="text-sm text-gray-400 mt-1">
                1. Download <a href="https://github.com/pbatard/libwdi/releases" target="_blank" rel="noopener noreferrer" className="text-blue-300 underline">zadig-2.9.exe</a>
              </p>
              <p className="text-sm text-gray-400">
                2. Run it → Options → List All Devices → Select SanDisk USB → Replace Driver
              </p>
            </div>
            
            <div className="border-l-4 border-green-500 pl-4">
              <h3 className="font-semibold text-green-400">Linux Users</h3>
              <p className="text-sm text-gray-400 mt-1">
                if facing issues,manually create udev rule:
              </p>
              <p className="text-xs text-gray-500 font-mono bg-gray-900 p-2 rounded">
                SUBSYSTEM=="usb", ATTR"{"idVendor"}"=="0x0781", TAG+="uaccess"
              </p>
            </div>
            
            <div className="border-l-4 border-yellow-500 pl-4">
              <h3 className="font-semibold text-yellow-400">macOS Users</h3>
              <p className="text-sm text-gray-400 mt-1">
                No special setup required. Anchor can detect SanDisk USB devices automatically.
              </p>
              <p className="text-sm text-gray-400">
                If you experience issues, check Security & Privacy permissions.
              </p>
            </div>
          </div>
        </div>
        
        <p className="text-gray-400 text-lg font-bold"><strong>Done? Please Insert Your SanDisk USB</strong></p>
      </div>
    </div>
  );

  const UnlockedView = () => (
    <div className="min-h-screen bg-gray-100">
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <h1 className="text-2xl font-bold text-gray-900">Anchor Dashboard</h1>
            <div className="flex items-center space-x-2">
              <span className="text-sm text-gray-600">USB Connected</span>
              {usbSerial && (
                <span className="text-xs text-gray-500 bg-gray-200 px-2 py-1 rounded">
                  {usbSerial}
                </span>
              )}
            </div>
          </div>
        </div>
      </div>
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {error && (
          <div className="mb-6 bg-red-50 border border-red-200 rounded-lg p-4">
            <div className="text-sm text-red-800">{error}</div>
          </div>
        )}
        
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">Secure Database</h2>
            
            {!dbInitialized ? (
              <div>
                <p className="text-gray-600 mb-4">Initialize your encrypted SQLite database to begin storing secure data.</p>
                <button 
                  onClick={initializeDatabase}
                  disabled={loading}
                  className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition-colors disabled:opacity-50"
                >
                  {loading ? "Initializing..." : "Initialize Database"}
                </button>
              </div>
            ) : (
              <div>
                <div className="mb-4">
                  <div className="flex items-center space-x-2 mb-2">
                    <span className="text-sm text-green-600 font-medium">Database Active</span>
                  </div>
                  <p className="text-sm text-gray-500">Encrypted database ready for secure operations</p>
                </div>
                
                <div className="border-t pt-4">
                  <h3 className="text-md font-medium text-gray-900 mb-2">Add New Data</h3>
                  <div className="flex space-x-2">
                    <input
                      type="text"
                      value={newData}
                      onChange={(e) => setNewData(e.target.value)}
                      onKeyPress={handleKeyPress}
                      placeholder="Enter secure data..."
                      className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      autoFocus
                    />
                    <button
                      onClick={addSecureData}
                      disabled={loading || !newData.trim()}
                      className="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 transition-colors disabled:opacity-50"
                    >
                      {loading ? "Adding..." : "Add"}
                    </button>
                  </div>
                </div>
              </div>
            )}
          </div>
          
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">Stored Data ({secureData.length})</h2>
            
            {!dbInitialized ? (
              <p className="text-gray-500 text-sm">Initialize database to view stored data</p>
            ) : secureData.length === 0 ? (
              <p className="text-gray-500 text-sm">No data stored yet</p>
            ) : (
              <div className="space-y-2 max-h-64 overflow-y-auto">
                {secureData.map((item, index) => (
                  <div key={index} className="bg-gray-50 rounded p-3">
                    <div className="text-sm font-medium text-gray-900">{item.data}</div>
                  </div>
                ))}
              </div>
            )}
          </div>
          
          <SecurityDashboard />
        </div>
        
        <div className="mt-8 grid grid-cols-1 gap-6">
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-2">Session Status</h2>
            <p className="text-gray-600">Active session with hardware key</p>
            <div className="mt-4">
              <div className="text-sm text-green-600 font-medium">✓ Authenticated</div>
              <div className="text-sm text-gray-500 mt-1">
                Database: {dbInitialized ? "Initialized" : "Not Initialized"}
              </div>
            </div>
          </div>
        </div>
        
        <div className="mt-8 bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <div className="ml-3">
            <h3 className="text-sm font-medium text-yellow-800">Security Reminder</h3>
            <div className="mt-2 text-sm text-yellow-700">
              <p>Your session will remain active as long as the USB device is connected. Removing the USB will immediately lock the application and secure your data.</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );

  return isUnlocked ? <UnlockedView /> : <LockedView />;
}

export default App;