#[doc = "Register `WR_DIS` reader"]
pub struct R(crate::R<WR_DIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_DIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_DIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_DIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK0_WR_DIS` reader - Otp block0 write disable data."]
pub type BLOCK0_WR_DIS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 write disable data."]
    #[inline(always)]
    pub fn block0_wr_dis(&self) -> BLOCK0_WR_DIS_R {
        BLOCK0_WR_DIS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_DIS")
            .field(
                "block0_wr_dis",
                &format_args!("{}", self.block0_wr_dis().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_DIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block0 data register1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_dis](index.html) module"]
pub struct WR_DIS_SPEC;
impl crate::RegisterSpec for WR_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_dis::R](R) reader structure"]
impl crate::Readable for WR_DIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WR_DIS to value 0"]
impl crate::Resettable for WR_DIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
