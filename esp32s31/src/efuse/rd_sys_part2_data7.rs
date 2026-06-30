#[doc = "Register `RD_SYS_PART2_DATA7` reader"]
pub type R = crate::R<RD_SYS_PART2_DATA7_SPEC>;
#[doc = "Field `PVT_0_LIMIT` reader - Power glitch monitor threthold."]
pub type PVT_0_LIMIT_R = crate::FieldReader<u16>;
#[doc = "Field `PVT_1_LIMIT` reader - Power glitch monitor threthold."]
pub type PVT_1_LIMIT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Power glitch monitor threthold."]
    #[inline(always)]
    pub fn pvt_0_limit(&self) -> PVT_0_LIMIT_R {
        PVT_0_LIMIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Power glitch monitor threthold."]
    #[inline(always)]
    pub fn pvt_1_limit(&self) -> PVT_1_LIMIT_R {
        PVT_1_LIMIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART2_DATA7")
            .field("pvt_0_limit", &self.pvt_0_limit())
            .field("pvt_1_limit", &self.pvt_1_limit())
            .finish()
    }
}
#[doc = "Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART2_DATA7_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data7::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA7_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA7 to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA7_SPEC {}
