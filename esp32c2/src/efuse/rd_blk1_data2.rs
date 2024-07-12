#[doc = "Register `RD_BLK1_DATA2` reader"]
pub type R = crate::R<RD_BLK1_DATA2_SPEC>;
#[doc = "Field `SYSTEM_DATA2` reader - Stores the bits \\[64:87\\] of system data."]
pub type SYSTEM_DATA2_R = crate::FieldReader<u32>;
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
            .field("system_data2", &self.system_data2())
            .finish()
    }
}
#[doc = "BLOCK1 data register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_blk1_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK1_DATA2_SPEC;
impl crate::RegisterSpec for RD_BLK1_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk1_data2::R`](R) reader structure"]
impl crate::Readable for RD_BLK1_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_BLK1_DATA2 to value 0"]
impl crate::Resettable for RD_BLK1_DATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
