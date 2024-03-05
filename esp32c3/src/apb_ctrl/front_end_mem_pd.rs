#[doc = "Register `FRONT_END_MEM_PD` reader"]
pub type R = crate::R<FRONT_END_MEM_PD_SPEC>;
#[doc = "Register `FRONT_END_MEM_PD` writer"]
pub type W = crate::W<FRONT_END_MEM_PD_SPEC>;
#[doc = "Field `AGC_MEM_FORCE_PU` reader - reg_agc_mem_force_pu"]
pub type AGC_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `AGC_MEM_FORCE_PU` writer - reg_agc_mem_force_pu"]
pub type AGC_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_MEM_FORCE_PD` reader - reg_agc_mem_force_pd"]
pub type AGC_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `AGC_MEM_FORCE_PD` writer - reg_agc_mem_force_pd"]
pub type AGC_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBUS_MEM_FORCE_PU` reader - reg_pbus_mem_force_pu"]
pub type PBUS_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PBUS_MEM_FORCE_PU` writer - reg_pbus_mem_force_pu"]
pub type PBUS_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBUS_MEM_FORCE_PD` reader - reg_pbus_mem_force_pd"]
pub type PBUS_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PBUS_MEM_FORCE_PD` writer - reg_pbus_mem_force_pd"]
pub type PBUS_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC_MEM_FORCE_PU` reader - reg_dc_mem_force_pu"]
pub type DC_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DC_MEM_FORCE_PU` writer - reg_dc_mem_force_pu"]
pub type DC_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC_MEM_FORCE_PD` reader - reg_dc_mem_force_pd"]
pub type DC_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DC_MEM_FORCE_PD` writer - reg_dc_mem_force_pd"]
pub type DC_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_agc_mem_force_pu"]
    #[inline(always)]
    pub fn agc_mem_force_pu(&self) -> AGC_MEM_FORCE_PU_R {
        AGC_MEM_FORCE_PU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_agc_mem_force_pd"]
    #[inline(always)]
    pub fn agc_mem_force_pd(&self) -> AGC_MEM_FORCE_PD_R {
        AGC_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_pbus_mem_force_pu"]
    #[inline(always)]
    pub fn pbus_mem_force_pu(&self) -> PBUS_MEM_FORCE_PU_R {
        PBUS_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_pbus_mem_force_pd"]
    #[inline(always)]
    pub fn pbus_mem_force_pd(&self) -> PBUS_MEM_FORCE_PD_R {
        PBUS_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_dc_mem_force_pu"]
    #[inline(always)]
    pub fn dc_mem_force_pu(&self) -> DC_MEM_FORCE_PU_R {
        DC_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_dc_mem_force_pd"]
    #[inline(always)]
    pub fn dc_mem_force_pd(&self) -> DC_MEM_FORCE_PD_R {
        DC_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRONT_END_MEM_PD")
            .field(
                "agc_mem_force_pu",
                &format_args!("{}", self.agc_mem_force_pu().bit()),
            )
            .field(
                "agc_mem_force_pd",
                &format_args!("{}", self.agc_mem_force_pd().bit()),
            )
            .field(
                "pbus_mem_force_pu",
                &format_args!("{}", self.pbus_mem_force_pu().bit()),
            )
            .field(
                "pbus_mem_force_pd",
                &format_args!("{}", self.pbus_mem_force_pd().bit()),
            )
            .field(
                "dc_mem_force_pu",
                &format_args!("{}", self.dc_mem_force_pu().bit()),
            )
            .field(
                "dc_mem_force_pd",
                &format_args!("{}", self.dc_mem_force_pd().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRONT_END_MEM_PD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_agc_mem_force_pu"]
    #[inline(always)]
    #[must_use]
    pub fn agc_mem_force_pu(&mut self) -> AGC_MEM_FORCE_PU_W<FRONT_END_MEM_PD_SPEC> {
        AGC_MEM_FORCE_PU_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_agc_mem_force_pd"]
    #[inline(always)]
    #[must_use]
    pub fn agc_mem_force_pd(&mut self) -> AGC_MEM_FORCE_PD_W<FRONT_END_MEM_PD_SPEC> {
        AGC_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - reg_pbus_mem_force_pu"]
    #[inline(always)]
    #[must_use]
    pub fn pbus_mem_force_pu(&mut self) -> PBUS_MEM_FORCE_PU_W<FRONT_END_MEM_PD_SPEC> {
        PBUS_MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_pbus_mem_force_pd"]
    #[inline(always)]
    #[must_use]
    pub fn pbus_mem_force_pd(&mut self) -> PBUS_MEM_FORCE_PD_W<FRONT_END_MEM_PD_SPEC> {
        PBUS_MEM_FORCE_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - reg_dc_mem_force_pu"]
    #[inline(always)]
    #[must_use]
    pub fn dc_mem_force_pu(&mut self) -> DC_MEM_FORCE_PU_W<FRONT_END_MEM_PD_SPEC> {
        DC_MEM_FORCE_PU_W::new(self, 4)
    }
    #[doc = "Bit 5 - reg_dc_mem_force_pd"]
    #[inline(always)]
    #[must_use]
    pub fn dc_mem_force_pd(&mut self) -> DC_MEM_FORCE_PD_W<FRONT_END_MEM_PD_SPEC> {
        DC_MEM_FORCE_PD_W::new(self, 5)
    }
}
#[doc = "APB_CTRL_FRONT_END_MEM_PD_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`front_end_mem_pd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`front_end_mem_pd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRONT_END_MEM_PD_SPEC;
impl crate::RegisterSpec for FRONT_END_MEM_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`front_end_mem_pd::R`](R) reader structure"]
impl crate::Readable for FRONT_END_MEM_PD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`front_end_mem_pd::W`](W) writer structure"]
impl crate::Writable for FRONT_END_MEM_PD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRONT_END_MEM_PD to value 0x15"]
impl crate::Resettable for FRONT_END_MEM_PD_SPEC {
    const RESET_VALUE: u32 = 0x15;
}
