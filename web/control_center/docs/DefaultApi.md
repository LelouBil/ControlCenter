# ControlCenter.DefaultApi

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
import ControlCenter from 'control_center';

let apiInstance = new ControlCenter.DefaultApi();
apiInstance.healthCheck((error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
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
import ControlCenter from 'control_center';

let apiInstance = new ControlCenter.DefaultApi();
apiInstance.listPostes((error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
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
import ControlCenter from 'control_center';

let apiInstance = new ControlCenter.DefaultApi();
let user = new ControlCenter.User(); // User | 
apiInstance.logIn(user, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
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

