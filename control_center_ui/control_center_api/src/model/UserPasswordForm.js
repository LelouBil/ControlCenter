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

import ApiClient from '../ApiClient';

/**
 * The UserPasswordForm model module.
 * @module model/UserPasswordForm
 * @version 0.1.0
 */
class UserPasswordForm {
    /**
     * Constructs a new <code>UserPasswordForm</code>.
     * @alias module:model/UserPasswordForm
     */
    constructor() { 
        
        UserPasswordForm.initialize(this);
    }

    /**
     * Initializes the fields of this object.
     * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
     * Only for internal use.
     */
    static initialize(obj) { 
    }

    /**
     * Constructs a <code>UserPasswordForm</code> from a plain JavaScript object, optionally creating a new instance.
     * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @param {module:model/UserPasswordForm} obj Optional instance to populate.
     * @return {module:model/UserPasswordForm} The populated <code>UserPasswordForm</code> instance.
     */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new UserPasswordForm();

            if (data.hasOwnProperty('password')) {
                obj['password'] = ApiClient.convertToType(data['password'], 'String');
            }
        }
        return obj;
    }


}

/**
 * @member {String} password
 */
UserPasswordForm.prototype['password'] = undefined;






export default UserPasswordForm;

