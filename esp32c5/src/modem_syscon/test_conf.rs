#[doc = "Register `TEST_CONF` reader"]
pub type R = crate::R<TEST_CONF_SPEC>;
#[doc = "Register `TEST_CONF` writer"]
pub type W = crate::W<TEST_CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_ANT_FORCE_SEL_BT` reader - "]
pub type MODEM_ANT_FORCE_SEL_BT_R = crate::BitReader;
#[doc = "Field `MODEM_ANT_FORCE_SEL_BT` writer - "]
pub type MODEM_ANT_FORCE_SEL_BT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_ANT_FORCE_SEL_WIFI` reader - "]
pub type MODEM_ANT_FORCE_SEL_WIFI_R = crate::BitReader;
#[doc = "Field `MODEM_ANT_FORCE_SEL_WIFI` writer - "]
pub type MODEM_ANT_FORCE_SEL_WIFI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPGA_DEBUG_CLKSWITCH` reader - "]
pub type FPGA_DEBUG_CLKSWITCH_R = crate::BitReader;
#[doc = "Field `FPGA_DEBUG_CLKSWITCH` writer - "]
pub type FPGA_DEBUG_CLKSWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPGA_DEBUG_CLK80` reader - "]
pub type FPGA_DEBUG_CLK80_R = crate::BitReader;
#[doc = "Field `FPGA_DEBUG_CLK80` writer - "]
pub type FPGA_DEBUG_CLK80_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPGA_DEBUG_CLK40` reader - "]
pub type FPGA_DEBUG_CLK40_R = crate::BitReader;
#[doc = "Field `FPGA_DEBUG_CLK40` writer - "]
pub type FPGA_DEBUG_CLK40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPGA_DEBUG_CLK20` reader - "]
pub type FPGA_DEBUG_CLK20_R = crate::BitReader;
#[doc = "Field `FPGA_DEBUG_CLK20` writer - "]
pub type FPGA_DEBUG_CLK20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPGA_DEBUG_CLK10` reader - "]
pub type FPGA_DEBUG_CLK10_R = crate::BitReader;
#[doc = "Field `FPGA_DEBUG_CLK10` writer - "]
pub type FPGA_DEBUG_CLK10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_MEM_MODE_FORCE` reader - "]
pub type MODEM_MEM_MODE_FORCE_R = crate::BitReader;
#[doc = "Field `MODEM_MEM_MODE_FORCE` writer - "]
pub type MODEM_MEM_MODE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modem_ant_force_sel_bt(&self) -> MODEM_ANT_FORCE_SEL_BT_R {
        MODEM_ANT_FORCE_SEL_BT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modem_ant_force_sel_wifi(&self) -> MODEM_ANT_FORCE_SEL_WIFI_R {
        MODEM_ANT_FORCE_SEL_WIFI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fpga_debug_clkswitch(&self) -> FPGA_DEBUG_CLKSWITCH_R {
        FPGA_DEBUG_CLKSWITCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fpga_debug_clk80(&self) -> FPGA_DEBUG_CLK80_R {
        FPGA_DEBUG_CLK80_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fpga_debug_clk40(&self) -> FPGA_DEBUG_CLK40_R {
        FPGA_DEBUG_CLK40_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fpga_debug_clk20(&self) -> FPGA_DEBUG_CLK20_R {
        FPGA_DEBUG_CLK20_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fpga_debug_clk10(&self) -> FPGA_DEBUG_CLK10_R {
        FPGA_DEBUG_CLK10_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn modem_mem_mode_force(&self) -> MODEM_MEM_MODE_FORCE_R {
        MODEM_MEM_MODE_FORCE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_CONF")
            .field("clk_en", &self.clk_en())
            .field("modem_ant_force_sel_bt", &self.modem_ant_force_sel_bt())
            .field("modem_ant_force_sel_wifi", &self.modem_ant_force_sel_wifi())
            .field("fpga_debug_clkswitch", &self.fpga_debug_clkswitch())
            .field("fpga_debug_clk80", &self.fpga_debug_clk80())
            .field("fpga_debug_clk40", &self.fpga_debug_clk40())
            .field("fpga_debug_clk20", &self.fpga_debug_clk20())
            .field("fpga_debug_clk10", &self.fpga_debug_clk10())
            .field("modem_mem_mode_force", &self.modem_mem_mode_force())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, TEST_CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modem_ant_force_sel_bt(&mut self) -> MODEM_ANT_FORCE_SEL_BT_W<'_, TEST_CONF_SPEC> {
        MODEM_ANT_FORCE_SEL_BT_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modem_ant_force_sel_wifi(&mut self) -> MODEM_ANT_FORCE_SEL_WIFI_W<'_, TEST_CONF_SPEC> {
        MODEM_ANT_FORCE_SEL_WIFI_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fpga_debug_clkswitch(&mut self) -> FPGA_DEBUG_CLKSWITCH_W<'_, TEST_CONF_SPEC> {
        FPGA_DEBUG_CLKSWITCH_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fpga_debug_clk80(&mut self) -> FPGA_DEBUG_CLK80_W<'_, TEST_CONF_SPEC> {
        FPGA_DEBUG_CLK80_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fpga_debug_clk40(&mut self) -> FPGA_DEBUG_CLK40_W<'_, TEST_CONF_SPEC> {
        FPGA_DEBUG_CLK40_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fpga_debug_clk20(&mut self) -> FPGA_DEBUG_CLK20_W<'_, TEST_CONF_SPEC> {
        FPGA_DEBUG_CLK20_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fpga_debug_clk10(&mut self) -> FPGA_DEBUG_CLK10_W<'_, TEST_CONF_SPEC> {
        FPGA_DEBUG_CLK10_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn modem_mem_mode_force(&mut self) -> MODEM_MEM_MODE_FORCE_W<'_, TEST_CONF_SPEC> {
        MODEM_MEM_MODE_FORCE_W::new(self, 8)
    }
}
#[doc = "TEST_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`test_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_CONF_SPEC;
impl crate::RegisterSpec for TEST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_conf::R`](R) reader structure"]
impl crate::Readable for TEST_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test_conf::W`](W) writer structure"]
impl crate::Writable for TEST_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST_CONF to value 0x0100"]
impl crate::Resettable for TEST_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
