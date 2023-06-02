#[doc = "Register `L2_CACHE_AUTOLOAD_BUF_CLR_CTRL` reader"]
pub struct R(crate::R<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
pub type L2_CACHE_ALD_BUF_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_ald_buf_clr(&self) -> L2_CACHE_ALD_BUF_CLR_R {
        L2_CACHE_ALD_BUF_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_AUTOLOAD_BUF_CLR_CTRL")
            .field(
                "l2_cache_ald_buf_clr",
                &format_args!("{}", self.l2_cache_ald_buf_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Autoload buffer clear control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_autoload_buf_clr_ctrl](index.html) module"]
pub struct L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_autoload_buf_clr_ctrl::R](R) reader structure"]
impl crate::Readable for L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_BUF_CLR_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
