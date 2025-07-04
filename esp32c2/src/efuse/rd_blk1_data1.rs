#[doc = "Register `RD_BLK1_DATA1` reader"]
pub type R = crate::R<RD_BLK1_DATA1_SPEC>;
#[doc = "Field `SYSTEM_DATA1` reader - Stores the bits \\[32:63\\] of system data."]
pub type SYSTEM_DATA1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the bits \\[32:63\\] of system data."]
    #[inline(always)]
    pub fn system_data1(&self) -> SYSTEM_DATA1_R {
        SYSTEM_DATA1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK1_DATA1")
            .field("system_data1", &self.system_data1())
            .finish()
    }
}
#[doc = "BLOCK1 data register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_blk1_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK1_DATA1_SPEC;
impl crate::RegisterSpec for RD_BLK1_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk1_data1::R`](R) reader structure"]
impl crate::Readable for RD_BLK1_DATA1_SPEC {}
#[doc = "`reset()` method sets RD_BLK1_DATA1 to value 0"]
impl crate::Resettable for RD_BLK1_DATA1_SPEC {}
