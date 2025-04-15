package main

import (
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/your-repo/lyn-backend/internal/llmproxyservice"
	"github.com/your-repo/lyn-backend/pkg/config"
	"github.com/your-repo/lyn-backend/pkg/logging"
)

func main() {
	// Initialize logger
	logger := logging.NewLogger("llmproxyservice")
	logger.Info("Starting LLM Proxy Service")

	// Load configuration
	cfg, err := config.Load("llmproxyservice")
	if err != nil {
		logger.Fatal("Failed to load configuration", "error", err)
	}

	// Initialize server
	server, err := llmproxyservice.NewServer(cfg, logger)
	if err != nil {
		logger.Fatal("Failed to initialize server", "error", err)
	}

	// Start server in a goroutine so it doesn't block
	go func() {
		logger.Info("Server listening", "address", cfg.ServerAddress)
		if err := server.Start(); err != nil && err != http.ErrServerClosed {
			logger.Fatal("Server failed", "error", err)
		}
	}()

	// Wait for interrupt signal to gracefully shut down the server
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	logger.Info("Shutting down server...")
	if err := server.Shutdown(time.Second * 5); err != nil {
		logger.Error("Server forced to shutdown", "error", err)
	}

	logger.Info("Server exited properly")
}
