package logging

import (
	"os"

	"github.com/rs/zerolog"
)

// Logger is the interface for logging
type Logger interface {
	Debug(msg string, keysAndValues ...any)
	Info(msg string, keysAndValues ...any)
	Warn(msg string, keysAndValues ...any)
	Error(msg string, keysAndValues ...any)
	Fatal(msg string, keysAndValues ...any)
}

// ZerologLogger is an implementation of Logger using zerolog
type ZerologLogger struct {
	logger zerolog.Logger
}

// NewLogger creates a new logger instance
func NewLogger(serviceName string) Logger {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix

	logger := zerolog.New(os.Stdout).
		With().
		Timestamp().
		Str("service", serviceName).
		Logger()

	return &ZerologLogger{logger: logger}
}

// Debug logs a debug message
func (l *ZerologLogger) Debug(msg string, keysAndValues ...any) {
	l.logEvent(l.logger.Debug(), msg, keysAndValues...)
}

// Info logs an info message
func (l *ZerologLogger) Info(msg string, keysAndValues ...any) {
	l.logEvent(l.logger.Info(), msg, keysAndValues...)
}

// Warn logs a warning message
func (l *ZerologLogger) Warn(msg string, keysAndValues ...any) {
	l.logEvent(l.logger.Warn(), msg, keysAndValues...)
}

// Error logs an error message
func (l *ZerologLogger) Error(msg string, keysAndValues ...any) {
	l.logEvent(l.logger.Error(), msg, keysAndValues...)
}

// Fatal logs a fatal message and exits
func (l *ZerologLogger) Fatal(msg string, keysAndValues ...any) {
	l.logEvent(l.logger.Fatal(), msg, keysAndValues...)
}

// logEvent logs an event with the given level, message, and key-value pairs
func (l *ZerologLogger) logEvent(event *zerolog.Event, msg string, keysAndValues ...any) {
	for i := 0; i < len(keysAndValues); i += 2 {
		if i+1 < len(keysAndValues) {
			key, ok := keysAndValues[i].(string)
			if !ok {
				key = "?"
			}
			event = event.Interface(key, keysAndValues[i+1])
		}
	}
	event.Msg(msg)
}
