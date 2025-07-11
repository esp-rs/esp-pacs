#[doc = "Register `RD_WR_DIS0` reader"]
pub type R = crate::R<RD_WR_DIS0_SPEC>;
#[doc = "Field `WR_DIS` reader - Represents whether programming of individual eFuse memory bit is disabled or enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type WR_DIS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents whether programming of individual eFuse memory bit is disabled or enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_WR_DIS0")
            .field("wr_dis", &self.wr_dis())
            .finish()
    }
}
#[doc = "Represents rd_wr_dis\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_wr_dis0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_WR_DIS0_SPEC;
impl crate::RegisterSpec for RD_WR_DIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_wr_dis0::R`](R) reader structure"]
impl crate::Readable for RD_WR_DIS0_SPEC {}
#[doc = "`reset()` method sets RD_WR_DIS0 to value 0"]
impl crate::Resettable for RD_WR_DIS0_SPEC {}
