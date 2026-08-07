#[doc = "Register `CORE1_CONF` reader"]
pub type R = crate::R<CORE1_CONF_SPEC>;
#[doc = "Register `CORE1_CONF` writer"]
pub type W = crate::W<CORE1_CONF_SPEC>;
#[doc = "Field `CORE1_CLK_EN` reader - Set 1 to enable core1 clock"]
pub type CORE1_CLK_EN_R = crate::BitReader;
#[doc = "Field `CORE1_CLK_EN` writer - Set 1 to enable core1 clock"]
pub type CORE1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_RST_EN` reader - Set 1 to reset core1 module"]
pub type CORE1_RST_EN_R = crate::BitReader;
#[doc = "Field `CORE1_RST_EN` writer - Set 1 to reset core1 module"]
pub type CORE1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable core1 clock"]
    #[inline(always)]
    pub fn core1_clk_en(&self) -> CORE1_CLK_EN_R {
        CORE1_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset core1 module"]
    #[inline(always)]
    pub fn core1_rst_en(&self) -> CORE1_RST_EN_R {
        CORE1_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE1_CONF")
            .field("core1_clk_en", &self.core1_clk_en())
            .field("core1_rst_en", &self.core1_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable core1 clock"]
    #[inline(always)]
    pub fn core1_clk_en(&mut self) -> CORE1_CLK_EN_W<'_, CORE1_CONF_SPEC> {
        CORE1_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset core1 module"]
    #[inline(always)]
    pub fn core1_rst_en(&mut self) -> CORE1_RST_EN_W<'_, CORE1_CONF_SPEC> {
        CORE1_RST_EN_W::new(self, 1)
    }
}
#[doc = "USB_OTG configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE1_CONF_SPEC;
impl crate::RegisterSpec for CORE1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core1_conf::R`](R) reader structure"]
impl crate::Readable for CORE1_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core1_conf::W`](W) writer structure"]
impl crate::Writable for CORE1_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE1_CONF to value 0x02"]
impl crate::Resettable for CORE1_CONF_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
