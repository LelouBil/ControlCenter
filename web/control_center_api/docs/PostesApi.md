# ControlCenterApi.PostesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**listPostes**](PostesApi.md#listPostes) | **GET** /postes/ | 



## listPostes

> [Poste] listPostes()



Liste les postes trouvés

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.PostesApi();
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

