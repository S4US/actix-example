pipeline {
    agent any

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Build in Docker') {
            steps {
                script {
                    def appName = 'actix-example'
                    def buildNumber = env.BUILD_NUMBER

                    sh '''
                        docker run --rm -v $PWD:/usr/src/app -w /usr/src/app rust:latest \
                        bash -c "cargo build --release && cargo test"
                    '''
                }
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    def appName = 'actix-example'
                    def buildNumber = env.BUILD_NUMBER

                    def dockerImage = docker.build("${appName}:latest")
                    sh "docker tag ${appName}:latest ${appName}:${buildNumber}"
                    echo "Built Docker image: ${appName}:latest and ${appName}:${buildNumber}"
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
