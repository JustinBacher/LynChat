package llmproxyservice

import (
	"context"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/your-repo/lyn-backend/pkg/config"
	"github.com/your-repo/lyn-backend/pkg/logging"
)

// Server represents the LLM proxy service
type Server struct {
	server     *http.Server
	router     *gin.Engine
	logger     logging.Logger
	cfg        *config.Config
	sanitizer  *PIISanitizer
	providers  map[string]LLMProvider
	anonymizer *RequestAnonymizer
}

// NewServer creates a new server instance
func NewServer(cfg *config.Config, logger logging.Logger) (*Server, error) {
	router := gin.New()

	// Middleware
	router.Use(gin.Recovery())
	router.Use(gin.Logger())

	// Initialize sanitizer
	sanitizer, err := NewPIISanitizer()
	if err != nil {
		return nil, err
	}

	// Initialize request anonymizer
	anonymizer, err := NewRequestAnonymizer(cfg)
	if err != nil {
		return nil, err
	}

	// Initialize providers
	providers := map[string]LLMProvider{
		"openai":    NewOpenAIProvider(cfg.OpenAIAPIKey, anonymizer),
		"anthropic": NewAnthropicProvider(cfg.AnthropicAPIKey, anonymizer),
		// Add more providers as needed
	}

	server := &Server{
		server: &http.Server{
			Addr:    cfg.ServerAddress,
			Handler: router,
		},
		router:     router,
		logger:     logger,
		cfg:        cfg,
		sanitizer:  sanitizer,
		providers:  providers,
		anonymizer: anonymizer,
	}

	// Register routes
	server.registerRoutes()

	return server, nil
}

// registerRoutes registers all the routes for the server
func (s *Server) registerRoutes() {
	// LLM proxy routes
	s.router.POST("/api/llm/complete", s.handleLLMComplete)
	s.router.POST("/api/llm/complete/stream", s.handleLLMCompleteStream)
	s.router.POST("/api/llm/sanitize", s.handleSanitize)
}

// Start starts the server
func (s *Server) Start() error {
	return s.server.ListenAndServe()
}

// Shutdown gracefully shuts down the server
func (s *Server) Shutdown(timeout time.Duration) error {
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()
	return s.server.Shutdown(ctx)
}
