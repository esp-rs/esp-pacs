#[doc = "Register `L2_CACHE_FREEZE_CTRL` reader"]
pub struct R(crate::R<L2_CACHE_FREEZE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_FREEZE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_FREEZE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_FREEZE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_FREEZE_EN` reader - The bit is used to enable freeze operation on L2-Cache. It can be cleared by software."]
pub type L2_CACHE_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L2-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type L2_CACHE_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `L2_CACHE_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
pub type L2_CACHE_FREEZE_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - The bit is used to enable freeze operation on L2-Cache. It can be cleared by software."]
    #[inline(always)]
    pub fn l2_cache_freeze_en(&self) -> L2_CACHE_FREEZE_EN_R {
        L2_CACHE_FREEZE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to configure mode of freeze operation L2-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn l2_cache_freeze_mode(&self) -> L2_CACHE_FREEZE_MODE_R {
        L2_CACHE_FREEZE_MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is used to indicate whether freeze operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l2_cache_freeze_done(&self) -> L2_CACHE_FREEZE_DONE_R {
        L2_CACHE_FREEZE_DONE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_FREEZE_CTRL")
            .field(
                "l2_cache_freeze_en",
                &format_args!("{}", self.l2_cache_freeze_en().bit()),
            )
            .field(
                "l2_cache_freeze_mode",
                &format_args!("{}", self.l2_cache_freeze_mode().bit()),
            )
            .field(
                "l2_cache_freeze_done",
                &format_args!("{}", self.l2_cache_freeze_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_FREEZE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Freeze control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_freeze_ctrl](index.html) module"]
pub struct L2_CACHE_FREEZE_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_FREEZE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_freeze_ctrl::R](R) reader structure"]
impl crate::Readable for L2_CACHE_FREEZE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_FREEZE_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_FREEZE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
