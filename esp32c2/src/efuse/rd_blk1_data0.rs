#[doc = "Register `RD_BLK1_DATA0` reader"]
pub type R = crate::R<RD_BLK1_DATA0_SPEC>;
#[doc = "Field `SYSTEM_DATA0` reader - Stores the bits \\[0:31\\] of system data."]
pub type SYSTEM_DATA0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the bits \\[0:31\\] of system data."]
    #[inline(always)]
    pub fn system_data0(&self) -> SYSTEM_DATA0_R {
        SYSTEM_DATA0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK1_DATA0")
            .field("system_data0", &self.system_data0().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_BLK1_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "BLOCK1 data register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk1_data0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK1_DATA0_SPEC;
impl crate::RegisterSpec for RD_BLK1_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk1_data0::R`](R) reader structure"]
impl crate::Readable for RD_BLK1_DATA0_SPEC {}
#[doc = "`reset()` method sets RD_BLK1_DATA0 to value 0"]
impl crate::Resettable for RD_BLK1_DATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
