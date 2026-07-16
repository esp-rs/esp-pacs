#[doc = "Register `REF_CONTROL` reader"]
pub type R = crate::R<REF_CONTROL_SPEC>;
#[doc = "Register `REF_CONTROL` writer"]
pub type W = crate::W<REF_CONTROL_SPEC>;
#[doc = "Field `RTC_REF_DELAY` reader - control the refgen short vdd"]
pub type RTC_REF_DELAY_R = crate::BitReader;
#[doc = "Field `RTC_REF_DELAY` writer - control the refgen short vdd"]
pub type RTC_REF_DELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_PRE_CHARGE` reader - control the refgen module pre charge"]
pub type RTC_PRE_CHARGE_R = crate::BitReader;
#[doc = "Field `RTC_PRE_CHARGE` writer - control the refgen module pre charge"]
pub type RTC_PRE_CHARGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_XPD_REFGEN` reader - control the refgen module power up"]
pub type RTC_XPD_REFGEN_R = crate::BitReader;
#[doc = "Field `RTC_XPD_REFGEN` writer - control the refgen module power up"]
pub type RTC_XPD_REFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - control the refgen short vdd"]
    #[inline(always)]
    pub fn rtc_ref_delay(&self) -> RTC_REF_DELAY_R {
        RTC_REF_DELAY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - control the refgen module pre charge"]
    #[inline(always)]
    pub fn rtc_pre_charge(&self) -> RTC_PRE_CHARGE_R {
        RTC_PRE_CHARGE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - control the refgen module power up"]
    #[inline(always)]
    pub fn rtc_xpd_refgen(&self) -> RTC_XPD_REFGEN_R {
        RTC_XPD_REFGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CONTROL")
            .field("rtc_ref_delay", &self.rtc_ref_delay())
            .field("rtc_pre_charge", &self.rtc_pre_charge())
            .field("rtc_xpd_refgen", &self.rtc_xpd_refgen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 29 - control the refgen short vdd"]
    #[inline(always)]
    pub fn rtc_ref_delay(&mut self) -> RTC_REF_DELAY_W<'_, REF_CONTROL_SPEC> {
        RTC_REF_DELAY_W::new(self, 29)
    }
    #[doc = "Bit 30 - control the refgen module pre charge"]
    #[inline(always)]
    pub fn rtc_pre_charge(&mut self) -> RTC_PRE_CHARGE_W<'_, REF_CONTROL_SPEC> {
        RTC_PRE_CHARGE_W::new(self, 30)
    }
    #[doc = "Bit 31 - control the refgen module power up"]
    #[inline(always)]
    pub fn rtc_xpd_refgen(&mut self) -> RTC_XPD_REFGEN_W<'_, REF_CONTROL_SPEC> {
        RTC_XPD_REFGEN_W::new(self, 31)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CONTROL_SPEC;
impl crate::RegisterSpec for REF_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_control::R`](R) reader structure"]
impl crate::Readable for REF_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_control::W`](W) writer structure"]
impl crate::Writable for REF_CONTROL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_CONTROL to value 0"]
impl crate::Resettable for REF_CONTROL_SPEC {}
