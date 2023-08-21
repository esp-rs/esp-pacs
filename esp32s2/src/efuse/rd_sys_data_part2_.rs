#[doc = "Register `RD_SYS_DATA_PART2_%s` reader"]
pub type R = crate::R<RD_SYS_DATA_PART2__SPEC>;
#[doc = "Field `SYS_DATA_PART2` reader - Stores the %sth 32 bits of the 2nd part of system data."]
pub type SYS_DATA_PART2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32 bits of the 2nd part of system data."]
    #[inline(always)]
    pub fn sys_data_part2(&self) -> SYS_DATA_PART2_R {
        SYS_DATA_PART2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_DATA_PART2_")
            .field(
                "sys_data_part2",
                &format_args!("{}", self.sys_data_part2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_SYS_DATA_PART2__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register %s of BLOCK10 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_data_part2_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_DATA_PART2__SPEC;
impl crate::RegisterSpec for RD_SYS_DATA_PART2__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_data_part2_::R`](R) reader structure"]
impl crate::Readable for RD_SYS_DATA_PART2__SPEC {}
#[doc = "`reset()` method sets RD_SYS_DATA_PART2_%s to value 0"]
impl crate::Resettable for RD_SYS_DATA_PART2__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
