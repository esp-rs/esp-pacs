#[doc = "Register `CACHE_CONF` reader"]
pub type R = crate::R<CACHE_CONF_SPEC>;
#[doc = "Register `CACHE_CONF` writer"]
pub type W = crate::W<CACHE_CONF_SPEC>;
#[doc = "Field `ICACHE0_CLK_EN` reader - Set 1 to enable icache0 clock"]
pub type ICACHE0_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_CLK_EN` writer - Set 1 to enable icache0 clock"]
pub type ICACHE0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_CLK_EN` reader - Set 1 to enable icache1 clock"]
pub type ICACHE1_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICACHE1_CLK_EN` writer - Set 1 to enable icache1 clock"]
pub type ICACHE1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_CLK_EN` reader - Set 1 to enable dcache clock"]
pub type DCACHE_CLK_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_CLK_EN` writer - Set 1 to enable dcache clock"]
pub type DCACHE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0_RST_EN` reader - Set 1 to reset icache0 module"]
pub type ICACHE0_RST_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_RST_EN` writer - Set 1 to reset icache0 module"]
pub type ICACHE0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_RST_EN` reader - Set 1 to reset icache1 module"]
pub type ICACHE1_RST_EN_R = crate::BitReader;
#[doc = "Field `ICACHE1_RST_EN` writer - Set 1 to reset icache1 module"]
pub type ICACHE1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_RST_EN` reader - Set 1 to reset dcache module"]
pub type DCACHE_RST_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_RST_EN` writer - Set 1 to reset dcache module"]
pub type DCACHE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_RST_EN` reader - Set 1 to reset total cache module"]
pub type CACHE_RST_EN_R = crate::BitReader;
#[doc = "Field `CACHE_RST_EN` writer - Set 1 to reset total cache module"]
pub type CACHE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable icache0 clock"]
    #[inline(always)]
    pub fn icache0_clk_en(&self) -> ICACHE0_CLK_EN_R {
        ICACHE0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable icache1 clock"]
    #[inline(always)]
    pub fn icache1_clk_en(&self) -> ICACHE1_CLK_EN_R {
        ICACHE1_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable dcache clock"]
    #[inline(always)]
    pub fn dcache_clk_en(&self) -> DCACHE_CLK_EN_R {
        DCACHE_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to reset icache0 module"]
    #[inline(always)]
    pub fn icache0_rst_en(&self) -> ICACHE0_RST_EN_R {
        ICACHE0_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to reset icache1 module"]
    #[inline(always)]
    pub fn icache1_rst_en(&self) -> ICACHE1_RST_EN_R {
        ICACHE1_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to reset dcache module"]
    #[inline(always)]
    pub fn dcache_rst_en(&self) -> DCACHE_RST_EN_R {
        DCACHE_RST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to reset total cache module"]
    #[inline(always)]
    pub fn cache_rst_en(&self) -> CACHE_RST_EN_R {
        CACHE_RST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CONF")
            .field("icache0_clk_en", &self.icache0_clk_en())
            .field("icache1_clk_en", &self.icache1_clk_en())
            .field("dcache_clk_en", &self.dcache_clk_en())
            .field("icache0_rst_en", &self.icache0_rst_en())
            .field("icache1_rst_en", &self.icache1_rst_en())
            .field("dcache_rst_en", &self.dcache_rst_en())
            .field("cache_rst_en", &self.cache_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable icache0 clock"]
    #[inline(always)]
    pub fn icache0_clk_en(&mut self) -> ICACHE0_CLK_EN_W<'_, CACHE_CONF_SPEC> {
        ICACHE0_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to enable icache1 clock"]
    #[inline(always)]
    pub fn icache1_clk_en(&mut self) -> ICACHE1_CLK_EN_W<'_, CACHE_CONF_SPEC> {
        ICACHE1_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to enable dcache clock"]
    #[inline(always)]
    pub fn dcache_clk_en(&mut self) -> DCACHE_CLK_EN_W<'_, CACHE_CONF_SPEC> {
        DCACHE_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to reset icache0 module"]
    #[inline(always)]
    pub fn icache0_rst_en(&mut self) -> ICACHE0_RST_EN_W<'_, CACHE_CONF_SPEC> {
        ICACHE0_RST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set 1 to reset icache1 module"]
    #[inline(always)]
    pub fn icache1_rst_en(&mut self) -> ICACHE1_RST_EN_W<'_, CACHE_CONF_SPEC> {
        ICACHE1_RST_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set 1 to reset dcache module"]
    #[inline(always)]
    pub fn dcache_rst_en(&mut self) -> DCACHE_RST_EN_W<'_, CACHE_CONF_SPEC> {
        DCACHE_RST_EN_W::new(self, 5)
    }
    #[doc = "Bit 8 - Set 1 to reset total cache module"]
    #[inline(always)]
    pub fn cache_rst_en(&mut self) -> CACHE_RST_EN_W<'_, CACHE_CONF_SPEC> {
        CACHE_RST_EN_W::new(self, 8)
    }
}
#[doc = "CACHE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CONF_SPEC;
impl crate::RegisterSpec for CACHE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_conf::W`](W) writer structure"]
impl crate::Writable for CACHE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_CONF to value 0x07"]
impl crate::Resettable for CACHE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
