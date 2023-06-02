#[doc = "Register `RD_BLK2_DATA7` reader"]
pub struct R(crate::R<RD_BLK2_DATA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK2_DATA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK2_DATA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK2_DATA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK2_RESERVED_DATA_1` reader - Store the bit \\[21:52\\] of block2 reserved data."]
pub type BLK2_RESERVED_DATA_1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Store the bit \\[21:52\\] of block2 reserved data."]
    #[inline(always)]
    pub fn blk2_reserved_data_1(&self) -> BLK2_RESERVED_DATA_1_R {
        BLK2_RESERVED_DATA_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA7")
            .field(
                "blk2_reserved_data_1",
                &format_args!("{}", self.blk2_reserved_data_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK2_DATA7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 7 of BLOCK2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk2_data7](index.html) module"]
pub struct RD_BLK2_DATA7_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk2_data7::R](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK2_DATA7 to value 0"]
impl crate::Resettable for RD_BLK2_DATA7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
