# ControlCenterApi.DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthCheck**](DefaultApi.md#healthCheck) | **GET** //alive | 
[**listPostes**](DefaultApi.md#listPostes) | **GET** /postes/ | 
[**logIn**](DefaultApi.md#logIn) | **POST** /auth/login | 



## healthCheck

> String healthCheck()



Verifie si l&#39;application est active

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.DefaultApi();
apiInstance.healthCheck().then((data) => {
  console.log('API called successfully. Returned data: ' + data);
}, (error) => {
  console.error(error);
});

```

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain


## listPostes

> [Poste] listPostes()



Liste les postes trouvés

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.DefaultApi();
apiInstance.listPostes().then((data) => {
  console.log('API called successfully. Returned data: ' + data);
}, (error) => {
  console.error(error);
});

```

### Parameters

This endpoint does not need any parameter.

### Return type

[**[Poste]**](Poste.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## logIn

> String logIn(user)



Connexion au serveur en utilisant un login/mot de passe

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.DefaultApi();
let user = new ControlCenterApi.User(); // User | 
apiInstance.logIn(user).then((data) => {
  console.log('API called successfully. Returned data: ' + data);
}, (error) => {
  console.error(error);
});

```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user** | [**User**](User.md)|  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

