#[doc = "Register `ARB_CTRL` reader"]
pub type R = crate::R<ARB_CTRL_SPEC>;
#[doc = "Register `ARB_CTRL` writer"]
pub type W = crate::W<ARB_CTRL_SPEC>;
#[doc = "Field `ADC_ARB_APB_FORCE` reader - adc2 arbiter force to enableapb controller"]
pub type ADC_ARB_APB_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_APB_FORCE` writer - adc2 arbiter force to enableapb controller"]
pub type ADC_ARB_APB_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_ARB_RTC_FORCE` reader - adc2 arbiter force to enable rtc controller"]
pub type ADC_ARB_RTC_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_RTC_FORCE` writer - adc2 arbiter force to enable rtc controller"]
pub type ADC_ARB_RTC_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_ARB_WIFI_FORCE` reader - adc2 arbiter force to enable wifi controller"]
pub type ADC_ARB_WIFI_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_WIFI_FORCE` writer - adc2 arbiter force to enable wifi controller"]
pub type ADC_ARB_WIFI_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_ARB_GRANT_FORCE` reader - adc2 arbiter force grant"]
pub type ADC_ARB_GRANT_FORCE_R = crate::BitReader;
#[doc = "Field `ADC_ARB_GRANT_FORCE` writer - adc2 arbiter force grant"]
pub type ADC_ARB_GRANT_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_ARB_APB_PRIORITY` reader - Set adc2 arbiterapb priority"]
pub type ADC_ARB_APB_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ADC_ARB_APB_PRIORITY` writer - Set adc2 arbiterapb priority"]
pub type ADC_ARB_APB_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_ARB_RTC_PRIORITY` reader - Set adc2 arbiter rtc priority"]
pub type ADC_ARB_RTC_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ADC_ARB_RTC_PRIORITY` writer - Set adc2 arbiter rtc priority"]
pub type ADC_ARB_RTC_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` reader - Set adc2 arbiter wifi priority"]
pub type ADC_ARB_WIFI_PRIORITY_R = crate::FieldReader;
#[doc = "Field `ADC_ARB_WIFI_PRIORITY` writer - Set adc2 arbiter wifi priority"]
pub type ADC_ARB_WIFI_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_ARB_FIX_PRIORITY` reader - adc2 arbiter uses fixed priority"]
pub type ADC_ARB_FIX_PRIORITY_R = crate::BitReader;
#[doc = "Field `ADC_ARB_FIX_PRIORITY` writer - adc2 arbiter uses fixed priority"]
pub type ADC_ARB_FIX_PRIORITY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - adc2 arbiter force to enableapb controller"]
    #[inline(always)]
    pub fn adc_arb_apb_force(&self) -> ADC_ARB_APB_FORCE_R {
        ADC_ARB_APB_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - adc2 arbiter force to enable rtc controller"]
    #[inline(always)]
    pub fn adc_arb_rtc_force(&self) -> ADC_ARB_RTC_FORCE_R {
        ADC_ARB_RTC_FORCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - adc2 arbiter force to enable wifi controller"]
    #[inline(always)]
    pub fn adc_arb_wifi_force(&self) -> ADC_ARB_WIFI_FORCE_R {
        ADC_ARB_WIFI_FORCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - adc2 arbiter force grant"]
    #[inline(always)]
    pub fn adc_arb_grant_force(&self) -> ADC_ARB_GRANT_FORCE_R {
        ADC_ARB_GRANT_FORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set adc2 arbiterapb priority"]
    #[inline(always)]
    pub fn adc_arb_apb_priority(&self) -> ADC_ARB_APB_PRIORITY_R {
        ADC_ARB_APB_PRIORITY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Set adc2 arbiter rtc priority"]
    #[inline(always)]
    pub fn adc_arb_rtc_priority(&self) -> ADC_ARB_RTC_PRIORITY_R {
        ADC_ARB_RTC_PRIORITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Set adc2 arbiter wifi priority"]
    #[inline(always)]
    pub fn adc_arb_wifi_priority(&self) -> ADC_ARB_WIFI_PRIORITY_R {
        ADC_ARB_WIFI_PRIORITY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - adc2 arbiter uses fixed priority"]
    #[inline(always)]
    pub fn adc_arb_fix_priority(&self) -> ADC_ARB_FIX_PRIORITY_R {
        ADC_ARB_FIX_PRIORITY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB_CTRL")
            .field("adc_arb_apb_force", &self.adc_arb_apb_force())
            .field("adc_arb_rtc_force", &self.adc_arb_rtc_force())
            .field("adc_arb_wifi_force", &self.adc_arb_wifi_force())
            .field("adc_arb_grant_force", &self.adc_arb_grant_force())
            .field("adc_arb_apb_priority", &self.adc_arb_apb_priority())
            .field("adc_arb_rtc_priority", &self.adc_arb_rtc_priority())
            .field("adc_arb_wifi_priority", &self.adc_arb_wifi_priority())
            .field("adc_arb_fix_priority", &self.adc_arb_fix_priority())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - adc2 arbiter force to enableapb controller"]
    #[inline(always)]
    pub fn adc_arb_apb_force(&mut self) -> ADC_ARB_APB_FORCE_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_APB_FORCE_W::new(self, 2)
    }
    #[doc = "Bit 3 - adc2 arbiter force to enable rtc controller"]
    #[inline(always)]
    pub fn adc_arb_rtc_force(&mut self) -> ADC_ARB_RTC_FORCE_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_RTC_FORCE_W::new(self, 3)
    }
    #[doc = "Bit 4 - adc2 arbiter force to enable wifi controller"]
    #[inline(always)]
    pub fn adc_arb_wifi_force(&mut self) -> ADC_ARB_WIFI_FORCE_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_WIFI_FORCE_W::new(self, 4)
    }
    #[doc = "Bit 5 - adc2 arbiter force grant"]
    #[inline(always)]
    pub fn adc_arb_grant_force(&mut self) -> ADC_ARB_GRANT_FORCE_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_GRANT_FORCE_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Set adc2 arbiterapb priority"]
    #[inline(always)]
    pub fn adc_arb_apb_priority(&mut self) -> ADC_ARB_APB_PRIORITY_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_APB_PRIORITY_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Set adc2 arbiter rtc priority"]
    #[inline(always)]
    pub fn adc_arb_rtc_priority(&mut self) -> ADC_ARB_RTC_PRIORITY_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_RTC_PRIORITY_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Set adc2 arbiter wifi priority"]
    #[inline(always)]
    pub fn adc_arb_wifi_priority(&mut self) -> ADC_ARB_WIFI_PRIORITY_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_WIFI_PRIORITY_W::new(self, 10)
    }
    #[doc = "Bit 12 - adc2 arbiter uses fixed priority"]
    #[inline(always)]
    pub fn adc_arb_fix_priority(&mut self) -> ADC_ARB_FIX_PRIORITY_W<'_, ARB_CTRL_SPEC> {
        ADC_ARB_FIX_PRIORITY_W::new(self, 12)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_CTRL_SPEC;
impl crate::RegisterSpec for ARB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ctrl::R`](R) reader structure"]
impl crate::Readable for ARB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_ctrl::W`](W) writer structure"]
impl crate::Writable for ARB_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARB_CTRL to value 0x0900"]
impl crate::Resettable for ARB_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0900;
}
