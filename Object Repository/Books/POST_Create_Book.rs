<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST_Create_Book</name>
   <tag></tag>
   <elementGuidId>d66d754b-2b06-44cf-bceb-853d2b3d7039</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;${Title}\&quot;,\n  \&quot;description\&quot;: \&quot;${Description}\&quot;,\n  \&quot;pageCount\&quot;: \&quot;${PageCount}\&quot;,\n  \&quot;excerpt\&quot;: \&quot;${Excerpt}\&quot;,\n  \&quot;publishDate\&quot;: \&quot;${PublishDate}\&quot;\n}&quot;,
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
      <webElementGuid>37750e84-59ff-4c54-8ef7-dca79f50d6b3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BaseUrl}/api/v1/Books</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Buku Hisana'</defaultValue>
      <description></description>
      <id>c1764d56-125f-407d-a327-5cbaaf8147b1</id>
      <masked>false</masked>
      <name>Title</name>
   </variables>
   <variables>
      <defaultValue>'Buku ayam'</defaultValue>
      <description></description>
      <id>5f6dbe87-1be0-42a8-80bf-85b70c878632</id>
      <masked>false</masked>
      <name>Description</name>
   </variables>
   <variables>
      <defaultValue>'500'</defaultValue>
      <description></description>
      <id>61a8d91a-469f-4bea-bd15-d5d21e812cc9</id>
      <masked>false</masked>
      <name>PageCount</name>
   </variables>
   <variables>
      <defaultValue>'HAaaaaa'</defaultValue>
      <description></description>
      <id>728d6e20-aed7-4676-be97-1f4be09b0056</id>
      <masked>false</masked>
      <name>Excerpt</name>
   </variables>
   <variables>
      <defaultValue>'2026-04-19T09:40:21.16Z'</defaultValue>
      <description></description>
      <id>b04149dd-289d-4f6b-ac9d-385ead95c7c3</id>
      <masked>false</masked>
      <name>PublishDate</name>
   </variables>
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

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
