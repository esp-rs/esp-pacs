#[doc = "Register `RD_BLK1_DATA2` reader"]
pub struct R(crate::R<RD_BLK1_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK1_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK1_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK1_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYSTEM_DATA2` reader - Stores the bits \\[64:87\\] of system data."]
pub type SYSTEM_DATA2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Stores the bits \\[64:87\\] of system data."]
    #[inline(always)]
    pub fn system_data2(&self) -> SYSTEM_DATA2_R {
        SYSTEM_DATA2_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK1_DATA2")
            .field(
                "system_data2",
                &format_args!("{}", self.system_data2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK1_DATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "BLOCK1 data register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk1_data2](index.html) module"]
pub struct RD_BLK1_DATA2_SPEC;
impl crate::RegisterSpec for RD_BLK1_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk1_data2::R](R) reader structure"]
impl crate::Readable for RD_BLK1_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK1_DATA2 to value 0"]
impl crate::Resettable for RD_BLK1_DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
