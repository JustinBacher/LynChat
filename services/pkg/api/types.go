package api

import (
	"time"
)

// AuthRequest represents an authentication request
type AuthRequest struct {
	Username string `json:"username" validate:"required"`
	Password string `json:"password" validate:"required"`
}

// AuthResponse represents an authentication response
type AuthResponse struct {
	Token   string `json:"token"`
	Expires int64  `json:"expires"`
}

// ChatMessage represents a message in a chat
type ChatMessage struct {
	ID        string    `json:"id"`
	UserID    string    `json:"user_id"`
	Role      string    `json:"role" validate:"oneof=user assistant system"`
	Content   string    `json:"content" validate:"required"`
	Timestamp time.Time `json:"timestamp"`
}

// ChatRequest represents a request to the chat service
type ChatRequest struct {
	Message   string         `json:"message" validate:"required"`
	Provider  string         `json:"provider" validate:"required,oneof=local openai anthropic"`
	ModelName string         `json:"model_name"`
	Params    map[string]any `json:"params"`
}

// ChatResponse represents a response from the chat service
type ChatResponse struct {
	ID        string    `json:"id"`
	Content   string    `json:"content"`
	Complete  bool      `json:"complete"`
	Thoughts  string    `json:"thoughts,omitempty"`
	Tools     []ToolUse `json:"tools,omitempty"`
	Timestamp time.Time `json:"timestamp"`
}

// ToolUse represents a tool used by the assistant
type ToolUse struct {
	Name      string         `json:"name"`
	Arguments map[string]any `json:"arguments"`
	Result    map[string]any `json:"result,omitempty"`
}

// Conversation represents a conversation between a user and the assistant
type Conversation struct {
	ID        string        `json:"id"`
	UserID    string        `json:"user_id"`
	Title     string        `json:"title"`
	Messages  []ChatMessage `json:"messages"`
	CreatedAt time.Time     `json:"created_at"`
	UpdatedAt time.Time     `json:"updated_at"`
}

// UserSettings represents user settings
type UserSettings struct {
	UserID              string    `json:"user_id"`
	Theme               string    `json:"theme" validate:"oneof=light dark system"`
	FontSize            string    `json:"font_size" validate:"oneof=small medium large"`
	StoreConversations  bool      `json:"store_conversations"`
	StorePreferences    bool      `json:"store_preferences"`
	AllowAnonymizedData bool      `json:"allow_anonymized_data"`
	LLMProvider         string    `json:"llm_provider" validate:"oneof=local openai anthropic"`
	ModelName           string    `json:"model_name"`
	UpdatedAt           time.Time `json:"updated_at"`
}

// UserResponse represents the response from user-related endpoints
type UserResponse struct {
	ID        string    `json:"id"`
	Username  string    `json:"username"`
	Email     string    `json:"email"`
	Role      string    `json:"role"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

// PII represents personally identifiable information
type PII struct {
	Type       string `json:"type" validate:"oneof=email phone_number credit_card ssn address name"`
	Value      string `json:"value"`
	StartIndex int    `json:"start_index"`
	EndIndex   int    `json:"end_index"`
}

// SanitizationResult represents the result of sanitizing a message
type SanitizationResult struct {
	SanitizedContent string `json:"sanitized_content"`
	DetectedPII      []PII  `json:"detected_pii"`
}

// AuditEntry represents an entry in the audit log
type AuditEntry struct {
	ID        string    `json:"id"`
	UserID    string    `json:"user_id"`
	Action    string    `json:"action" validate:"required"`
	Resource  string    `json:"resource" validate:"required"`
	Details   string    `json:"details"`
	Timestamp time.Time `json:"timestamp"`
}
