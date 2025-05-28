pipeline {
    agent any

    environment {
        IMAGE_NAME = "actix-example"
        PATH = "$HOME/.cargo/bin:$PATH"
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Install Rust') {
            steps {
                sh '''
                    if ! command -v rustc &> /dev/null; then
                        echo "Installing Rust..."
                        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    fi
                    export PATH="$HOME/.cargo/bin:$PATH"
                    source $HOME/.cargo/env || true

                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Build Rust Project') {
            steps {
                sh '''
                    export PATH="$HOME/.cargo/bin:$PATH"
                    source $HOME/.cargo/env || true

                    cargo build --release
                '''
            }
        }

        stage('Build Docker Image') {
            steps {
                sh "docker build -t ${IMAGE_NAME}:latest ."
                echo "✅ Built Docker image: ${IMAGE_NAME}:latest"
            }
        }
    }

    post {
        always {
            cleanWs()
        }
        success {
            echo "✅ Pipeline completed successfully!"
        }
        failure {
            echo "❌ Pipeline failed!"
        }
    }
}
