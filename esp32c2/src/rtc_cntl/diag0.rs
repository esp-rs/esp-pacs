#[doc = "Register `DIAG0` reader"]
pub type R = crate::R<DIAG0_SPEC>;
#[doc = "Register `DIAG0` writer"]
pub type W = crate::W<DIAG0_SPEC>;
#[doc = "Field `LOW_POWER_DIAG1` reader - Need add desc"]
pub type LOW_POWER_DIAG1_R = crate::FieldReader<u32>;
#[doc = "Field `LOW_POWER_DIAG1` writer - Need add desc"]
pub type LOW_POWER_DIAG1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn low_power_diag1(&self) -> LOW_POWER_DIAG1_R {
        LOW_POWER_DIAG1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIAG0")
            .field(
                "low_power_diag1",
                &format_args!("{}", self.low_power_diag1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIAG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn low_power_diag1(&mut self) -> LOW_POWER_DIAG1_W<DIAG0_SPEC> {
        LOW_POWER_DIAG1_W::new(self, 0)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diag0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diag0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIAG0_SPEC;
impl crate::RegisterSpec for DIAG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diag0::R`](R) reader structure"]
impl crate::Readable for DIAG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diag0::W`](W) writer structure"]
impl crate::Writable for DIAG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIAG0 to value 0"]
impl crate::Resettable for DIAG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
