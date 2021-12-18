# ControlCenterApi.UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**changePassword**](UsersApi.md#changePassword) | **POST** /users/{user_name} | 
[**createUser**](UsersApi.md#createUser) | **POST** /users/ | 
[**deleteUser**](UsersApi.md#deleteUser) | **DELETE** /users/{user_name} | 
[**getUser**](UsersApi.md#getUser) | **GET** /users/{user_name} | 
[**listUsers**](UsersApi.md#listUsers) | **GET** /users/ | 



## changePassword

> changePassword(userName, userPasswordForm)



Change le mot de passe d&#39;un utilisateur

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.UsersApi();
let userName = "userName_example"; // String | 
let userPasswordForm = new ControlCenterApi.UserPasswordForm(); // UserPasswordForm | 
apiInstance.changePassword(userName, userPasswordForm).then(() => {
  console.log('API called successfully.');
}, (error) => {
  console.error(error);
});

```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userName** | **String**|  | 
 **userPasswordForm** | [**UserPasswordForm**](UserPasswordForm.md)|  | 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined


## createUser

> User createUser(loginForm)



Créé un utilisateur

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.UsersApi();
let loginForm = new ControlCenterApi.LoginForm(); // LoginForm | 
apiInstance.createUser(loginForm).then((data) => {
  console.log('API called successfully. Returned data: ' + data);
}, (error) => {
  console.error(error);
});

```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **loginForm** | [**LoginForm**](LoginForm.md)|  | 

### Return type

[**User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## deleteUser

> deleteUser(userName)



Supprime un utilisateur

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.UsersApi();
let userName = "userName_example"; // String | 
apiInstance.deleteUser(userName).then(() => {
  console.log('API called successfully.');
}, (error) => {
  console.error(error);
});

```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userName** | **String**|  | 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined


## getUser

> User getUser(userName)



Récupere un utilisateur par son nom d&#39;utilisateur

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.UsersApi();
let userName = "userName_example"; // String | 
apiInstance.getUser(userName).then((data) => {
  console.log('API called successfully. Returned data: ' + data);
}, (error) => {
  console.error(error);
});

```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userName** | **String**|  | 

### Return type

[**User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## listUsers

> [User] listUsers()



Liste les utilisateurs

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.UsersApi();
apiInstance.listUsers().then((data) => {
  console.log('API called successfully. Returned data: ' + data);
}, (error) => {
  console.error(error);
});

```

### Parameters

This endpoint does not need any parameter.

### Return type

[**[User]**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

