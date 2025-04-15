import { writable } from "svelte/store";

// Define the PII data type
export interface PIIData {
  type: string;
  value: any;
}

// Define the security alert type
export interface SecurityAlert {
  id: string;
  timestamp: Date;
  action: "masked" | "allowed";
  type: string;
}

// Define the security preferences type
export interface SecurityPreferences {
  detectEmails: boolean;
  detectPhoneNumbers: boolean;
  detectCreditCards: boolean;
  detectSSNs: boolean;
  detectAddresses: boolean;
  autoRedact: boolean;
}

// Initial security state
interface SecurityState {
  detectedPII: PIIData | null;
  redactedData: Record<string, any>; // Map of sensitive data to redacted forms
  securityAlerts: SecurityAlert[]; // History of security alerts
  securityPreferences: SecurityPreferences;
}

const initialState: SecurityState = {
  detectedPII: null,
  redactedData: {},
  securityAlerts: [],
  securityPreferences: {
    detectEmails: true,
    detectPhoneNumbers: true,
    detectCreditCards: true,
    detectSSNs: true,
    detectAddresses: true,
    autoRedact: false,
  },
};

// Create the security store
const createSecurityStore = () => {
  const { subscribe, update } = writable(initialState);

  return {
    subscribe,

    // Set detected PII
    setDetectedPII: (piiData: PIIData | null): void => {
      update((state) => ({
        ...state,
        detectedPII: piiData,
      }));
    },

    // Clear detected PII
    clearDetectedPII: (): void => {
      update((state) => ({
        ...state,
        detectedPII: null,
      }));
    },

    // Mask sensitive data (mark as private)
    maskSensitiveData: (piiId: string): void => {
      update((state) => {
        // In a real implementation, this would store the sensitive data
        // and its redacted form, potentially communicating with a backend
        const alert: SecurityAlert = {
          id: piiId,
          timestamp: new Date(),
          action: "masked",
          type: state.detectedPII?.type || "unknown",
        };

        return {
          ...state,
          detectedPII: null,
          securityAlerts: [alert, ...state.securityAlerts].slice(0, 50), // Keep last 50 alerts
        };
      });
    },

    // Allow sensitive data (mark as public)
    allowSensitiveData: (piiId: string): void => {
      update((state) => {
        const alert: SecurityAlert = {
          id: piiId,
          timestamp: new Date(),
          action: "allowed",
          type: state.detectedPII?.type || "unknown",
        };

        return {
          ...state,
          detectedPII: null,
          securityAlerts: [alert, ...state.securityAlerts].slice(0, 50), // Keep last 50 alerts
        };
      });
    },

    // Update security preferences
    updateSecurityPreference: <K extends keyof SecurityPreferences>(
      key: K,
      value: SecurityPreferences[K],
    ): void => {
      update((state) => ({
        ...state,
        securityPreferences: {
          ...state.securityPreferences,
          [key]: value,
        },
      }));
    },
  };
};

// Export the store
export const securityStore = createSecurityStore();
