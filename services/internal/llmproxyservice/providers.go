package llmproxyservice

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

// LLMProvider defines the interface for LLM providers
type LLMProvider interface {
	Complete(ctx context.Context, message string, params map[string]any) (string, error)
	CompleteStream(ctx context.Context, message string, params map[string]any) (<-chan string, <-chan error)
}

// OpenAIProvider is an implementation of LLMProvider for OpenAI
type OpenAIProvider struct {
	apiKey     string
	anonymizer *RequestAnonymizer
	client     *http.Client
}

// NewOpenAIProvider creates a new OpenAIProvider instance
func NewOpenAIProvider(apiKey string, anonymizer *RequestAnonymizer) *OpenAIProvider {
	transport := &http.Transport{}

	// Set up the proxy if available
	if proxyURL, err := anonymizer.GetProxy(); err == nil && proxyURL != nil {
		transport.Proxy = http.ProxyURL(proxyURL)
	}

	client := &http.Client{
		Transport: transport,
		Timeout:   60 * time.Second,
	}

	return &OpenAIProvider{
		apiKey:     apiKey,
		anonymizer: anonymizer,
		client:     client,
	}
}

// Complete sends a completion request to OpenAI
func (p *OpenAIProvider) Complete(ctx context.Context, message string, params map[string]any) (string, error) {
	// Prepare the request
	reqBody := map[string]any{
		"model":    params["model_name"].(string),
		"messages": []map[string]string{{"role": "user", "content": message}},
	}

	// Add any additional parameters
	for k, v := range params {
		if k != "model_name" {
			reqBody[k] = v
		}
	}

	reqBytes, err := json.Marshal(reqBody)
	if err != nil {
		return "", fmt.Errorf("failed to marshal request: %w", err)
	}

	// Create the request
	req, err := http.NewRequestWithContext(ctx, "POST", "https://api.openai.com/v1/chat/completions", bytes.NewBuffer(reqBytes))
	if err != nil {
		return "", fmt.Errorf("failed to create request: %w", err)
	}

	// Set headers
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+p.apiKey)

	// Anonymize the request
	if err := p.anonymizer.AnonymizeRequest(req); err != nil {
		return "", fmt.Errorf("failed to anonymize request: %w", err)
	}

	// Send the request
	resp, err := p.client.Do(req)
	if err != nil {
		return "", fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	// Read the response
	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response: %w", err)
	}

	// Check for errors
	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("OpenAI API error: %s", respBody)
	}

	// Parse the response
	var openaiResp struct {
		Choices []struct {
			Message struct {
				Content string `json:"content"`
			} `json:"message"`
		} `json:"choices"`
	}

	if err := json.Unmarshal(respBody, &openaiResp); err != nil {
		return "", fmt.Errorf("failed to parse response: %w", err)
	}

	if len(openaiResp.Choices) == 0 {
		return "", fmt.Errorf("no choices in response")
	}

	return openaiResp.Choices[0].Message.Content, nil
}

// CompleteStream sends a streaming completion request to OpenAI
func (p *OpenAIProvider) CompleteStream(ctx context.Context, message string, params map[string]any) (<-chan string, <-chan error) {
	// Create channels for data and errors
	dataChan := make(chan string)
	errChan := make(chan error, 1)

	go func() {
		defer close(dataChan)
		defer close(errChan)

		// Prepare the request
		reqBody := map[string]any{
			"model":    params["model_name"].(string),
			"messages": []map[string]string{{"role": "user", "content": message}},
			"stream":   true,
		}

		// Add any additional parameters
		for k, v := range params {
			if k != "model_name" {
				reqBody[k] = v
			}
		}

		reqBytes, err := json.Marshal(reqBody)
		if err != nil {
			errChan <- fmt.Errorf("failed to marshal request: %w", err)
			return
		}

		// Create the request
		req, err := http.NewRequestWithContext(ctx, "POST", "https://api.openai.com/v1/chat/completions", bytes.NewBuffer(reqBytes))
		if err != nil {
			errChan <- fmt.Errorf("failed to create request: %w", err)
			return
		}

		// Set headers
		req.Header.Set("Content-Type", "application/json")
		req.Header.Set("Authorization", "Bearer "+p.apiKey)

		// Anonymize the request
		if err := p.anonymizer.AnonymizeRequest(req); err != nil {
			errChan <- fmt.Errorf("failed to anonymize request: %w", err)
			return
		}

		// Send the request
		resp, err := p.client.Do(req)
		if err != nil {
			errChan <- fmt.Errorf("failed to send request: %w", err)
			return
		}
		defer resp.Body.Close()

		// Check for errors
		if resp.StatusCode != http.StatusOK {
			respBody, _ := io.ReadAll(resp.Body)
			errChan <- fmt.Errorf("OpenAI API error: %s", respBody)
			return
		}

		// Process the streaming response
		// This is a simplified implementation that would need to be adapted for the
		// actual format of OpenAI's streaming responses
		decoder := json.NewDecoder(resp.Body)
		for {
			var chunk struct {
				Choices []struct {
					Delta struct {
						Content string `json:"content"`
					} `json:"delta"`
				} `json:"choices"`
			}

			if err := decoder.Decode(&chunk); err != nil {
				if err.Error() != "EOF" {
					errChan <- fmt.Errorf("failed to decode chunk: %w", err)
				}
				return
			}

			if len(chunk.Choices) > 0 && chunk.Choices[0].Delta.Content != "" {
				select {
				case dataChan <- chunk.Choices[0].Delta.Content:
				case <-ctx.Done():
					errChan <- ctx.Err()
					return
				}
			}
		}
	}()

	return dataChan, errChan
}

