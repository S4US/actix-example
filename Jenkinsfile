pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '-u root:root'
        }
    }
    
    stages {
        stage('Checkout') {
            steps {
                // Checkout code from SCM (e.g., Git)
                checkout scm
            }
        }
        
        stage('Build') {
            steps {
                // Build the Rust project in release mode
                sh 'cargo build --release'
            }
        }
        
        stage('Test') {
            steps {
                // Run tests
                sh 'cargo test'
            }
        }
        
        stage('Build Docker Image') {
            steps {
                // Create Docker image
                script {
                    def appName = 'actix-example'
                    def binaryName = 'actix-example' // Replace with your binary name from Cargo.toml
                    def dockerImage = docker.build("${appName}:latest")
                    
                    // Optional: Push to registry if needed
                    // docker.withRegistry('https://registry.example.com', 'registry-credentials-id') {
                    //     dockerImage.push()
                    // }
                }
            }
        }
    }
    
    post {
        always {
            // Clean up workspace
            cleanWs()
        }
    }
}