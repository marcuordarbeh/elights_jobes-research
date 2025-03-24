// main.go
// Optional orchestrator (API gateway) written in Go using Gin.
// This file demonstrates exposing endpoints that mirror your Rust backend.
// In a production system, you might have more robust error handling and configuration.

package main

import (
	"log"

	"github.com/gin-gonic/gin"
	
	// Replace these with your actual package paths
	"your_project/internal/config"
	"your_project/internal/controllers"
	"your_project/internal/middlewares"
)

func main() {
	// Initialize Gin router
	r := gin.Default()

	// Setup database connection using your preferred method.
	// config.SetupDatabase should return a database handle (e.g., *sql.DB)
	db := config.SetupDatabase()
	
	// Initialize JWT middleware for route protection.
	jwtMiddleware := middlewares.JWTMiddleware()

	// Create controllers (ensure that your NewUserController and NewPaymentController
	// are implemented in the respective packages).
	userController := controllers.NewUserController(db)
	paymentController := controllers.NewPaymentController(db)

	// Public endpoints for registration and login.
	r.POST("/register", userController.Register)
	r.POST("/login", userController.Login)

	// Protected endpoints – these routes require a valid JWT token.
	authorized := r.Group("/")
	authorized.Use(jwtMiddleware)
	{
		authorized.POST("/process_card", paymentController.ProcessCard)
		authorized.POST("/generate_ach", paymentController.GenerateACH)
		authorized.POST("/receive_bank_transfer", paymentController.ReceiveBankTransfer)
		authorized.POST("/convert_to_crypto", paymentController.ConvertToCrypto)
	}

	// Start the server on port 8080.
	log.Println("API gateway running on port 8080...")
	log.Fatal(r.Run(":8080"))
}
