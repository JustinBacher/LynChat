package config

import (
	"fmt"
	"os"

	"github.com/spf13/viper"
)

// Config represents the application configuration
type Config struct {
	ServerAddress      string `env:"LYN_SERVER_ADDRESS" default:":8080"`
	DatabaseURL        string `env:"LYN_DATABASE_URL" required:"true"`
	JWTSecret          string `env:"LYN_JWT_SECRET" required:"true"`
	UserDataServiceURL string `env:"LYN_USER_DATA_SERVICE_URL" default:"http://userdataservice:8081"`
	OpenAIAPIKey       string `env:"LYN_OPENAI_API_KEY"`
	AnthropicAPIKey    string `env:"LYN_ANTHROPIC_API_KEY"`
}

// Load loads the configuration from the specified file
func Load(service string) (*Config, error) {
	v := viper.New()

	// Set default values
	v.SetDefault("server_address", "0.0.0.0:8080")
	v.SetDefault("database_url", "postgres://postgres:postgres@localhost:5432/lyn?sslmode=disable")
	v.SetDefault("jwt_secret", "supersecretkey")
	v.SetDefault("user_data_service_url", "http://userdataservice:8081")

	// Set the config file name and path
	v.SetConfigName(fmt.Sprintf("%s.config", service))
	v.SetConfigType("yaml")
	v.AddConfigPath(".")
	v.AddConfigPath("./config")
	v.AddConfigPath("./configs")
	v.AddConfigPath("$HOME/.lyn")
	v.AddConfigPath("/etc/lyn")

	// Environment variables override the config file
	v.AutomaticEnv()
	v.SetEnvPrefix("LYN")

	// Read the config file
	if err := v.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			return nil, fmt.Errorf("failed to read config file: %w", err)
		}
		fmt.Fprintf(os.Stderr, "Warning: No configuration file found\n")
	}

	// Parse the config
	var config Config
	if err := v.Unmarshal(&config); err != nil {
		return nil, fmt.Errorf("failed to unmarshal config: %w", err)
	}

	return &config, nil
}
