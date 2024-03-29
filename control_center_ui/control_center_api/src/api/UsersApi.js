/**
 * control_center
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 *
 */


import ApiClient from "../ApiClient";
import LoginForm from '../model/LoginForm';
import User from '../model/User';
import UserPasswordForm from '../model/UserPasswordForm';

/**
* Users service.
* @module api/UsersApi
* @version 0.1.0
*/
export default class UsersApi {

    /**
    * Constructs a new UsersApi. 
    * @alias module:api/UsersApi
    * @class
    * @param {module:ApiClient} [apiClient] Optional API client implementation to use,
    * default to {@link module:ApiClient#instance} if unspecified.
    */
    constructor(apiClient) {
        this.apiClient = apiClient || ApiClient.instance;
    }



    /**
     * Change le mot de passe d'un utilisateur
     * @param {String} userName 
     * @param {module:model/UserPasswordForm} userPasswordForm 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with an object containing HTTP response
     */
    changePasswordWithHttpInfo(userName, userPasswordForm) {
      let postBody = userPasswordForm;
      // verify the required parameter 'userName' is set
      if (userName === undefined || userName === null) {
        throw new Error("Missing the required parameter 'userName' when calling changePassword");
      }
      // verify the required parameter 'userPasswordForm' is set
      if (userPasswordForm === undefined || userPasswordForm === null) {
        throw new Error("Missing the required parameter 'userPasswordForm' when calling changePassword");
      }

      let pathParams = {
        'user_name': userName
      };
      let queryParams = {
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = [];
      let contentTypes = ['application/json'];
      let accepts = [];
      let returnType = null;
      return this.apiClient.callApi(
        '/users/{user_name}', 'POST',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null
      );
    }

    /**
     * Change le mot de passe d'un utilisateur
     * @param {String} userName 
     * @param {module:model/UserPasswordForm} userPasswordForm 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}
     */
    changePassword(userName, userPasswordForm) {
      return this.changePasswordWithHttpInfo(userName, userPasswordForm)
        .then(function(response_and_data) {
          return response_and_data.data;
        });
    }


    /**
     * Créé un utilisateur
     * @param {module:model/LoginForm} loginForm 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with an object containing data of type {@link module:model/User} and HTTP response
     */
    createUserWithHttpInfo(loginForm) {
      let postBody = loginForm;
      // verify the required parameter 'loginForm' is set
      if (loginForm === undefined || loginForm === null) {
        throw new Error("Missing the required parameter 'loginForm' when calling createUser");
      }

      let pathParams = {
      };
      let queryParams = {
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = [];
      let contentTypes = ['application/json'];
      let accepts = ['application/json'];
      let returnType = User;
      return this.apiClient.callApi(
        '/users/', 'POST',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null
      );
    }

    /**
     * Créé un utilisateur
     * @param {module:model/LoginForm} loginForm 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with data of type {@link module:model/User}
     */
    createUser(loginForm) {
      return this.createUserWithHttpInfo(loginForm)
        .then(function(response_and_data) {
          return response_and_data.data;
        });
    }


    /**
     * Supprime un utilisateur
     * @param {String} userName 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with an object containing HTTP response
     */
    deleteUserWithHttpInfo(userName) {
      let postBody = null;
      // verify the required parameter 'userName' is set
      if (userName === undefined || userName === null) {
        throw new Error("Missing the required parameter 'userName' when calling deleteUser");
      }

      let pathParams = {
        'user_name': userName
      };
      let queryParams = {
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = [];
      let contentTypes = [];
      let accepts = [];
      let returnType = null;
      return this.apiClient.callApi(
        '/users/{user_name}', 'DELETE',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null
      );
    }

    /**
     * Supprime un utilisateur
     * @param {String} userName 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}
     */
    deleteUser(userName) {
      return this.deleteUserWithHttpInfo(userName)
        .then(function(response_and_data) {
          return response_and_data.data;
        });
    }


    /**
     * Récupere un utilisateur par son nom d'utilisateur
     * @param {String} userName 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with an object containing data of type {@link module:model/User} and HTTP response
     */
    getUserWithHttpInfo(userName) {
      let postBody = null;
      // verify the required parameter 'userName' is set
      if (userName === undefined || userName === null) {
        throw new Error("Missing the required parameter 'userName' when calling getUser");
      }

      let pathParams = {
        'user_name': userName
      };
      let queryParams = {
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = [];
      let contentTypes = [];
      let accepts = ['application/json'];
      let returnType = User;
      return this.apiClient.callApi(
        '/users/{user_name}', 'GET',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null
      );
    }

    /**
     * Récupere un utilisateur par son nom d'utilisateur
     * @param {String} userName 
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with data of type {@link module:model/User}
     */
    getUser(userName) {
      return this.getUserWithHttpInfo(userName)
        .then(function(response_and_data) {
          return response_and_data.data;
        });
    }


    /**
     * Liste les utilisateurs
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with an object containing data of type {@link Array.<module:model/User>} and HTTP response
     */
    listUsersWithHttpInfo() {
      let postBody = null;

      let pathParams = {
      };
      let queryParams = {
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = [];
      let contentTypes = [];
      let accepts = ['application/json'];
      let returnType = [User];
      return this.apiClient.callApi(
        '/users/', 'GET',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null
      );
    }

    /**
     * Liste les utilisateurs
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with data of type {@link Array.<module:model/User>}
     */
    listUsers() {
      return this.listUsersWithHttpInfo()
        .then(function(response_and_data) {
          return response_and_data.data;
        });
    }


}
