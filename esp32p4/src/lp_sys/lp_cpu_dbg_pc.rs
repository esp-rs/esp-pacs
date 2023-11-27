#[doc = "Register `LP_CPU_DBG_PC` reader"]
pub type R = crate::R<LP_CPU_DBG_PC_SPEC>;
#[doc = "Field `LP_CPU_DBG_PC` reader - need_des"]
pub type LP_CPU_DBG_PC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_dbg_pc(&self) -> LP_CPU_DBG_PC_R {
        LP_CPU_DBG_PC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CPU_DBG_PC")
            .field(
                "lp_cpu_dbg_pc",
                &format_args!("{}", self.lp_cpu_dbg_pc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_CPU_DBG_PC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_dbg_pc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CPU_DBG_PC_SPEC;
impl crate::RegisterSpec for LP_CPU_DBG_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_dbg_pc::R`](R) reader structure"]
impl crate::Readable for LP_CPU_DBG_PC_SPEC {}
#[doc = "`reset()` method sets LP_CPU_DBG_PC to value 0"]
impl crate::Resettable for LP_CPU_DBG_PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
