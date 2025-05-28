pipeline {
    // Use a Docker agent with Rust and build dependencies
    agent {
        docker {
            image 'rust:latest'
            args '-u root' // Run as root to avoid permission issues
        }
    }

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
                        # Install build dependencies (gcc, make, curl) if not already present
                        apt-get update
                        apt-get install -y gcc make curl
                    '''
                }
            }
        }

        stage('Install Rust') {
            steps {
                script {
                    sh '''
                        # Rust is already installed in rust:latest, but verify
                        rustc --version
                        cargo --version
                    '''
                }
            }
        }

        stage('Build') {
            steps {
                sh '''
                    cargo build --release
                '''
            }
        }

        stage('Test') {
            steps {
                sh '''
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