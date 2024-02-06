#[doc = "Register `LACTALARMHI` reader"]
pub type R = crate::R<LACTALARMHI_SPEC>;
#[doc = "Register `LACTALARMHI` writer"]
pub type W = crate::W<LACTALARMHI_SPEC>;
#[doc = "Field `LACT_ALARM_HI` reader - Reserved."]
pub type LACT_ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `LACT_ALARM_HI` writer - Reserved."]
pub type LACT_ALARM_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn lact_alarm_hi(&self) -> LACT_ALARM_HI_R {
        LACT_ALARM_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTALARMHI")
            .field(
                "lact_alarm_hi",
                &format_args!("{}", self.lact_alarm_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTALARMHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn lact_alarm_hi(&mut self) -> LACT_ALARM_HI_W<LACTALARMHI_SPEC> {
        LACT_ALARM_HI_W::new(self, 0)
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
#[doc = "LACT alarm high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lactalarmhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTALARMHI_SPEC;
impl crate::RegisterSpec for LACTALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactalarmhi::R`](R) reader structure"]
impl crate::Readable for LACTALARMHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactalarmhi::W`](W) writer structure"]
impl crate::Writable for LACTALARMHI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTALARMHI to value 0"]
impl crate::Resettable for LACTALARMHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
