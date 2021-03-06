# ControlCenterApi.LoginApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logIn**](LoginApi.md#logIn) | **POST** /auth/login | 



## logIn

> String logIn(loginForm)



Connexion au serveur en utilisant un login/mot de passe

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.LoginApi();
let loginForm = new ControlCenterApi.LoginForm(); // LoginForm | 
apiInstance.logIn(loginForm).then((data) => {
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

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

