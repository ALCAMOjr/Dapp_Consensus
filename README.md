VottingDapp is an application designed to facilitate democratic decision-making processes. Allows users to create proposals and vote on them. The app keeps track of the number of votes each proposal receives. Additionally, the application provides the functionality of deleting proposals. This could be a powerful tool for communities, organizations, or any group that values collective decision making.



# Deployment Guide for Project App Backend

Follow these steps to deploy your application:

1. **Start the Local Canister Execution Environment (Internet Computer)**
    Run the following command to start the local canister execution environment in the background and clean the state of your Internet Computer network:
    ```
    dfx start --clean --background
    ```

2. **Create a Canister for the Backend**
    Run the following command to create a new canister for your project's backend:
    ```
    dfx canister create proyect_app_backend
    ```

3. **Build the Backend**
    Run the following command to build your project's backend:
    ```
    dfx build proyect_app_backend
    ```

4. **Interact with the Canister**
    If you want to interact with your canister, you can use the `dfx canister` command followed by the name of your canister:
    ```
    dfx canister proyect_app_backend
    ```

5. **Deploy the Backend**
    Finally, run the following command to deploy your project's backend:
    ```
    dfx deploy proyect_app_backend
    ```

That's it! Your backend should now be deployed and running on your local Internet Computer network.

