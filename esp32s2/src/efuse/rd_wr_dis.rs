#[doc = "Register `RD_WR_DIS` reader"]
pub type R = crate::R<RD_WR_DIS_SPEC>;
#[doc = "Field `WR_DIS` reader - Disables programming of individual eFuses."]
pub type WR_DIS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Disables programming of individual eFuses."]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_WR_DIS")
            .field("wr_dis", &self.wr_dis())
            .finish()
    }
}
#[doc = "Register 0 of BLOCK0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_wr_dis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_WR_DIS_SPEC;
impl crate::RegisterSpec for RD_WR_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_wr_dis::R`](R) reader structure"]
impl crate::Readable for RD_WR_DIS_SPEC {}
#[doc = "`reset()` method sets RD_WR_DIS to value 0"]
impl crate::Resettable for RD_WR_DIS_SPEC {}
