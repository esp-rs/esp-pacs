#[doc = "Register `EDMA_PMS_RMT_LOCK` reader"]
pub type R = crate::R<EDMA_PMS_RMT_LOCK_SPEC>;
#[doc = "Register `EDMA_PMS_RMT_LOCK` writer"]
pub type W = crate::W<EDMA_PMS_RMT_LOCK_SPEC>;
#[doc = "Field `EDMA_PMS_RMT_LOCK` reader - Set 1 to lock EDMA-RMT permission control registers."]
pub type EDMA_PMS_RMT_LOCK_R = crate::BitReader;
#[doc = "Field `EDMA_PMS_RMT_LOCK` writer - Set 1 to lock EDMA-RMT permission control registers."]
pub type EDMA_PMS_RMT_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock EDMA-RMT permission control registers."]
    #[inline(always)]
    pub fn edma_pms_rmt_lock(&self) -> EDMA_PMS_RMT_LOCK_R {
        EDMA_PMS_RMT_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_PMS_RMT_LOCK")
            .field(
                "edma_pms_rmt_lock",
                &format_args!("{}", self.edma_pms_rmt_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDMA_PMS_RMT_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock EDMA-RMT permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn edma_pms_rmt_lock(&mut self) -> EDMA_PMS_RMT_LOCK_W<EDMA_PMS_RMT_LOCK_SPEC, 0> {
        EDMA_PMS_RMT_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EDMA-RMT permission lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_rmt_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_rmt_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDMA_PMS_RMT_LOCK_SPEC;
impl crate::RegisterSpec for EDMA_PMS_RMT_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edma_pms_rmt_lock::R`](R) reader structure"]
impl crate::Readable for EDMA_PMS_RMT_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`edma_pms_rmt_lock::W`](W) writer structure"]
impl crate::Writable for EDMA_PMS_RMT_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDMA_PMS_RMT_LOCK to value 0"]
impl crate::Resettable for EDMA_PMS_RMT_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
