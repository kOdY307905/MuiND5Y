@echo off
:: Create the base directory for the project
:: mkdir ecom-microservices

:: Navigate to the project directory
:: cd ecom-microservices

:: Create main project files
:: echo. > .gitignore
:: echo. > README.md
:: echo. > LICENSE

:: Create Docker and Kubernetes folders and files
mkdir k8s
echo. > k8s\deployment.yaml
echo. > k8s\service.yaml
echo. > k8s\ingress.yaml

:: Create source folder and subdirectories for Java files
mkdir src
cd src
mkdir main
cd main
mkdir java
cd java
mkdir com
cd com
mkdir example
cd example
mkdir ecommerce
cd ecommerce
mkdir controller
mkdir service
mkdir repository
mkdir model
mkdir config
echo. > ecommerce\controller\DummyController.java
echo. > ecommerce\service\DummyService.java
echo. > ecommerce\repository\DummyRepository.java
echo. > ecommerce\model\DummyModel.java
echo. > ecommerce\config\DummyConfig.java

:: Create resources and application properties file
cd ..\..\..\..\..
mkdir resources
echo. > resources\application.properties
echo. > resources\bootstrap.yml

:: Create Dockerfile for the microservice
echo. > Dockerfile

:: Create test files and directories
mkdir test
cd test
mkdir java
cd java
mkdir com
cd com
mkdir example
cd example
mkdir ecommerce
cd ecommerce
mkdir controller
mkdir service
mkdir repository
mkdir model
echo. > ecommerce\controller\DummyControllerTest.java
echo. > ecommerce\service\DummyServiceTest.java
echo. > ecommerce\repository\DummyRepositoryTest.java
echo. > ecommerce\model\DummyModelTest.java

:: Create resources folder for tests
cd ..\..\..\..\..
mkdir resources

:: Create scripts folder for utility scripts
mkdir scripts
echo. > scripts\setup-db.sh

:: Create docs folder for documentation
mkdir docs
echo. > docs\architecture.md

:: Create Maven build file
echo. > pom.xml

echo Project setup is complete!
pause
