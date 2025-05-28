pipeline {
    agent any
   
    stages {
        stage('Checkout') {
            steps {
                checkout scm
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