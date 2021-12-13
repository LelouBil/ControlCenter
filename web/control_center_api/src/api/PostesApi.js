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
import Poste from '../model/Poste';

/**
* Postes service.
* @module api/PostesApi
* @version 0.1.0
*/
export default class PostesApi {

    /**
    * Constructs a new PostesApi. 
    * @alias module:api/PostesApi
    * @class
    * @param {module:ApiClient} [apiClient] Optional API client implementation to use,
    * default to {@link module:ApiClient#instance} if unspecified.
    */
    constructor(apiClient) {
        this.apiClient = apiClient || ApiClient.instance;
    }



    /**
     * Liste les postes trouvés
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with an object containing data of type {@link Array.<module:model/Poste>} and HTTP response
     */
    listPostesWithHttpInfo() {
      let postBody = null;

      let pathParams = {
      };
      let queryParams = {
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = ['JWT'];
      let contentTypes = [];
      let accepts = ['application/json'];
      let returnType = [Poste];
      return this.apiClient.callApi(
        '/postes/', 'GET',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null
      );
    }

    /**
     * Liste les postes trouvés
     * @return {Promise} a {@link https://www.promisejs.org/|Promise}, with data of type {@link Array.<module:model/Poste>}
     */
    listPostes() {
      return this.listPostesWithHttpInfo()
        .then(function(response_and_data) {
          return response_and_data.data;
        });
    }


}
