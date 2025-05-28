pipeline {
    agent any

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Install Dependencies') {
            steps {
                script {
                    sh '''
                        # Install build dependencies (e.g., gcc, make)
                        if [ -f /etc/debian_version ]; then
                            echo "Detected Debian/Ubuntu-based system, installing dependencies..."
                            sudo apt-get update
                            sudo apt-get install -y gcc make
                        elif [ -f /etc/redhat-release ]; then
                            echo "Detected RedHat/CentOS-based system, installing dependencies..."
                            sudo yum install -y gcc make
                        else
                            echo "Unsupported OS, please ensure gcc and make are installed"
                            exit 1
                        fi
                    '''
                }
            }
        }

        stage('Install Rust') {
            steps {
                script {
                    sh '''
                        # Install Rust if not present
                        if ! command -v rustc &> /dev/null; then
                            echo "Installing Rust..."
                            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                        fi

                        # Ensure Rust is in PATH
                        export PATH="$HOME/.cargo/bin:$PATH"
                        source $HOME/.cargo/env 2>/dev/null || true

                        rustc --version
                        cargo --version
                    '''
                }
            }
        }

        stage('Build') {
            steps {
                sh '''
                    export PATH="$HOME/.cargo/bin:$PATH"
                    source $HOME/.cargo/env 2>/dev/null || true
                    cargo build --release
                '''
            }
        }

        stage('Test') {
            steps {
                sh '''
                    export PATH="$HOME/.cargo/bin:$PATH"
                    source $HOME/.cargo/env 2>/dev/null || true
                    cargo test
                '''
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    def appName = 'actix-example'
                    def dockerImage = docker.build("${appName}:latest")

                    // Tag with build number as well
                    sh "docker tag ${appName}:latest ${appName}:${BUILD_NUMBER}"

                    echo "Built Docker image: ${appName}:latest and ${appName}:${BUILD_NUMBER}"
                }
            }
        }
    }

    post {
        always {
            cleanWs()
        }
        success {
            echo "Pipeline completed successfully!"
        }
        failure {
            echo "Pipeline failed!"
        }
    }
}