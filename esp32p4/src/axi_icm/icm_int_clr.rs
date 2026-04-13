#[doc = "Register `ICM_INT_CLR` writer"]
pub type W = crate::W<ICM_INT_CLR_SPEC>;
#[doc = "Field `ICM_REG_DLOCK_INT_CLR` writer - "]
pub type ICM_REG_DLOCK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_ICM_SYS_ADDRHOLE_INT_CLR` writer - "]
pub type ICM_REG_ICM_SYS_ADDRHOLE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_ICM_CPU_ADDRHOLE_INT_CLR` writer - "]
pub type ICM_REG_ICM_CPU_ADDRHOLE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icm_reg_dlock_int_clr(&mut self) -> ICM_REG_DLOCK_INT_CLR_W<'_, ICM_INT_CLR_SPEC> {
        ICM_REG_DLOCK_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_reg_icm_sys_addrhole_int_clr(
        &mut self,
    ) -> ICM_REG_ICM_SYS_ADDRHOLE_INT_CLR_W<'_, ICM_INT_CLR_SPEC> {
        ICM_REG_ICM_SYS_ADDRHOLE_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icm_reg_icm_cpu_addrhole_int_clr(
        &mut self,
    ) -> ICM_REG_ICM_CPU_ADDRHOLE_INT_CLR_W<'_, ICM_INT_CLR_SPEC> {
        ICM_REG_ICM_CPU_ADDRHOLE_INT_CLR_W::new(self, 2)
    }
}
#[doc = "ICM interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_INT_CLR_SPEC;
impl crate::RegisterSpec for ICM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icm_int_clr::W`](W) writer structure"]
impl crate::Writable for ICM_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_INT_CLR to value 0"]
impl crate::Resettable for ICM_INT_CLR_SPEC {}
