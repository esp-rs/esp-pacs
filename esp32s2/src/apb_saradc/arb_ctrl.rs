#[doc = "Register `ARB_CTRL` reader"]
pub type R = crate::R<ARB_CTRL_SPEC>;
#[doc = "Register `ARB_CTRL` writer"]
pub type W = crate::W<ARB_CTRL_SPEC>;
#[doc = "Field `APB_FORCE` reader - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
pub type APB_FORCE_R = crate::BitReader;
#[doc = "Field `APB_FORCE` writer - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
pub type APB_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_FORCE` reader - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
pub type RTC_FORCE_R = crate::BitReader;
#[doc = "Field `RTC_FORCE` writer - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
pub type RTC_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIFI_FORCE` reader - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
pub type WIFI_FORCE_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE` writer - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
pub type WIFI_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRANT_FORCE` reader - ADC2 arbiter force grant."]
pub type GRANT_FORCE_R = crate::BitReader;
#[doc = "Field `GRANT_FORCE` writer - ADC2 arbiter force grant."]
pub type GRANT_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_PRIORITY` reader - Set DIG ADC2 CTRL priority."]
pub type APB_PRIORITY_R = crate::FieldReader;
#[doc = "Field `APB_PRIORITY` writer - Set DIG ADC2 CTRL priority."]
pub type APB_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTC_PRIORITY` reader - Set RTC ADC2 CTRL priority."]
pub type RTC_PRIORITY_R = crate::FieldReader;
#[doc = "Field `RTC_PRIORITY` writer - Set RTC ADC2 CTRL priority."]
pub type RTC_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WIFI_PRIORITY` reader - Set PWDET/PKDET CTRL priority."]
pub type WIFI_PRIORITY_R = crate::FieldReader;
#[doc = "Field `WIFI_PRIORITY` writer - Set PWDET/PKDET CTRL priority."]
pub type WIFI_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIX_PRIORITY` reader - ADC2 arbiter uses fixed priority."]
pub type FIX_PRIORITY_R = crate::BitReader;
#[doc = "Field `FIX_PRIORITY` writer - ADC2 arbiter uses fixed priority."]
pub type FIX_PRIORITY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
    #[inline(always)]
    pub fn apb_force(&self) -> APB_FORCE_R {
        APB_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
    #[inline(always)]
    pub fn rtc_force(&self) -> RTC_FORCE_R {
        RTC_FORCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
    #[inline(always)]
    pub fn wifi_force(&self) -> WIFI_FORCE_R {
        WIFI_FORCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC2 arbiter force grant."]
    #[inline(always)]
    pub fn grant_force(&self) -> GRANT_FORCE_R {
        GRANT_FORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set DIG ADC2 CTRL priority."]
    #[inline(always)]
    pub fn apb_priority(&self) -> APB_PRIORITY_R {
        APB_PRIORITY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Set RTC ADC2 CTRL priority."]
    #[inline(always)]
    pub fn rtc_priority(&self) -> RTC_PRIORITY_R {
        RTC_PRIORITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Set PWDET/PKDET CTRL priority."]
    #[inline(always)]
    pub fn wifi_priority(&self) -> WIFI_PRIORITY_R {
        WIFI_PRIORITY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC2 arbiter uses fixed priority."]
    #[inline(always)]
    pub fn fix_priority(&self) -> FIX_PRIORITY_R {
        FIX_PRIORITY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB_CTRL")
            .field("apb_force", &format_args!("{}", self.apb_force().bit()))
            .field("rtc_force", &format_args!("{}", self.rtc_force().bit()))
            .field("wifi_force", &format_args!("{}", self.wifi_force().bit()))
            .field("grant_force", &format_args!("{}", self.grant_force().bit()))
            .field(
                "apb_priority",
                &format_args!("{}", self.apb_priority().bits()),
            )
            .field(
                "rtc_priority",
                &format_args!("{}", self.rtc_priority().bits()),
            )
            .field(
                "wifi_priority",
                &format_args!("{}", self.wifi_priority().bits()),
            )
            .field(
                "fix_priority",
                &format_args!("{}", self.fix_priority().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ARB_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 2 - ADC2 arbiter forces to enable DIG ADC2 CTRL."]
    #[inline(always)]
    #[must_use]
    pub fn apb_force(&mut self) -> APB_FORCE_W<ARB_CTRL_SPEC> {
        APB_FORCE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC2 arbiter forces to enable RTC ADC2 CTRL."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_force(&mut self) -> RTC_FORCE_W<ARB_CTRL_SPEC> {
        RTC_FORCE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC2 arbiter forces to enable PWDET/PKDET CTRL."]
    #[inline(always)]
    #[must_use]
    pub fn wifi_force(&mut self) -> WIFI_FORCE_W<ARB_CTRL_SPEC> {
        WIFI_FORCE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC2 arbiter force grant."]
    #[inline(always)]
    #[must_use]
    pub fn grant_force(&mut self) -> GRANT_FORCE_W<ARB_CTRL_SPEC> {
        GRANT_FORCE_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Set DIG ADC2 CTRL priority."]
    #[inline(always)]
    #[must_use]
    pub fn apb_priority(&mut self) -> APB_PRIORITY_W<ARB_CTRL_SPEC> {
        APB_PRIORITY_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Set RTC ADC2 CTRL priority."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_priority(&mut self) -> RTC_PRIORITY_W<ARB_CTRL_SPEC> {
        RTC_PRIORITY_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Set PWDET/PKDET CTRL priority."]
    #[inline(always)]
    #[must_use]
    pub fn wifi_priority(&mut self) -> WIFI_PRIORITY_W<ARB_CTRL_SPEC> {
        WIFI_PRIORITY_W::new(self, 10)
    }
    #[doc = "Bit 12 - ADC2 arbiter uses fixed priority."]
    #[inline(always)]
    #[must_use]
    pub fn fix_priority(&mut self) -> FIX_PRIORITY_W<ARB_CTRL_SPEC> {
        FIX_PRIORITY_W::new(self, 12)
    }
}
#[doc = "Configure the settings of DIG ADC2 arbiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_CTRL_SPEC;
impl crate::RegisterSpec for ARB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ctrl::R`](R) reader structure"]
impl crate::Readable for ARB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_ctrl::W`](W) writer structure"]
impl crate::Writable for ARB_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_CTRL to value 0x0900"]
impl crate::Resettable for ARB_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0900;
}
