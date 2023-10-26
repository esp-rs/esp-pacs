#[doc = "Register `LACTALARMLO` reader"]
pub type R = crate::R<LACTALARMLO_SPEC>;
#[doc = "Register `LACTALARMLO` writer"]
pub type W = crate::W<LACTALARMLO_SPEC>;
#[doc = "Field `LACT_ALARM_LO` reader - Reserved."]
pub type LACT_ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LACT_ALARM_LO` writer - Reserved."]
pub type LACT_ALARM_LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn lact_alarm_lo(&self) -> LACT_ALARM_LO_R {
        LACT_ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTALARMLO")
            .field(
                "lact_alarm_lo",
                &format_args!("{}", self.lact_alarm_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTALARMLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn lact_alarm_lo(&mut self) -> LACT_ALARM_LO_W<LACTALARMLO_SPEC, 0> {
        LACT_ALARM_LO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LACT alarm low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTALARMLO_SPEC;
impl crate::RegisterSpec for LACTALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactalarmlo::R`](R) reader structure"]
impl crate::Readable for LACTALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactalarmlo::W`](W) writer structure"]
impl crate::Writable for LACTALARMLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTALARMLO to value 0"]
impl crate::Resettable for LACTALARMLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
