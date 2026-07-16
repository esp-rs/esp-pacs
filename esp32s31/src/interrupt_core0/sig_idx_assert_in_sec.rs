#[doc = "Register `SIG_IDX_ASSERT_IN_SEC` reader"]
pub type R = crate::R<SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "Register `SIG_IDX_ASSERT_IN_SEC` writer"]
pub type W = crate::W<SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = "Field `INT_SIG_IDX_ASSERT_IN_S` reader - "]
pub type INT_SIG_IDX_ASSERT_IN_S_R = crate::FieldReader;
#[doc = "Field `INT_SIG_IDX_ASSERT_IN_S` writer - "]
pub type INT_SIG_IDX_ASSERT_IN_S_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `INT_SIG_IDX_ASSERT_IN_M` reader - "]
pub type INT_SIG_IDX_ASSERT_IN_M_R = crate::FieldReader;
#[doc = "Field `INT_SIG_IDX_ASSERT_IN_M` writer - "]
pub type INT_SIG_IDX_ASSERT_IN_M_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_sig_idx_assert_in_s(&self) -> INT_SIG_IDX_ASSERT_IN_S_R {
        INT_SIG_IDX_ASSERT_IN_S_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn int_sig_idx_assert_in_m(&self) -> INT_SIG_IDX_ASSERT_IN_M_R {
        INT_SIG_IDX_ASSERT_IN_M_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIG_IDX_ASSERT_IN_SEC")
            .field("int_sig_idx_assert_in_s", &self.int_sig_idx_assert_in_s())
            .field("int_sig_idx_assert_in_m", &self.int_sig_idx_assert_in_m())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn int_sig_idx_assert_in_s(
        &mut self,
    ) -> INT_SIG_IDX_ASSERT_IN_S_W<'_, SIG_IDX_ASSERT_IN_SEC_SPEC> {
        INT_SIG_IDX_ASSERT_IN_S_W::new(self, 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn int_sig_idx_assert_in_m(
        &mut self,
    ) -> INT_SIG_IDX_ASSERT_IN_M_W<'_, SIG_IDX_ASSERT_IN_SEC_SPEC> {
        INT_SIG_IDX_ASSERT_IN_M_W::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sig_idx_assert_in_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sig_idx_assert_in_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
