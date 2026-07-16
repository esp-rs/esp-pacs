#[doc = "Register `CONFIG_STATE` reader"]
pub type R = crate::R<CONFIG_STATE_SPEC>;
#[doc = "Field `SIG_IDX_ASSERT_S_ERR` reader - "]
pub type SIG_IDX_ASSERT_S_ERR_R = crate::BitReader;
#[doc = "Field `SIG_IDX_ASSERT_M_ERR` reader - "]
pub type SIG_IDX_ASSERT_M_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sig_idx_assert_s_err(&self) -> SIG_IDX_ASSERT_S_ERR_R {
        SIG_IDX_ASSERT_S_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sig_idx_assert_m_err(&self) -> SIG_IDX_ASSERT_M_ERR_R {
        SIG_IDX_ASSERT_M_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_STATE")
            .field("sig_idx_assert_s_err", &self.sig_idx_assert_s_err())
            .field("sig_idx_assert_m_err", &self.sig_idx_assert_m_err())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`config_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_STATE_SPEC;
impl crate::RegisterSpec for CONFIG_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_state::R`](R) reader structure"]
impl crate::Readable for CONFIG_STATE_SPEC {}
#[doc = "`reset()` method sets CONFIG_STATE to value 0"]
impl crate::Resettable for CONFIG_STATE_SPEC {}
