<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Method Post untuk buat post kesepuluh</description>
   <name>POST Post 10</name>
   <tag></tag>
   <elementGuidId>27644462-4414-4699-9606-a6a03d580174</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;title\&quot; : \&quot;Yeojin - Kiss Later\&quot;,\n\t\&quot;body\&quot; : \&quot;Yeojin is the fourth member of LOONA\&quot;,\n\t\&quot;userId\&quot; : 11\n}&quot;,
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
      <webElementGuid>f609525b-6eca-4521-aeb2-42af9502a4fb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/posts</restUrl>
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

WS.verifyResponseStatusCode(response, 201)
assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'title', 'Yeojin - Kiss Later')
WS.verifyElementPropertyValue(response, 'body', 'Yeojin is the fourth member of LOONA')
WS.verifyElementPropertyValue(response, 'userId', 11)
WS.verifyElementPropertyValue(response, 'id', 101)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
