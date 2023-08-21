#[doc = "Register `RD_BLK3_DATA4` reader"]
pub type R = crate::R<RD_BLK3_DATA4_SPEC>;
#[doc = "Field `BLK3_DATA4` reader - Store the fifth 32-bit of Block3."]
pub type BLK3_DATA4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Store the fifth 32-bit of Block3."]
    #[inline(always)]
    pub fn blk3_data4(&self) -> BLK3_DATA4_R {
        BLK3_DATA4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK3_DATA4")
            .field("blk3_data4", &format_args!("{}", self.blk3_data4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK3_DATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 4 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK3_DATA4_SPEC;
impl crate::RegisterSpec for RD_BLK3_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk3_data4::R`](R) reader structure"]
impl crate::Readable for RD_BLK3_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_BLK3_DATA4 to value 0"]
impl crate::Resettable for RD_BLK3_DATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
