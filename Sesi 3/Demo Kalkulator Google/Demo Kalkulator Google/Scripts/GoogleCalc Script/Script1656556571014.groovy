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

// Untuk membuka browser
WebUI.openBrowser('')

// Untuk navigasi ke Google
WebUI.navigateToUrl('http://google.com')

// Untuk menunggu elemen ada (Opsional)
WebUI.waitForElementPresent(findTestObject('Page_Google/input_SearchBar'), 0)

// Untuk set text 2 + 2
WebUI.setText(findTestObject('Page_Google/input_SearchBar'), '2 + 2')

// Untuk klik tombol Search
WebUI.click(findTestObject('Page_Google/input_Search_inFlowdown'))

// Untuk cek elemen apakah sama dengan 4
WebUI.verifyElementText(findTestObject('Page_2  2 - Penelusuran Google/div_4 (function()var adocument.getElementBy_1579e9'),
	'4')

// Untuk menutup browser
WebUI.closeBrowser()