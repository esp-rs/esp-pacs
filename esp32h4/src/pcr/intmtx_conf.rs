#[doc = "Register `INTMTX_CONF` reader"]
pub type R = crate::R<INTMTX_CONF_SPEC>;
#[doc = "Register `INTMTX_CONF` writer"]
pub type W = crate::W<INTMTX_CONF_SPEC>;
#[doc = "Field `INTMTX_CORE0_CLK_EN` reader - Set 1 to enable core0 intmtx clock"]
pub type INTMTX_CORE0_CLK_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_CORE0_CLK_EN` writer - Set 1 to enable core0 intmtx clock"]
pub type INTMTX_CORE0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMTX_CORE0_RST_EN` reader - Set 1 to reset core0 intmtx module"]
pub type INTMTX_CORE0_RST_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_CORE0_RST_EN` writer - Set 1 to reset core0 intmtx module"]
pub type INTMTX_CORE0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMTX_CORE1_CLK_EN` reader - Set 1 to enable core1 intmtx clock"]
pub type INTMTX_CORE1_CLK_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_CORE1_CLK_EN` writer - Set 1 to enable core1 intmtx clock"]
pub type INTMTX_CORE1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMTX_CORE1_RST_EN` reader - Set 1 to reset core1 intmtx module"]
pub type INTMTX_CORE1_RST_EN_R = crate::BitReader;
#[doc = "Field `INTMTX_CORE1_RST_EN` writer - Set 1 to reset core1 intmtx module"]
pub type INTMTX_CORE1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMTX_CORE0_READY` reader - Query this field after reset intmtx module"]
pub type INTMTX_CORE0_READY_R = crate::BitReader;
#[doc = "Field `INTMTX_CORE1_READY` reader - Query this field after reset intmtx module"]
pub type INTMTX_CORE1_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable core0 intmtx clock"]
    #[inline(always)]
    pub fn intmtx_core0_clk_en(&self) -> INTMTX_CORE0_CLK_EN_R {
        INTMTX_CORE0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset core0 intmtx module"]
    #[inline(always)]
    pub fn intmtx_core0_rst_en(&self) -> INTMTX_CORE0_RST_EN_R {
        INTMTX_CORE0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable core1 intmtx clock"]
    #[inline(always)]
    pub fn intmtx_core1_clk_en(&self) -> INTMTX_CORE1_CLK_EN_R {
        INTMTX_CORE1_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to reset core1 intmtx module"]
    #[inline(always)]
    pub fn intmtx_core1_rst_en(&self) -> INTMTX_CORE1_RST_EN_R {
        INTMTX_CORE1_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Query this field after reset intmtx module"]
    #[inline(always)]
    pub fn intmtx_core0_ready(&self) -> INTMTX_CORE0_READY_R {
        INTMTX_CORE0_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Query this field after reset intmtx module"]
    #[inline(always)]
    pub fn intmtx_core1_ready(&self) -> INTMTX_CORE1_READY_R {
        INTMTX_CORE1_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMTX_CONF")
            .field("intmtx_core0_clk_en", &self.intmtx_core0_clk_en())
            .field("intmtx_core0_rst_en", &self.intmtx_core0_rst_en())
            .field("intmtx_core1_clk_en", &self.intmtx_core1_clk_en())
            .field("intmtx_core1_rst_en", &self.intmtx_core1_rst_en())
            .field("intmtx_core0_ready", &self.intmtx_core0_ready())
            .field("intmtx_core1_ready", &self.intmtx_core1_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable core0 intmtx clock"]
    #[inline(always)]
    pub fn intmtx_core0_clk_en(&mut self) -> INTMTX_CORE0_CLK_EN_W<'_, INTMTX_CONF_SPEC> {
        INTMTX_CORE0_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset core0 intmtx module"]
    #[inline(always)]
    pub fn intmtx_core0_rst_en(&mut self) -> INTMTX_CORE0_RST_EN_W<'_, INTMTX_CONF_SPEC> {
        INTMTX_CORE0_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to enable core1 intmtx clock"]
    #[inline(always)]
    pub fn intmtx_core1_clk_en(&mut self) -> INTMTX_CORE1_CLK_EN_W<'_, INTMTX_CONF_SPEC> {
        INTMTX_CORE1_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to reset core1 intmtx module"]
    #[inline(always)]
    pub fn intmtx_core1_rst_en(&mut self) -> INTMTX_CORE1_RST_EN_W<'_, INTMTX_CONF_SPEC> {
        INTMTX_CORE1_RST_EN_W::new(self, 3)
    }
}
#[doc = "INTMTX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmtx_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmtx_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMTX_CONF_SPEC;
impl crate::RegisterSpec for INTMTX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmtx_conf::R`](R) reader structure"]
impl crate::Readable for INTMTX_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmtx_conf::W`](W) writer structure"]
impl crate::Writable for INTMTX_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMTX_CONF to value 0x39"]
impl crate::Resettable for INTMTX_CONF_SPEC {
    const RESET_VALUE: u32 = 0x39;
}
