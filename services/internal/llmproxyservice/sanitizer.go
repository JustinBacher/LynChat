package llmproxyservice

import (
	"regexp"
	"strings"

	"github.com/your-repo/lyn-backend/pkg/api"
)

// PIISanitizer sanitizes personally identifiable information
type PIISanitizer struct {
	patterns map[string]*regexp.Regexp
}

// NewPIISanitizer creates a new PIISanitizer instance
func NewPIISanitizer() (*PIISanitizer, error) {
	// Define regex patterns for PII detection
	patterns := map[string]*regexp.Regexp{
		"email":        regexp.MustCompile(`\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b`),
		"phone_number": regexp.MustCompile(`\b(\+\d{1,3}[ -]?)?\(?\d{3}\)?[ -]?\d{3}[ -]?\d{4}\b`),
		"credit_card":  regexp.MustCompile(`\b(?:\d{4}[ -]?){3}\d{4}\b`),
		"ssn":          regexp.MustCompile(`\b\d{3}[ -]?\d{2}[ -]?\d{4}\b`),
		"address":      regexp.MustCompile(`\b\d+\s+[A-Za-z\s]+(?:Avenue|Ave|Street|St|Road|Rd|Boulevard|Blvd|Drive|Dr|Lane|Ln|Way|Court|Ct|Place|Pl)\b`),
	}

	return &PIISanitizer{patterns: patterns}, nil
}

// Sanitize sanitizes a string by detecting and replacing PII
func (s *PIISanitizer) Sanitize(text string) api.SanitizationResult {
	detectedPII := []api.PII{}
	sanitizedContent := text

	for piiType, pattern := range s.patterns {
		matches := pattern.FindAllStringIndex(text, -1)

		// Process matches in reverse order to avoid index issues
		for i := len(matches) - 1; i >= 0; i-- {
			match := matches[i]
			startIndex := match[0]
			endIndex := match[1]

			value := text[startIndex:endIndex]

			// Create a PII entry
			pii := api.PII{
				Type:       piiType,
				Value:      value,
				StartIndex: startIndex,
				EndIndex:   endIndex,
			}

			detectedPII = append(detectedPII, pii)

			// Replace the PII with a placeholder
			replacement := "[" + strings.ToUpper(piiType) + "]"
			sanitizedContent = sanitizedContent[:startIndex] + replacement + sanitizedContent[endIndex:]
		}
	}

	return api.SanitizationResult{
		SanitizedContent: sanitizedContent,
		DetectedPII:      detectedPII,
	}
}