// AnthropicProvider is an implementation of LLMProvider for Anthropic
type AnthropicProvider struct {
	apiKey     string
	anonymizer *RequestAnonymizer
	client     *http.Client
}

// NewAnthropicProvider creates a new AnthropicProvider instance
func NewAnthropicProvider(apiKey string, anonymizer *RequestAnonymizer) *AnthropicProvider {
	transport := &http.Transport{}

	// Set up the proxy if available
	if proxyURL, err := anonymizer.GetProxy(); err == nil && proxyURL != nil {
		transport.Proxy = http.ProxyURL(proxyURL)
	}

	client := &http.Client{
		Transport: transport,
		Timeout:   60 * time.Second,
	}

	return &AnthropicProvider{
		apiKey:     apiKey,
		anonymizer: anonymizer,
		client:     client,
	}
}

// Complete sends a completion request to Anthropic
func (p *AnthropicProvider) Complete(ctx context.Context, message string, params map[string]any) (string, error) {
	// Prepare the request
	reqBody := map[string]any{
		"model":      params["model_name"].(string),
		"messages":   []map[string]string{{"role": "user", "content": message}},
		"max_tokens": 1000,
	}

	// Add any additional parameters
	for k, v := range params {
		if k != "model_name" {
			reqBody[k] = v
		}
	}

	reqBytes, err := json.Marshal(reqBody)
	if err != nil {
		return "", fmt.Errorf("failed to marshal request: %w", err)
	}

	// Create the request
	req, err := http.NewRequestWithContext(ctx, "POST", "https://api.anthropic.com/v1/messages", bytes.NewBuffer(reqBytes))
	if err != nil {
		return "", fmt.Errorf("failed to create request: %w", err)
	}

	// Set headers
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("x-api-key", p.apiKey)
	req.Header.Set("anthropic-version", "2023-06-01")

	// Anonymize the request
	if err := p.anonymizer.AnonymizeRequest(req); err != nil {
		return "", fmt.Errorf("failed to anonymize request: %w", err)
	}

	// Send the request
	resp, err := p.client.Do(req)
	if err != nil {
		return "", fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	// Read the response
	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response: %w", err)
	}

	// Check for errors
	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("anthropic API error: %s", respBody)
	}

	// Parse the response
	var anthropicResp struct {
		Content []struct {
			Text string `json:"text"`
		} `json:"content"`
	}

	if err := json.Unmarshal(respBody, &anthropicResp); err != nil {
		return "", fmt.Errorf("failed to parse response: %w", err)
	}

	if len(anthropicResp.Content) == 0 {
		return "", fmt.Errorf("no content in response")
	}

	return anthropicResp.Content[0].Text, nil
}

// CompleteStream sends a streaming completion request to Anthropic
func (p *AnthropicProvider) CompleteStream(ctx context.Context, message string, params map[string]any) (<-chan string, <-chan error) {
	// Create channels for data and errors
	dataChan := make(chan string)
	errChan := make(chan error, 1)

	go func() {
		defer close(dataChan)
		defer close(errChan)

		// Prepare the request
		reqBody := map[string]any{
			"model":      params["model_name"].(string),
			"messages":   []map[string]string{{"role": "user", "content": message}},
			"max_tokens": 1000,
			"stream":     true,
		}

		// Add any additional parameters
		for k, v := range params {
			if k != "model_name" {
				reqBody[k] = v
			}
		}

		reqBytes, err := json.Marshal(reqBody)
		if err != nil {
			errChan <- fmt.Errorf("failed to marshal request: %w", err)
			return
		}

		// Create the request
		req, err := http.NewRequestWithContext(ctx, "POST", "https://api.anthropic.com/v1/messages", bytes.NewBuffer(reqBytes))
		if err != nil {
			errChan <- fmt.Errorf("failed to create request: %w", err)
			return
		}

		// Set headers
		req.Header.Set("Content-Type", "application/json")
		req.Header.Set("x-api-key", p.apiKey)
		req.Header.Set("anthropic-version", "2023-06-01")

		// Anonymize the request
		if err := p.anonymizer.AnonymizeRequest(req); err != nil {
			errChan <- fmt.Errorf("failed to anonymize request: %w", err)
			return
		}

		// Send the request
		resp, err := p.client.Do(req)
		if err != nil {
			errChan <- fmt.Errorf("failed to send request: %w", err)
			return
		}
		defer resp.Body.Close()

		// Check for errors
		if resp.StatusCode != http.StatusOK {
			respBody, _ := io.ReadAll(resp.Body)
			errChan <- fmt.Errorf("anthropic API error: %s", respBody)
			return
		}

		// Process the streaming response
		// This is a simplified implementation that would need to be adapted for the
		// actual format of Anthropic's streaming responses
		decoder := json.NewDecoder(resp.Body)
		for {
			var chunk struct {
				Type    string `json:"type"`
				Content []struct {
					Text string `json:"text"`
				} `json:"content"`
			}

			if err := decoder.Decode(&chunk); err != nil {
				if err.Error() != "EOF" {
					errChan <- fmt.Errorf("failed to decode chunk: %w", err)
				}
				return
			}

			if chunk.Type == "content_block_delta" && len(chunk.Content) > 0 {
				select {
				case dataChan <- chunk.Content[0].Text:
				case <-ctx.Done():
					errChan <- ctx.Err()
					return
				}
			}
		}
	}()

	return dataChan, errChan
}
