<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Method Put untuk update post ke-45</description>
   <name>PUT Post 9 (ID 45)</name>
   <tag></tag>
   <elementGuidId>29fc417e-e332-4209-b129-549c71dcf75b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \t\&quot;id\&quot; : 45,\n\t\&quot;title\&quot; : \&quot;Haseul - Let Me in\&quot;,\n\t\&quot;body\&quot; : \&quot;I love this song\&quot;,\n\t\&quot;userId\&quot; : 11\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1600d796-883f-43bb-af69-267c55997384</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/posts/45</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'id', 45)
WS.verifyElementPropertyValue(response, 'title', 'Haseul - Let Me in')
WS.verifyElementPropertyValue(response, 'body', 'I love this song')
WS.verifyElementPropertyValue(response, 'userId', 11)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
