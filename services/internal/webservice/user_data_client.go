package webservice

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/your-repo/lyn-backend/pkg/api"
)

type UserDataClient struct {
	baseURL    string
	httpClient *http.Client
}

func NewUserDataClient(baseURL string) *UserDataClient {
	return &UserDataClient{
		baseURL:    baseURL,
		httpClient: &http.Client{},
	}
}

func (c *UserDataClient) AuthenticateUser(ctx context.Context, username, password string) (*api.UserResponse, error) {
	reqBody, err := json.Marshal(api.AuthRequest{
		Username: username,
		Password: password,
	})
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST",
		fmt.Sprintf("%s/api/users/authenticate", c.baseURL),
		bytes.NewBuffer(reqBody))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("authentication failed with status: %d", resp.StatusCode)
	}

	var userResp api.UserResponse
	if err := json.NewDecoder(resp.Body).Decode(&userResp); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &userResp, nil
}
