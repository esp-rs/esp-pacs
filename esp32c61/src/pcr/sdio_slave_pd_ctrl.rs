#[doc = "Register `SDIO_SLAVE_PD_CTRL` reader"]
pub type R = crate::R<SDIO_SLAVE_PD_CTRL_SPEC>;
#[doc = "Register `SDIO_SLAVE_PD_CTRL` writer"]
pub type W = crate::W<SDIO_SLAVE_PD_CTRL_SPEC>;
#[doc = "Field `SDIO_MEM_FORCE_PU` reader - Set this bit to force power down SDIO memory."]
pub type SDIO_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SDIO_MEM_FORCE_PU` writer - Set this bit to force power down SDIO memory."]
pub type SDIO_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_MEM_FORCE_PD` reader - Set this bit to force power up SDIO memory."]
pub type SDIO_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SDIO_MEM_FORCE_PD` writer - Set this bit to force power up SDIO memory."]
pub type SDIO_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power down SDIO memory."]
    #[inline(always)]
    pub fn sdio_mem_force_pu(&self) -> SDIO_MEM_FORCE_PU_R {
        SDIO_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up SDIO memory."]
    #[inline(always)]
    pub fn sdio_mem_force_pd(&self) -> SDIO_MEM_FORCE_PD_R {
        SDIO_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SLAVE_PD_CTRL")
            .field("sdio_mem_force_pu", &self.sdio_mem_force_pu())
            .field("sdio_mem_force_pd", &self.sdio_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power down SDIO memory."]
    #[inline(always)]
    pub fn sdio_mem_force_pu(&mut self) -> SDIO_MEM_FORCE_PU_W<'_, SDIO_SLAVE_PD_CTRL_SPEC> {
        SDIO_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power up SDIO memory."]
    #[inline(always)]
    pub fn sdio_mem_force_pd(&mut self) -> SDIO_MEM_FORCE_PD_W<'_, SDIO_SLAVE_PD_CTRL_SPEC> {
        SDIO_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "LEDC power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_slave_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_slave_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_SLAVE_PD_CTRL_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_slave_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for SDIO_SLAVE_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_slave_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for SDIO_SLAVE_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_SLAVE_PD_CTRL to value 0x02"]
impl crate::Resettable for SDIO_SLAVE_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
