# ControlCenterApi.HealthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthCheck**](HealthApi.md#healthCheck) | **GET** //alive | 



## healthCheck

> String healthCheck()



Verifie si l&#39;application est active

### Example

```javascript
import ControlCenterApi from 'control_center_api';

let apiInstance = new ControlCenterApi.HealthApi();
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

