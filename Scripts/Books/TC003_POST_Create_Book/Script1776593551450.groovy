import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

def response = WS.sendRequest(findTestObject('Books/POST_Create_Book'))

//Verify Status Code
WS.verifyResponseStatusCode(response, 200, FailureHandling.STOP_ON_FAILURE)

//Verify Book Title
WS.verifyElementPropertyValue(response, 'title', BookTitle, FailureHandling.STOP_ON_FAILURE)

//Verify Book description
WS.verifyElementPropertyValue(response, 'description', BookDesc, FailureHandling.STOP_ON_FAILURE)

//Verify Book pageCount
WS.verifyElementPropertyValue(response, 'pageCount', BookPage, FailureHandling.STOP_ON_FAILURE)

//Verify Book excerpt
WS.verifyElementPropertyValue(response, 'excerpt', BookExcerpt, FailureHandling.STOP_ON_FAILURE)

//Verify Book publishDate
WS.verifyElementPropertyValue(response, 'publishDate', BookPublishDate, FailureHandling.STOP_ON_FAILURE)



