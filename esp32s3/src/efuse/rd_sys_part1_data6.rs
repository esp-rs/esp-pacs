#[doc = "Register `RD_SYS_PART1_DATA6` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA6_SPEC>;
#[doc = "Field `SYS_DATA_PART1_6` reader - Stores the sixth 32 bits of the first part of system data."]
pub type SYS_DATA_PART1_6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the sixth 32 bits of the first part of system data."]
    #[inline(always)]
    pub fn sys_data_part1_6(&self) -> SYS_DATA_PART1_6_R {
        SYS_DATA_PART1_6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA6")
            .field(
                "sys_data_part1_6",
                &format_args!("{}", self.sys_data_part1_6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_SYS_PART1_DATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register 6 of BLOCK2 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA6_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data6::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA6_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA6 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
