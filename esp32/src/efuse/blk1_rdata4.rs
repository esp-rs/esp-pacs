#[doc = "Register `BLK1_RDATA4` reader"]
pub struct R(crate::R<BLK1_RDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK1_RDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK1_RDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK1_RDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK1_DOUT4` reader - read for BLOCK1"]
pub type BLK1_DOUT4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - read for BLOCK1"]
    #[inline(always)]
    pub fn blk1_dout4(&self) -> BLK1_DOUT4_R {
        BLK1_DOUT4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_RDATA4")
            .field("blk1_dout4", &format_args!("{}", self.blk1_dout4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK1_RDATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk1_rdata4](index.html) module"]
pub struct BLK1_RDATA4_SPEC;
impl crate::RegisterSpec for BLK1_RDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk1_rdata4::R](R) reader structure"]
impl crate::Readable for BLK1_RDATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK1_RDATA4 to value 0"]
impl crate::Resettable for BLK1_RDATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
