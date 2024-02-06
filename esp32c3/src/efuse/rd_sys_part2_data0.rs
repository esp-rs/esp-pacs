#[doc = "Register `RD_SYS_PART2_DATA0` reader"]
pub type R = crate::R<RD_SYS_PART2_DATA0_SPEC>;
#[doc = "Field `SYS_DATA_PART2_0` reader - Stores the 0th 32 bits of the 2nd part of system data."]
pub type SYS_DATA_PART2_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the 0th 32 bits of the 2nd part of system data."]
    #[inline(always)]
    pub fn sys_data_part2_0(&self) -> SYS_DATA_PART2_0_R {
        SYS_DATA_PART2_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART2_DATA0")
            .field(
                "sys_data_part2_0",
                &format_args!("{}", self.sys_data_part2_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_SYS_PART2_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register 0 of BLOCK10 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART2_DATA0_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data0::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA0_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA0 to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
