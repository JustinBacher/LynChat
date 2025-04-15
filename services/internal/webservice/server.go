package webservice

import (
	"context"
	"net/http"
	"strings"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"github.com/your-repo/lyn-backend/pkg/config"
	"github.com/your-repo/lyn-backend/pkg/logging"
)

// Server represents the web service
type Server struct {
	server         *http.Server
	router         *chi.Mux
	logger         logging.Logger
	cfg            *config.Config
	jwtManager     *JWTManager
	userDataClient *UserDataClient
}

// NewServer creates a new server instance

func NewServer(cfg *config.Config, logger logging.Logger) (*Server, error) {
	r := chi.NewRouter()

	// Middleware
	r.Use(middleware.RequestID)
	r.Use(middleware.RealIP)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Timeout(60 * time.Second))

	// CORS configuration
	r.Use(cors.Handler(cors.Options{
		AllowedOrigins:   []string{"*"}, // In production, restrict this
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "X-CSRF-Token"},
		ExposedHeaders:   []string{"Link"},
		AllowCredentials: true,
		MaxAge:           300,
	}))

	// Initialize JWT manager
	jwtManager := NewJWTManager(cfg.JWTSecret)

	// Initialize user data client
	userDataClient := NewUserDataClient(cfg.UserDataServiceURL)

	server := &Server{
		server: &http.Server{
			Addr:    cfg.ServerAddress,
			Handler: r,
		},
		router:         r,
		logger:         logger,
		cfg:            cfg,
		jwtManager:     jwtManager,
		userDataClient: userDataClient,
	}

	// Register routes
	server.registerRoutes()

	return server, nil
}

// registerRoutes registers all the routes for the server
func (s *Server) registerRoutes() {
	// Register authentication routes
	s.router.Route("/api/auth", func(r chi.Router) {
		r.Post("/login", s.handleLogin)
		r.Post("/register", s.handleRegister)
		r.Post("/logout", s.handleLogout)
		r.Get("/me", s.withAuth(s.handleGetCurrentUser))
	})

	// Register chat routes
	s.router.Route("/api/chat", func(r chi.Router) {
		r.Get("/history", s.withAuth(s.handleGetChatHistory))
		r.Post("/", s.withAuth(s.handleChatRequest))
		r.Delete("/history", s.withAuth(s.handleClearChatHistory))
	})

	// Register settings routes
	s.router.Route("/api/settings", func(r chi.Router) {
		r.Get("/", s.withAuth(s.handleGetSettings))
		r.Put("/", s.withAuth(s.handleUpdateSettings))
		r.Post("/reset", s.withAuth(s.handleResetSettings))
	})

	// Register WebSocket endpoint
	s.router.Get("/ws/chat", s.handleWebSocketChat)

	// Serve static files
	s.router.Handle("/*", http.FileServer(http.Dir("./static")))
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

// withAuth is a middleware that checks if the user is authenticated
func (s *Server) withAuth(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if authHeader == "" {
			respondError(w, http.StatusUnauthorized, "Authorization header is required")
			return
		}

		// Extract the token from the Authorization header
		// Format: "Bearer {token}"
		parts := strings.Split(authHeader, " ")
		if len(parts) != 2 || parts[0] != "Bearer" {
			respondError(w, http.StatusUnauthorized, "Invalid authorization header format")
			return
		}

		// Validate the token
		claims, err := s.jwtManager.Validate(parts[1])
		if err != nil {
			respondError(w, http.StatusUnauthorized, err.Error())
			return
		}

		// Add user ID to the request context
		ctx := context.WithValue(r.Context(), "user_id", claims.UserID)
		next(w, r.WithContext(ctx))
	}
}
