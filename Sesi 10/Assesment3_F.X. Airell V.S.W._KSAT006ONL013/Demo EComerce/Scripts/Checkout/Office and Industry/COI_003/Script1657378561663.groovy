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

Mobile.startApplication('C:\\Users\\095068\\Documents\\Automation Testing with Katalon Studio\\Sesi 10\\Assesment3_F.X. Airell V.S.W._KSAT006ONL013\\Demo EComerce\\APK\\ECommerce -SAMPLE-Android.apk', 
    true)

Mobile.waitForElementPresent(findTestObject('Home/btn_Category'), 0)

Mobile.delay(10)

Mobile.tap(findTestObject('Home/btn_Category'), 0)

Mobile.delay(5)

Mobile.tap(findTestObject('Category/btn_Office and Industry'), 0)

Mobile.delay(5)

Mobile.tap(findTestObject('Office and Industry/btn_Guoache'), 0)

Mobile.delay(5)

Mobile.tap(findTestObject('Checkout/btn_Add To Cart'), 0)

Mobile.delay(5)

Mobile.setText(findTestObject('Checkout/txt_Number of Order'), '1', 0)

Mobile.delay(5)

Mobile.tap(findTestObject('Checkout/btn_Add Order'), 0)

Mobile.delay(5)

Mobile.tap(findTestObject('Checkout/btn_Cart'), 0)

Mobile.delay(10)

Mobile.tap(findTestObject('Checkout/btn_Checkout'), 0)

Mobile.delay(10)

Mobile.tap(findTestObject('Checkout/btn_Process Checkout (Buy)'), 0)

Mobile.delay(5)

Mobile.tap(findTestObject('Checkout/btn_Yes (Buy)'), 0)

Mobile.delay(15)

Mobile.tap(findTestObject('Checkout/btn_OK'), 0)

Mobile.delay(5)

Mobile.closeApplication()

