# Serverless Rust Microservice
[![pipeline status](https://gitlab.com/dukeaiml/IDS721/rg361_wk02/badges/main/pipeline.svg)](https://gitlab.com/dukeaiml/IDS721/rg361_wk02/-/commits/main)

## Overview

This simple AWS Lambda funtion takes in 2 input parameters and returns the first value raised to the second value.  
The fuctions uses AWS API Gateway as the trigger and returns the result in JSON format.

The function is programmed in ``Rust`` and is deployed using ``Cargo Lambda`` and ``AWS CLI``.

![AWS Lambda Function](./resources/console.png)

## Sample Execution

The function is tested in the AWS Lambda console with the following input parameters:

![AWS Lambda Function](./resources/test_input.png)

The funtion returns the output as expected:

![AWS Lambda Function](./resources/test_response.png)

## Development

1. Install [Cargo Lambda](https://www.cargo-lambda.info/) and [AWS CLI](https://aws.amazon.com/cli/) following the instructions in the respective links.
2. Setup the AWS CLI with the required credentials
3. Create the project repository and clone the repository to the local machine
4. navigate to the project repository and run the following command to create a new project:
    ```bash
    cargo lambda new <project_name>
    ```
5. Navigate to the newly created folder and develop the rust function as required
6. Test the function locally using the following commands (check the [Cargo Lambda](https://www.cargo-lambda.info/) documentation for more details):
    ```bash
    cargo lambda watch
    ```
    ```bash
    cargo lambda invoke
    ```
7. Once the function is tested and ready, deploy the function to AWS Lambda using the following command:
    ```bash
    cargo lambda build
    ```
    ```bash
    cargo lambda deploy
    ```
    ![AWS Lambda Function](./resources/deploy.png)

8. The function is now deployed and can be tested in the AWS Lambda console as shown in the [Sample Execution](#sample-execution) section.
9. Add the API Gateway trigger to the function as per the requirement.
