package llmproxyservice

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/your-repo/lyn-backend/pkg/api"
)

// handleLLMComplete handles a request to complete a message
func (s *Server) handleLLMComplete(c *gin.Context) {
	var req api.ChatRequest
	if err := c.BindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request body"})
		return
	}

	if err := api.Validate(req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Sanitize the message
	sanitizationResult := s.sanitizer.Sanitize(req.Message)

	// Get the appropriate provider
	provider, ok := s.providers[req.Provider]
	if !ok {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Unsupported provider"})
		return
	}

	// Send the request to the provider
	content, err := provider.Complete(c.Request.Context(), sanitizationResult.SanitizedContent, req.Params)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to complete message"})
		return
	}

	// Create the response
	requestID, err := s.anonymizer.GenerateRequestID()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to generate request ID"})
		return
	}

	resp := api.ChatResponse{
		ID:        requestID,
		Content:   content,
		Complete:  true,
		Timestamp: time.Now(),
	}

	c.JSON(http.StatusOK, resp)
}

// handleLLMCompleteStream handles a request to complete a message with streaming
func (s *Server) handleLLMCompleteStream(c *gin.Context) {
	var req api.ChatRequest
	if err := c.BindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request body"})
		return
	}

	if err := api.Validate(req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Sanitize the message
	sanitizationResult := s.sanitizer.Sanitize(req.Message)

	// Get the appropriate provider
	provider, ok := s.providers[req.Provider]
	if !ok {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Unsupported provider"})
		return
	}

	// Generate a request ID
	requestID, err := s.anonymizer.GenerateRequestID()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to generate request ID"})
		return
	}

	// Set up streaming response
	c.Header("Content-Type", "text/event-stream")
	c.Header("Cache-Control", "no-cache")
	c.Header("Connection", "keep-alive")
	c.Header("Transfer-Encoding", "chunked")

	// Create a channel that will notify when the client disconnects
	clientGone := c.Request.Context().Done()

	// Send the request to the provider
	dataChan, errChan := provider.CompleteStream(c.Request.Context(), sanitizationResult.SanitizedContent, req.Params)

	// Stream the response
	for {
		select {
		case <-clientGone:
			// Client disconnected
			return
		case chunk, ok := <-dataChan:
			if !ok {
				// Channel closed, end of stream
				c.SSEvent("end", gin.H{"id": requestID})
				return
			}
			c.SSEvent("chunk", gin.H{"id": requestID, "content": chunk})
			c.Writer.Flush()
		case err, ok := <-errChan:
			if !ok {
				// Channel closed, end of stream
				return
			}
			c.SSEvent("error", gin.H{"id": requestID, "error": err.Error()})
			c.Writer.Flush()
			return
		}
	}
}

// handleSanitize handles a request to sanitize a message
func (s *Server) handleSanitize(c *gin.Context) {
	var req struct {
		Message string `json:"message" binding:"required"`
	}
	if err := c.BindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid request body"})
		return
	}

	// Sanitize the message
	sanitizationResult := s.sanitizer.Sanitize(req.Message)

	c.JSON(http.StatusOK, sanitizationResult)
}
