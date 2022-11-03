#[doc = "Register `L2_CACHE_PRELOAD_RST_CTRL` reader"]
pub struct R(crate::R<L2_CACHE_PRELOAD_RST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_PRELOAD_RST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_PRELOAD_RST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_PRELOAD_RST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_PLD_RST` reader - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L2_CACHE_PLD_RST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 5 - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l2_cache_pld_rst(&self) -> L2_CACHE_PLD_RST_R {
        L2_CACHE_PLD_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Cache Preload Reset control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_preload_rst_ctrl](index.html) module"]
pub struct L2_CACHE_PRELOAD_RST_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_preload_rst_ctrl::R](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOAD_RST_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_PRELOAD_RST_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
