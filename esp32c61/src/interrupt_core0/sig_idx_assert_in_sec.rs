#[doc = "Register `SIG_IDX_ASSERT_IN_SEC` reader"]
pub type R = crate::R<SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "Register `SIG_IDX_ASSERT_IN_SEC` writer"]
pub type W = crate::W<SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "Field `INT_SIG_IDX_ASSERT_IN_SEC` reader - reserved"]
pub type INT_SIG_IDX_ASSERT_IN_SEC_R = crate::FieldReader;
#[doc = "Field `INT_SIG_IDX_ASSERT_IN_SEC` writer - reserved"]
pub type INT_SIG_IDX_ASSERT_IN_SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - reserved"]
    #[inline(always)]
    pub fn int_sig_idx_assert_in_sec(&self) -> INT_SIG_IDX_ASSERT_IN_SEC_R {
        INT_SIG_IDX_ASSERT_IN_SEC_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIG_IDX_ASSERT_IN_SEC")
            .field(
                "int_sig_idx_assert_in_sec",
                &self.int_sig_idx_assert_in_sec(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - reserved"]
    #[inline(always)]
    pub fn int_sig_idx_assert_in_sec(
        &mut self,
    ) -> INT_SIG_IDX_ASSERT_IN_SEC_W<SIG_IDX_ASSERT_IN_SEC_SPEC> {
        INT_SIG_IDX_ASSERT_IN_SEC_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sig_idx_assert_in_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sig_idx_assert_in_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIG_IDX_ASSERT_IN_SEC_SPEC;
impl crate::RegisterSpec for SIG_IDX_ASSERT_IN_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sig_idx_assert_in_sec::R`](R) reader structure"]
impl crate::Readable for SIG_IDX_ASSERT_IN_SEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sig_idx_assert_in_sec::W`](W) writer structure"]
impl crate::Writable for SIG_IDX_ASSERT_IN_SEC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIG_IDX_ASSERT_IN_SEC to value 0"]
impl crate::Resettable for SIG_IDX_ASSERT_IN_SEC_SPEC {}
