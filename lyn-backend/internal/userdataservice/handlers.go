package userdataservice

import (
	"encoding/json"
	"net/http"
	"strconv"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/your-repo/lyn-backend/pkg/api"
)

// Error response
type ErrorResponse struct {
	Error string `json:"error"`
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

// handleCreateUser handles creating a user
func (s *Server) handleCreateUser(w http.ResponseWriter, r *http.Request) {
	var user User
	if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := s.db.CreateUser(&user); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to create user")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    user.ID,
		Action:    "create",
		Resource:  "user",
		Details:   "User created",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusCreated, user)
}

// handleGetUser handles getting a user
func (s *Server) handleGetUser(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")
	user, err := s.db.GetUser(id)
	if err != nil {
		respondError(w, http.StatusNotFound, "User not found")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    id,
		Action:    "read",
		Resource:  "user",
		Details:   "User retrieved",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, user)
}

// handleUpdateUser handles updating a user
func (s *Server) handleUpdateUser(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")
	var user User
	if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	user.ID = id
	if err := s.db.UpdateUser(&user); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to update user")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    id,
		Action:    "update",
		Resource:  "user",
		Details:   "User updated",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, user)
}

// handleDeleteUser handles deleting a user
func (s *Server) handleDeleteUser(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")
	if err := s.db.DeleteUser(id); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to delete user")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    id,
		Action:    "delete",
		Resource:  "user",
		Details:   "User deleted",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, map[string]bool{"success": true})
}

// handleAuthenticateUser handles authenticating a user
func (s *Server) handleAuthenticateUser(w http.ResponseWriter, r *http.Request) {
	var req api.AuthRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(req); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	// Get the user from the database
	user, err := s.db.GetUserByUsername(req.Username)
	if err != nil {
		respondError(w, http.StatusUnauthorized, "Invalid credentials")
		return
	}

	// Check password
	if !auth.CheckPassword(req.Password, user.PasswordHash) {
		respondError(w, http.StatusUnauthorized, "Invalid credentials")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    user.ID,
		Action:    "authenticate",
		Resource:  "user",
		Details:   "User authenticated",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	// Clear sensitive information before returning
	user.PasswordHash = ""

	respondJSON(w, http.StatusOK, user)
}

// handleCreateConversation handles creating a conversation
func (s *Server) handleCreateConversation(w http.ResponseWriter, r *http.Request) {
	var conversation api.Conversation
	if err := json.NewDecoder(r.Body).Decode(&conversation); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(conversation); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	if err := s.db.CreateConversation(&conversation); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to create conversation")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    conversation.UserID,
		Action:    "create",
		Resource:  "conversation",
		Details:   "Conversation created",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusCreated, conversation)
}

// handleGetConversation handles getting a conversation
func (s *Server) handleGetConversation(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")
	conversation, err := s.db.GetConversation(id)
	if err != nil {
		respondError(w, http.StatusNotFound, "Conversation not found")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    conversation.UserID,
		Action:    "read",
		Resource:  "conversation",
		Details:   "Conversation retrieved",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, conversation)
}

// handleUpdateConversation handles updating a conversation
func (s *Server) handleUpdateConversation(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")
	var conversation api.Conversation
	if err := json.NewDecoder(r.Body).Decode(&conversation); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(conversation); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	conversation.ID = id
	if err := s.db.UpdateConversation(&conversation); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to update conversation")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    conversation.UserID,
		Action:    "update",
		Resource:  "conversation",
		Details:   "Conversation updated",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, conversation)
}

// handleDeleteConversation handles deleting a conversation
func (s *Server) handleDeleteConversation(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")

	// Get the conversation first to get the user ID for the audit entry
	conversation, err := s.db.GetConversation(id)
	if err != nil {
		respondError(w, http.StatusNotFound, "Conversation not found")
		return
	}

	if err := s.db.DeleteConversation(id); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to delete conversation")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    conversation.UserID,
		Action:    "delete",
		Resource:  "conversation",
		Details:   "Conversation deleted",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, map[string]bool{"success": true})
}

// handleGetUserConversations handles getting all conversations for a user
func (s *Server) handleGetUserConversations(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "userId")
	conversations, err := s.db.GetUserConversations(userID)
	if err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to get conversations")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    userID,
		Action:    "read",
		Resource:  "conversations",
		Details:   "User conversations retrieved",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, conversations)
}

// handleGetSettings handles getting settings for a user
func (s *Server) handleGetSettings(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "userId")
	settings, err := s.db.GetSettings(userID)
	if err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to get settings")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    userID,
		Action:    "read",
		Resource:  "settings",
		Details:   "User settings retrieved",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, settings)
}

// handleUpdateSettings handles updating settings for a user
func (s *Server) handleUpdateSettings(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "userId")
	var settings api.UserSettings
	if err := json.NewDecoder(r.Body).Decode(&settings); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if err := api.Validate(settings); err != nil {
		respondError(w, http.StatusBadRequest, err.Error())
		return
	}

	settings.UserID = userID
	if err := s.db.UpdateSettings(&settings); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to update settings")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    userID,
		Action:    "update",
		Resource:  "settings",
		Details:   "User settings updated",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	respondJSON(w, http.StatusOK, settings)
}

// handleResetSettings handles resetting settings for a user
func (s *Server) handleResetSettings(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "userId")
	if err := s.db.ResetSettings(userID); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to reset settings")
		return
	}

	// Add audit entry
	auditEntry := api.AuditEntry{
		UserID:    userID,
		Action:    "update",
		Resource:  "settings",
		Details:   "User settings reset to default",
		Timestamp: time.Now(),
	}
	if err := s.db.AddAuditEntry(&auditEntry); err != nil {
		s.logger.Error("Failed to add audit entry", "error", err)
	}

	settings, err := s.db.GetSettings(userID)
	if err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to get settings after reset")
		return
	}

	respondJSON(w, http.StatusOK, settings)
}

// handleGetAuditLog handles getting audit log entries for a user
func (s *Server) handleGetAuditLog(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "userId")

	// Get query parameters
	limitStr := r.URL.Query().Get("limit")
	offsetStr := r.URL.Query().Get("offset")

	limit := 50 // Default limit
	offset := 0 // Default offset

	if limitStr != "" {
		parsedLimit, err := strconv.Atoi(limitStr)
		if err == nil && parsedLimit > 0 {
			limit = parsedLimit
		}
	}

	if offsetStr != "" {
		parsedOffset, err := strconv.Atoi(offsetStr)
		if err == nil && parsedOffset >= 0 {
			offset = parsedOffset
		}
	}

	auditEntries, err := s.db.GetAuditLog(userID, limit, offset)
	if err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to get audit log")
		return
	}

	respondJSON(w, http.StatusOK, auditEntries)
}
