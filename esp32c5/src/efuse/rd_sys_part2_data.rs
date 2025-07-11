#[doc = "Register `RD_SYS_PART2_DATA%s` reader"]
pub type R = crate::R<RD_SYS_PART2_DATA_SPEC>;
#[doc = "Field `SYS_DATA_PART2` reader - Represents the zeroth 32-bit of second part of system data."]
pub type SYS_DATA_PART2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of second part of system data."]
    #[inline(always)]
    pub fn sys_data_part2(&self) -> SYS_DATA_PART2_R {
        SYS_DATA_PART2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART2_DATA")
            .field("sys_data_part2", &self.sys_data_part2())
            .finish()
    }
}
#[doc = "Represents rd_sys_part2_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART2_DATA_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA%s to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA_SPEC {}
