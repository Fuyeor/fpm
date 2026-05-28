// src/types/organization.d.ts

/**
 * Request payload to check if a scope name is available
 */
export interface CheckScopeRequest {
  /** The lower-case scope name, e.g., 'fuyeor' */
  name: string;
}

/**
 * Response payload for scope name validation
 */
export interface ScopeValidationResponse {
  /** True if the name can be registered */
  available: boolean;
  /** Localization key, e.g., 'scope.available', 'scope.invalid.reserved' */
  message: string;
}

/**
 * Request payload to register a new Scope (Organization)
 */
export interface CreateScopeRequest {
  /** The desired scope name, e.g., 'webroamer' */
  name: string;
}

/**
 * Response payload after successfully creating a Scope
 */
export interface CreateScopeResponse {
  /** UUID v7 of the created organization */
  id: string;
  /** Confirmed scope name */
  name: string;
}
