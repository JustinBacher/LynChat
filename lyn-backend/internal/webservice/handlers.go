package webservice

import (
	"encoding/json"
	"net/http"
	"time"

	"github.com/gorilla/websocket"
	"github.com/your-repo/lyn-backend/pkg/api"
)

// Error response
type ErrorResponse struct {
	Error string `json:"error"`
}

// upgrader for WebSocket
var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true // In production, restrict this
	},
}

// respondJSON sends a JSON response
func respondJSON(w http.ResponseWriter, status int, payload interface{}) {
	response, err := json.Marshal(payload)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte(`{"error":"Internal Server Error"}`))
		return
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	w.Write(response)
}

// respondError sends an error response
func respondError(w http.ResponseWriter, status int, message string) {
	respondJSON(w, status, ErrorResponse{Error: message})
}

// handleLogin handles user login
func (s *Server) handleLogin(w http.ResponseWriter, r *http.Request) {
	var req api.AuthRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(req); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	// Call User Data Service to authenticate user
	userResp, err := s.userDataClient.AuthenticateUser(r.Context(), req.Username, req.Password)
	if err != nil {
		respondError(w, http.StatusUnauthorized, "Invalid credentials")
		return
	}

	// Generate JWT token
	token, err := s.jwtManager.Generate(userResp.ID, userResp.Role)
	if err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to generate token")
		return
	}

	// Create response
	resp := api.AuthResponse{
		Token:   token,
		Expires: time.Now().Add(24 * time.Hour).Unix(),
	}

	respondJSON(w, http.StatusOK, resp)
}

// handleRegister handles user registration
func (s *Server) handleRegister(w http.ResponseWriter, r *http.Request) {
	var req api.AuthRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(req); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	// TODO: Implement actual registration logic with the User Data Service
	// For now, return a dummy token
	resp := api.AuthResponse{
		Token:   "dummy_token",
		Expires: time.Now().Add(24 * time.Hour).Unix(),
	}

	respondJSON(w, http.StatusOK, resp)
}

// handleLogout handles user logout
func (s *Server) handleLogout(w http.ResponseWriter, r *http.Request) {
	// TODO: Implement actual logout logic
	respondJSON(w, http.StatusOK, map[string]bool{"success": true})
}

// handleGetCurrentUser handles getting the current user
func (s *Server) handleGetCurrentUser(w http.ResponseWriter, r *http.Request) {
	// TODO: Implement actual current user logic
	user := map[string]string{
		"id":    "1",
		"name":  "Test User",
		"email": "test@example.com",
	}

	respondJSON(w, http.StatusOK, user)
}

// handleGetChatHistory handles getting chat history
func (s *Server) handleGetChatHistory(w http.ResponseWriter, r *http.Request) {
	// TODO: Implement actual chat history logic with the User Data Service
	// For now, return dummy data
	history := []api.Conversation{
		{
			ID:        "1",
			UserID:    "1",
			Title:     "Test Conversation",
			CreatedAt: time.Now().Add(-24 * time.Hour),
			UpdatedAt: time.Now(),
			Messages: []api.ChatMessage{
				{
					ID:        "1",
					UserID:    "1",
					Role:      "user",
					Content:   "Hello, how are you?",
					Timestamp: time.Now().Add(-24 * time.Hour),
				},
				{
					ID:        "2",
					UserID:    "1",
					Role:      "assistant",
					Content:   "I'm doing well, thank you for asking!",
					Timestamp: time.Now().Add(-24 * time.Hour),
				},
			},
		},
	}

	respondJSON(w, http.StatusOK, history)
}

// handleChatRequest handles a chat request
func (s *Server) handleChatRequest(w http.ResponseWriter, r *http.Request) {
	var req api.ChatRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(req); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	// TODO: Implement actual chat logic with the LLM Proxy Service
	// For now, return dummy data
	resp := api.ChatResponse{
		ID:        "1",
		Content:   "This is a dummy response.",
		Complete:  true,
		Timestamp: time.Now(),
	}

	respondJSON(w, http.StatusOK, resp)
}

// handleClearChatHistory handles clearing chat history
func (s *Server) handleClearChatHistory(w http.ResponseWriter, r *http.Request) {
	// TODO: Implement actual clear chat history logic with the User Data Service
	respondJSON(w, http.StatusOK, map[string]bool{"success": true})
}

// handleGetSettings handles getting user settings
func (s *Server) handleGetSettings(w http.ResponseWriter, r *http.Request) {
	// TODO: Implement actual get settings logic with the User Data Service
	// For now, return dummy data
	settings := api.UserSettings{
		UserID:              "1",
		Theme:               "light",
		FontSize:            "medium",
		StoreConversations:  true,
		StorePreferences:    true,
		AllowAnonymizedData: false,
		LLMProvider:         "local",
		ModelName:           "llama3.2:1b",
		UpdatedAt:           time.Now(),
	}

	respondJSON(w, http.StatusOK, settings)
}

// handleUpdateSettings handles updating user settings
func (s *Server) handleUpdateSettings(w http.ResponseWriter, r *http.Request) {
	var req api.UserSettings
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(req); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	// TODO: Implement actual update settings logic with the User Data Service
	// For now, return success
	respondJSON(w, http.StatusOK, map[string]bool{"success": true})
}

// handleResetSettings handles resetting user settings
func (s *Server) handleResetSettings(w http.ResponseWriter, r *http.Request) {
	// TODO: Implement actual reset settings logic with the User Data Service
	// For now, return success
	respondJSON(w, http.StatusOK, map[string]bool{"success": true})
}

// handleWebSocketChat handles WebSocket connection for real-time chat
func (s *Server) handleWebSocketChat(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		s.logger.Error("Failed to upgrade to WebSocket", "error", err)
		return
	}
	defer conn.Close()

	// TODO: Implement WebSocket chat logic
	// For now, just echo messages back
	for {
		messageType, p, err := conn.ReadMessage()
		if err != nil {
			s.logger.Error("WebSocket read error", "error", err)
			return
		}
		if err := conn.WriteMessage(messageType, p); err != nil {
			s.logger.Error("WebSocket write error", "error", err)
			return
		}
	}
}
