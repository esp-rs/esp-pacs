#[doc = "Register `BLK2_RDATA3` reader"]
pub struct R(crate::R<BLK2_RDATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK2_RDATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK2_RDATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK2_RDATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK2_DOUT3` reader - read for BLOCK2"]
pub type BLK2_DOUT3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn blk2_dout3(&self) -> BLK2_DOUT3_R {
        BLK2_DOUT3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_RDATA3")
            .field("blk2_dout3", &format_args!("{}", self.blk2_dout3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK2_RDATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk2_rdata3](index.html) module"]
pub struct BLK2_RDATA3_SPEC;
impl crate::RegisterSpec for BLK2_RDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk2_rdata3::R](R) reader structure"]
impl crate::Readable for BLK2_RDATA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK2_RDATA3 to value 0"]
impl crate::Resettable for BLK2_RDATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
