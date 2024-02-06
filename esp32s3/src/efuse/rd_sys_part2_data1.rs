#[doc = "Register `RD_SYS_PART2_DATA1` reader"]
pub type R = crate::R<RD_SYS_PART2_DATA1_SPEC>;
#[doc = "Field `SYS_DATA_PART2_1` reader - Stores the 1st 32 bits of the 2nd part of system data."]
pub type SYS_DATA_PART2_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the 1st 32 bits of the 2nd part of system data."]
    #[inline(always)]
    pub fn sys_data_part2_1(&self) -> SYS_DATA_PART2_1_R {
        SYS_DATA_PART2_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART2_DATA1")
            .field(
                "sys_data_part2_1",
                &format_args!("{}", self.sys_data_part2_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_SYS_PART2_DATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register 1 of BLOCK9 (KEY5).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART2_DATA1_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data1::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA1_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA1 to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
