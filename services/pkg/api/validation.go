package api

import (
	"github.com/go-playground/validator/v10"
)

var validate *validator.Validate

func init() {
	validate = validator.New()
}

// Validate validates a struct based on its validation tags
func Validate(s any) error {
	return validate.Struct(s)
}
