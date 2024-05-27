///Register `LP_CPU_EXC_PC` reader
pub type R = crate::R<LP_CPU_EXC_PC_SPEC>;
///Field `LP_CPU_EXC_PC` reader - need_des
pub type LP_CPU_EXC_PC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn lp_cpu_exc_pc(&self) -> LP_CPU_EXC_PC_R {
        LP_CPU_EXC_PC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CPU_EXC_PC")
            .field("lp_cpu_exc_pc", &self.lp_cpu_exc_pc())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_exc_pc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_CPU_EXC_PC_SPEC;
impl crate::RegisterSpec for LP_CPU_EXC_PC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_cpu_exc_pc::R`](R) reader structure
impl crate::Readable for LP_CPU_EXC_PC_SPEC {}
///`reset()` method sets LP_CPU_EXC_PC to value 0
impl crate::Resettable for LP_CPU_EXC_PC_SPEC {
    const RESET_VALUE: u32 = 0;
}
