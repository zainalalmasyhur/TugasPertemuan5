<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_Update_Book</name>
   <tag></tag>
   <elementGuidId>eac0f75b-b83f-4db6-8931-081d946017cc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: 1,\n  \&quot;title\&quot;: \&quot;${Title}\&quot;,\n  \&quot;description\&quot;: \&quot;${Description}\&quot;,\n  \&quot;pageCount\&quot;: \&quot;${PageCount}\&quot;,\n  \&quot;excerpt\&quot;: \&quot;${Excerpt}\&quot;,\n  \&quot;publishDate\&quot;: \&quot;${PublishDate}\&quot;\n}&quot;,
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
      <webElementGuid>98fedbc7-524a-4466-bde8-7b7fb6c24ee4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.BaseUrl}/api/v1/Books/${BookId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>dc194c40-a113-46f1-a39a-32288b27012b</id>
      <masked>false</masked>
      <name>BookId</name>
   </variables>
   <variables>
      <defaultValue>'Buku ayam'</defaultValue>
      <description></description>
      <id>0520f5cf-eb61-4736-8db3-56f94903d128</id>
      <masked>false</masked>
      <name>Title</name>
   </variables>
   <variables>
      <defaultValue>'Buku Hisana'</defaultValue>
      <description></description>
      <id>b8a960ec-4e76-4f43-aadc-635187a6044f</id>
      <masked>false</masked>
      <name>Description</name>
   </variables>
   <variables>
      <defaultValue>'200'</defaultValue>
      <description></description>
      <id>d576d4d0-7b3a-4db8-bc07-daa4105ed10d</id>
      <masked>false</masked>
      <name>PageCount</name>
   </variables>
   <variables>
      <defaultValue>'Test123'</defaultValue>
      <description></description>
      <id>26f1a427-4088-4234-886c-1383137240b3</id>
      <masked>false</masked>
      <name>Excerpt</name>
   </variables>
   <variables>
      <defaultValue>'2026-04-19T09:40:21.16Z'</defaultValue>
      <description></description>
      <id>c0d3cae4-223e-46b6-abed-74eb92d4b43a</id>
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
