#[doc = "Register `HPCORE_0_PD_CTRL` reader"]
pub type R = crate::R<HPCORE_0_PD_CTRL_SPEC>;
#[doc = "Register `HPCORE_0_PD_CTRL` writer"]
pub type W = crate::W<HPCORE_0_PD_CTRL_SPEC>;
#[doc = "Field `HPCORE_0_MEM_FORCE_PU` reader - Set this bit to force power up HP CORE0 memory."]
pub type HPCORE_0_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `HPCORE_0_MEM_FORCE_PU` writer - Set this bit to force power up HP CORE0 memory."]
pub type HPCORE_0_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE_0_MEM_FORCE_PD` reader - Set this bit to force power down HP CORE0 memory."]
pub type HPCORE_0_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `HPCORE_0_MEM_FORCE_PD` writer - Set this bit to force power down HP CORE0 memory."]
pub type HPCORE_0_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power up HP CORE0 memory."]
    #[inline(always)]
    pub fn hpcore_0_mem_force_pu(&self) -> HPCORE_0_MEM_FORCE_PU_R {
        HPCORE_0_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power down HP CORE0 memory."]
    #[inline(always)]
    pub fn hpcore_0_mem_force_pd(&self) -> HPCORE_0_MEM_FORCE_PD_R {
        HPCORE_0_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPCORE_0_PD_CTRL")
            .field("hpcore_0_mem_force_pu", &self.hpcore_0_mem_force_pu())
            .field("hpcore_0_mem_force_pd", &self.hpcore_0_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power up HP CORE0 memory."]
    #[inline(always)]
    pub fn hpcore_0_mem_force_pu(&mut self) -> HPCORE_0_MEM_FORCE_PU_W<HPCORE_0_PD_CTRL_SPEC> {
        HPCORE_0_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power down HP CORE0 memory."]
    #[inline(always)]
    pub fn hpcore_0_mem_force_pd(&mut self) -> HPCORE_0_MEM_FORCE_PD_W<HPCORE_0_PD_CTRL_SPEC> {
        HPCORE_0_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "HP CORE0 power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_0_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_0_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCORE_0_PD_CTRL_SPEC;
impl crate::RegisterSpec for HPCORE_0_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcore_0_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for HPCORE_0_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcore_0_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for HPCORE_0_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPCORE_0_PD_CTRL to value 0x02"]
impl crate::Resettable for HPCORE_0_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
