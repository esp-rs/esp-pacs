#[doc = "Register `PD_CONF` reader"]
pub type R = crate::R<PD_CONF_SPEC>;
#[doc = "Register `PD_CONF` writer"]
pub type W = crate::W<PD_CONF_SPEC>;
#[doc = "Field `RAM_FORCE_PD` reader - Force power down signal to RAM. 0: force RAM power up; 1: only when CRYPTO_DMA_RAM_FORCE_PU is 0, power down RAM."]
pub type RAM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `RAM_FORCE_PD` writer - Force power down signal to RAM. 0: force RAM power up; 1: only when CRYPTO_DMA_RAM_FORCE_PU is 0, power down RAM."]
pub type RAM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_FORCE_PU` reader - Force power up signal to RAM. 0: only when CRYPTO_DMA_RAM_FORCE_PD is 1, power down RAM; 1: force RAM power up."]
pub type RAM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `RAM_FORCE_PU` writer - Force power up signal to RAM. 0: only when CRYPTO_DMA_RAM_FORCE_PD is 1, power down RAM; 1: force RAM power up."]
pub type RAM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_CLK_FO` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type RAM_CLK_FO_R = crate::BitReader;
#[doc = "Field `RAM_CLK_FO` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type RAM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Force power down signal to RAM. 0: force RAM power up; 1: only when CRYPTO_DMA_RAM_FORCE_PU is 0, power down RAM."]
    #[inline(always)]
    pub fn ram_force_pd(&self) -> RAM_FORCE_PD_R {
        RAM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force power up signal to RAM. 0: only when CRYPTO_DMA_RAM_FORCE_PD is 1, power down RAM; 1: force RAM power up."]
    #[inline(always)]
    pub fn ram_force_pu(&self) -> RAM_FORCE_PU_R {
        RAM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn ram_clk_fo(&self) -> RAM_CLK_FO_R {
        RAM_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PD_CONF")
            .field("ram_force_pd", &self.ram_force_pd())
            .field("ram_force_pu", &self.ram_force_pu())
            .field("ram_clk_fo", &self.ram_clk_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Force power down signal to RAM. 0: force RAM power up; 1: only when CRYPTO_DMA_RAM_FORCE_PU is 0, power down RAM."]
    #[inline(always)]
    pub fn ram_force_pd(&mut self) -> RAM_FORCE_PD_W<PD_CONF_SPEC> {
        RAM_FORCE_PD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Force power up signal to RAM. 0: only when CRYPTO_DMA_RAM_FORCE_PD is 1, power down RAM; 1: force RAM power up."]
    #[inline(always)]
    pub fn ram_force_pu(&mut self) -> RAM_FORCE_PU_W<PD_CONF_SPEC> {
        RAM_FORCE_PU_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn ram_clk_fo(&mut self) -> RAM_CLK_FO_W<PD_CONF_SPEC> {
        RAM_CLK_FO_W::new(self, 6)
    }
}
#[doc = "Power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_CONF_SPEC;
impl crate::RegisterSpec for PD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_conf::R`](R) reader structure"]
impl crate::Readable for PD_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_conf::W`](W) writer structure"]
impl crate::Writable for PD_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD_CONF to value 0x20"]
impl crate::Resettable for PD_CONF_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
