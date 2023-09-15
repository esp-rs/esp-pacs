#[doc = "Register `EMACWDOGTO` reader"]
pub type R = crate::R<EMACWDOGTO_SPEC>;
#[doc = "Register `EMACWDOGTO` writer"]
pub type W = crate::W<EMACWDOGTO_SPEC>;
#[doc = "Field `WDOGTO` reader - When Bit\\[16\\] (PWE) is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field such frame is terminated and declared as an error frame."]
pub type WDOGTO_R = crate::FieldReader<u16>;
#[doc = "Field `WDOGTO` writer - When Bit\\[16\\] (PWE) is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field such frame is terminated and declared as an error frame."]
pub type WDOGTO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `PWDOGEN` reader - When this bit is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared the watchdog timeout for a received frame is controlled by the setting of Bit\\[23\\] (WD) and Bit\\[20\\] (JE) in EMACCONFIG_REG."]
pub type PWDOGEN_R = crate::BitReader;
#[doc = "Field `PWDOGEN` writer - When this bit is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared the watchdog timeout for a received frame is controlled by the setting of Bit\\[23\\] (WD) and Bit\\[20\\] (JE) in EMACCONFIG_REG."]
pub type PWDOGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
        f.debug_struct("EMACWDOGTO")
            .field("wdogto", &format_args!("{}", self.wdogto().bits()))
            .field("pwdogen", &format_args!("{}", self.pwdogen().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACWDOGTO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - When Bit\\[16\\] (PWE) is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field such frame is terminated and declared as an error frame."]
    #[inline(always)]
    #[must_use]
    pub fn wdogto(&mut self) -> WDOGTO_W<EMACWDOGTO_SPEC, 0> {
        WDOGTO_W::new(self)
    }
    #[doc = "Bit 16 - When this bit is set and Bit\\[23\\] (WD) of EMACCONFIG_REG is reset the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared the watchdog timeout for a received frame is controlled by the setting of Bit\\[23\\] (WD) and Bit\\[20\\] (JE) in EMACCONFIG_REG."]
    #[inline(always)]
    #[must_use]
    pub fn pwdogen(&mut self) -> PWDOGEN_W<EMACWDOGTO_SPEC, 16> {
        PWDOGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Watchdog timeout control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacwdogto::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacwdogto::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACWDOGTO_SPEC;
impl crate::RegisterSpec for EMACWDOGTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacwdogto::R`](R) reader structure"]
impl crate::Readable for EMACWDOGTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacwdogto::W`](W) writer structure"]
impl crate::Writable for EMACWDOGTO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACWDOGTO to value 0"]
impl crate::Resettable for EMACWDOGTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
