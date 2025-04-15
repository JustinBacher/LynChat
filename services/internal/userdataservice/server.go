package userdataservice

import (
	"context"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/your-repo/lyn-backend/pkg/config"
	"github.com/your-repo/lyn-backend/pkg/logging"
)

// Server represents the user data service
type Server struct {
	server *http.Server
	router *chi.Mux
	logger logging.Logger
	cfg    *config.Config
	db     *DB
}

// NewServer creates a new server instance
func NewServer(cfg *config.Config, logger logging.Logger) (*Server, error) {
	r := chi.NewRouter()

	// Middleware
	r.Use(middleware.RequestID)
	r.Use(middleware.RealIP)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Timeout(30 * time.Second))

	// Initialize database
	db, err := NewDB(cfg.DatabaseURL)
	if err != nil {
		return nil, err
	}

	server := &Server{
		server: &http.Server{
			Addr:    cfg.ServerAddress,
			Handler: r,
		},
		router: r,
		logger: logger,
		cfg:    cfg,
		db:     db,
	}

	// Register routes
	server.registerRoutes()

	return server, nil
}

// registerRoutes registers all the routes for the server
func (s *Server) registerRoutes() {
	// User routes
	s.router.Route("/api/users", func(r chi.Router) {
		r.Post("/", s.handleCreateUser)
		r.Get("/{id}", s.handleGetUser)
		r.Put("/{id}", s.handleUpdateUser)
		r.Delete("/{id}", s.handleDeleteUser)
		r.Post("/{id}/authenticate", s.handleAuthenticateUser)
	})

	// Conversation routes
	s.router.Route("/api/conversations", func(r chi.Router) {
		r.Post("/", s.handleCreateConversation)
		r.Get("/{id}", s.handleGetConversation)
		r.Put("/{id}", s.handleUpdateConversation)
		r.Delete("/{id}", s.handleDeleteConversation)
		r.Get("/user/{userId}", s.handleGetUserConversations)
	})

	// Settings routes
	s.router.Route("/api/settings", func(r chi.Router) {
		r.Get("/{userId}", s.handleGetSettings)
		r.Put("/{userId}", s.handleUpdateSettings)
		r.Post("/{userId}/reset", s.handleResetSettings)
	})

	// Audit routes
	s.router.Route("/api/audit", func(r chi.Router) {
		r.Get("/{userId}", s.handleGetAuditLog)
	})
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
