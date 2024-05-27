///Register `RD_SYS_PART1_DATA5` reader
pub type R = crate::R<RD_SYS_PART1_DATA5_SPEC>;
///Field `SYS_DATA_PART1_5` reader - Stores the fifth 32 bits of the first part of system data.
pub type SYS_DATA_PART1_5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the fifth 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_5(&self) -> SYS_DATA_PART1_5_R {
        SYS_DATA_PART1_5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA5")
            .field("sys_data_part1_5", &self.sys_data_part1_5())
            .finish()
    }
}
/**Register 5 of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_SYS_PART1_DATA5_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_sys_part1_data5::R`](R) reader structure
impl crate::Readable for RD_SYS_PART1_DATA5_SPEC {}
///`reset()` method sets RD_SYS_PART1_DATA5 to value 0
impl crate::Resettable for RD_SYS_PART1_DATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
