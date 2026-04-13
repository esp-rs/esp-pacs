#[doc = "Register `INTR_SIG_IDX_ASSERT_IN_SEC` reader"]
pub type R = crate::R<INTR_SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "Register `INTR_SIG_IDX_ASSERT_IN_SEC` writer"]
pub type W = crate::W<INTR_SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "Field `CORE1_INTR_SIG_IDX_ASSERT_IN_SEC` reader - "]
pub type CORE1_INTR_SIG_IDX_ASSERT_IN_SEC_R = crate::FieldReader;
#[doc = "Field `CORE1_INTR_SIG_IDX_ASSERT_IN_SEC` writer - "]
pub type CORE1_INTR_SIG_IDX_ASSERT_IN_SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn core1_intr_sig_idx_assert_in_sec(&self) -> CORE1_INTR_SIG_IDX_ASSERT_IN_SEC_R {
        CORE1_INTR_SIG_IDX_ASSERT_IN_SEC_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_SIG_IDX_ASSERT_IN_SEC")
            .field(
                "core1_intr_sig_idx_assert_in_sec",
                &self.core1_intr_sig_idx_assert_in_sec(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn core1_intr_sig_idx_assert_in_sec(
        &mut self,
    ) -> CORE1_INTR_SIG_IDX_ASSERT_IN_SEC_W<'_, INTR_SIG_IDX_ASSERT_IN_SEC_SPEC> {
        CORE1_INTR_SIG_IDX_ASSERT_IN_SEC_W::new(self, 0)
    }
}
#[doc = "Interrupt signal index assert in secure\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sig_idx_assert_in_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sig_idx_assert_in_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SIG_IDX_ASSERT_IN_SEC_SPEC;
impl crate::RegisterSpec for INTR_SIG_IDX_ASSERT_IN_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sig_idx_assert_in_sec::R`](R) reader structure"]
impl crate::Readable for INTR_SIG_IDX_ASSERT_IN_SEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_sig_idx_assert_in_sec::W`](W) writer structure"]
impl crate::Writable for INTR_SIG_IDX_ASSERT_IN_SEC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_SIG_IDX_ASSERT_IN_SEC to value 0"]
impl crate::Resettable for INTR_SIG_IDX_ASSERT_IN_SEC_SPEC {}
