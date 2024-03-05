#[doc = "Register `EXTMEM_REJECT_INT_RAW` reader"]
pub type R = crate::R<EXTMEM_REJECT_INT_RAW_SPEC>;
#[doc = "Register `EXTMEM_REJECT_INT_RAW` writer"]
pub type W = crate::W<EXTMEM_REJECT_INT_RAW_SPEC>;
#[doc = "Field `EXTMEM_REJECT_INT_RAW` reader - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
pub type EXTMEM_REJECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `EXTMEM_REJECT_INT_RAW` writer - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
pub type EXTMEM_REJECT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
    #[inline(always)]
    pub fn extmem_reject_int_raw(&self) -> EXTMEM_REJECT_INT_RAW_R {
        EXTMEM_REJECT_INT_RAW_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM_REJECT_INT_RAW")
            .field(
                "extmem_reject_int_raw",
                &format_args!("{}", self.extmem_reject_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTMEM_REJECT_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
    #[inline(always)]
    #[must_use]
    pub fn extmem_reject_int_raw(&mut self) -> EXTMEM_REJECT_INT_RAW_W<EXTMEM_REJECT_INT_RAW_SPEC> {
        EXTMEM_REJECT_INT_RAW_W::new(self, 0)
    }
}
#[doc = "Raw interrupt status of external RAM permission\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTMEM_REJECT_INT_RAW_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extmem_reject_int_raw::R`](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extmem_reject_int_raw::W`](W) writer structure"]
impl crate::Writable for EXTMEM_REJECT_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_RAW to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
