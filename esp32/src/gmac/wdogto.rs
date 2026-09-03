#[doc = "Register `WDOGTO` reader"]
pub type R = crate::R<WDOGTO_SPEC>;
#[doc = "Register `WDOGTO` writer"]
pub type W = crate::W<WDOGTO_SPEC>;
#[doc = "Field `WDOGTO` reader - When Bit\\[16\\] (PWE) is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field such frame is terminated and declared as an error frame."]
pub type WDOGTO_R = crate::FieldReader<u16>;
#[doc = "Field `WDOGTO` writer - When Bit\\[16\\] (PWE) is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field such frame is terminated and declared as an error frame."]
pub type WDOGTO_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PWDOGEN` reader - When this bit is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared the watchdog timeout for a received frame is controlled by the setting of Bit\\[23\\] (WD) and Bit\\[20\\] (JE) in EMACCONFIG_REG."]
pub type PWDOGEN_R = crate::BitReader;
#[doc = "Field `PWDOGEN` writer - When this bit is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared the watchdog timeout for a received frame is controlled by the setting of Bit\\[23\\] (WD) and Bit\\[20\\] (JE) in EMACCONFIG_REG."]
pub type PWDOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - When Bit\\[16\\] (PWE) is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field such frame is terminated and declared as an error frame."]
    #[inline(always)]
    pub fn wdogto(&self) -> WDOGTO_R {
        WDOGTO_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - When this bit is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared the watchdog timeout for a received frame is controlled by the setting of Bit\\[23\\] (WD) and Bit\\[20\\] (JE) in EMACCONFIG_REG."]
    #[inline(always)]
    pub fn pwdogen(&self) -> PWDOGEN_R {
        PWDOGEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOGTO")
            .field("wdogto", &self.wdogto())
            .field("pwdogen", &self.pwdogen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - When Bit\\[16\\] (PWE) is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field such frame is terminated and declared as an error frame."]
    #[inline(always)]
    pub fn wdogto(&mut self) -> WDOGTO_W<'_, WDOGTO_SPEC> {
        WDOGTO_W::new(self, 0)
    }
    #[doc = "Bit 16 - When this bit is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared the watchdog timeout for a received frame is controlled by the setting of Bit\\[23\\] (WD) and Bit\\[20\\] (JE) in EMACCONFIG_REG."]
    #[inline(always)]
    pub fn pwdogen(&mut self) -> PWDOGEN_W<'_, WDOGTO_SPEC> {
        PWDOGEN_W::new(self, 16)
    }
}
#[doc = "Watchdog timeout control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGTO_SPEC;
impl crate::RegisterSpec for WDOGTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogto::R`](R) reader structure"]
impl crate::Readable for WDOGTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdogto::W`](W) writer structure"]
impl crate::Writable for WDOGTO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGTO to value 0"]
impl crate::Resettable for WDOGTO_SPEC {}
