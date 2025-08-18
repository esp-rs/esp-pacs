#[doc = "Register `PSRAM_MEM_MONITOR_CONF` reader"]
pub type R = crate::R<PSRAM_MEM_MONITOR_CONF_SPEC>;
#[doc = "Register `PSRAM_MEM_MONITOR_CONF` writer"]
pub type W = crate::W<PSRAM_MEM_MONITOR_CONF_SPEC>;
#[doc = "Field `PSRAM_MEM_MONITOR_CLK_EN` reader - Set 1 to enable psram_mem_monitor clock"]
pub type PSRAM_MEM_MONITOR_CLK_EN_R = crate::BitReader;
#[doc = "Field `PSRAM_MEM_MONITOR_CLK_EN` writer - Set 1 to enable psram_mem_monitor clock"]
pub type PSRAM_MEM_MONITOR_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_MEM_MONITOR_RST_EN` reader - Set 1 to reset psram_mem_monitor module"]
pub type PSRAM_MEM_MONITOR_RST_EN_R = crate::BitReader;
#[doc = "Field `PSRAM_MEM_MONITOR_RST_EN` writer - Set 1 to reset psram_mem_monitor module"]
pub type PSRAM_MEM_MONITOR_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_MEM_MONITOR_READY` reader - Query this field after reset psram_mem_monitor module"]
pub type PSRAM_MEM_MONITOR_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable psram_mem_monitor clock"]
    #[inline(always)]
    pub fn psram_mem_monitor_clk_en(&self) -> PSRAM_MEM_MONITOR_CLK_EN_R {
        PSRAM_MEM_MONITOR_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset psram_mem_monitor module"]
    #[inline(always)]
    pub fn psram_mem_monitor_rst_en(&self) -> PSRAM_MEM_MONITOR_RST_EN_R {
        PSRAM_MEM_MONITOR_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset psram_mem_monitor module"]
    #[inline(always)]
    pub fn psram_mem_monitor_ready(&self) -> PSRAM_MEM_MONITOR_READY_R {
        PSRAM_MEM_MONITOR_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM_MEM_MONITOR_CONF")
            .field("psram_mem_monitor_clk_en", &self.psram_mem_monitor_clk_en())
            .field("psram_mem_monitor_rst_en", &self.psram_mem_monitor_rst_en())
            .field("psram_mem_monitor_ready", &self.psram_mem_monitor_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable psram_mem_monitor clock"]
    #[inline(always)]
    pub fn psram_mem_monitor_clk_en(
        &mut self,
    ) -> PSRAM_MEM_MONITOR_CLK_EN_W<'_, PSRAM_MEM_MONITOR_CONF_SPEC> {
        PSRAM_MEM_MONITOR_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset psram_mem_monitor module"]
    #[inline(always)]
    pub fn psram_mem_monitor_rst_en(
        &mut self,
    ) -> PSRAM_MEM_MONITOR_RST_EN_W<'_, PSRAM_MEM_MONITOR_CONF_SPEC> {
        PSRAM_MEM_MONITOR_RST_EN_W::new(self, 1)
    }
}
#[doc = "PSRAM_MEM_MONITOR configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_mem_monitor_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_mem_monitor_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_MEM_MONITOR_CONF_SPEC;
impl crate::RegisterSpec for PSRAM_MEM_MONITOR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_mem_monitor_conf::R`](R) reader structure"]
impl crate::Readable for PSRAM_MEM_MONITOR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_mem_monitor_conf::W`](W) writer structure"]
impl crate::Writable for PSRAM_MEM_MONITOR_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_MEM_MONITOR_CONF to value 0x05"]
impl crate::Resettable for PSRAM_MEM_MONITOR_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
