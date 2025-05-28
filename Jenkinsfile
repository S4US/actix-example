pipeline {
    agent any

    environment {
        PATH = "$HOME/.cargo/bin:$PATH"
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test'
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
