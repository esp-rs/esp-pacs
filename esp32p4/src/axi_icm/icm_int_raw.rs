#[doc = "Register `ICM_INT_RAW` reader"]
pub type R = crate::R<ICM_INT_RAW_SPEC>;
#[doc = "Register `ICM_INT_RAW` writer"]
pub type W = crate::W<ICM_INT_RAW_SPEC>;
#[doc = "Field `ICM_REG_DLOCK_INT_RAW` reader - "]
pub type ICM_REG_DLOCK_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICM_REG_DLOCK_INT_RAW` writer - "]
pub type ICM_REG_DLOCK_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW` reader - "]
pub type ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW` writer - "]
pub type ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW` reader - "]
pub type ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW` writer - "]
pub type ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icm_reg_dlock_int_raw(&self) -> ICM_REG_DLOCK_INT_RAW_R {
        ICM_REG_DLOCK_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_reg_icm_sys_addrhole_int_raw(&self) -> ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW_R {
        ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icm_reg_icm_cpu_addrhole_int_raw(&self) -> ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW_R {
        ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_INT_RAW")
            .field("icm_reg_dlock_int_raw", &self.icm_reg_dlock_int_raw())
            .field(
                "icm_reg_icm_sys_addrhole_int_raw",
                &self.icm_reg_icm_sys_addrhole_int_raw(),
            )
            .field(
                "icm_reg_icm_cpu_addrhole_int_raw",
                &self.icm_reg_icm_cpu_addrhole_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icm_reg_dlock_int_raw(&mut self) -> ICM_REG_DLOCK_INT_RAW_W<'_, ICM_INT_RAW_SPEC> {
        ICM_REG_DLOCK_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_reg_icm_sys_addrhole_int_raw(
        &mut self,
    ) -> ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW_W<'_, ICM_INT_RAW_SPEC> {
        ICM_REG_ICM_SYS_ADDRHOLE_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icm_reg_icm_cpu_addrhole_int_raw(
        &mut self,
    ) -> ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW_W<'_, ICM_INT_RAW_SPEC> {
        ICM_REG_ICM_CPU_ADDRHOLE_INT_RAW_W::new(self, 2)
    }
}
#[doc = "ICM interrupt raw\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_INT_RAW_SPEC;
impl crate::RegisterSpec for ICM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_int_raw::R`](R) reader structure"]
impl crate::Readable for ICM_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_int_raw::W`](W) writer structure"]
impl crate::Writable for ICM_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_INT_RAW to value 0"]
impl crate::Resettable for ICM_INT_RAW_SPEC {}
