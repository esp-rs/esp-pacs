#[doc = "Register `UART2_CONF` reader"]
pub type R = crate::R<UART2_CONF_SPEC>;
#[doc = "Register `UART2_CONF` writer"]
pub type W = crate::W<UART2_CONF_SPEC>;
#[doc = "Field `UART2_CLK_EN` reader - Set 1 to enable uart2 apb clock"]
pub type UART2_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART2_CLK_EN` writer - Set 1 to enable uart2 apb clock"]
pub type UART2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_RST_EN` reader - Set 1 to reset uart2 module"]
pub type UART2_RST_EN_R = crate::BitReader;
#[doc = "Field `UART2_RST_EN` writer - Set 1 to reset uart2 module"]
pub type UART2_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_READY` reader - Query this field after reset uart2 module"]
pub type UART2_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable uart2 apb clock"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset uart2 module"]
    #[inline(always)]
    pub fn uart2_rst_en(&self) -> UART2_RST_EN_R {
        UART2_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset uart2 module"]
    #[inline(always)]
    pub fn uart2_ready(&self) -> UART2_READY_R {
        UART2_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART2_CONF")
            .field("uart2_clk_en", &self.uart2_clk_en())
            .field("uart2_rst_en", &self.uart2_rst_en())
            .field("uart2_ready", &self.uart2_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable uart2 apb clock"]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W<UART2_CONF_SPEC> {
        UART2_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset uart2 module"]
    #[inline(always)]
    pub fn uart2_rst_en(&mut self) -> UART2_RST_EN_W<UART2_CONF_SPEC> {
        UART2_RST_EN_W::new(self, 1)
    }
}
#[doc = "UART2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART2_CONF_SPEC;
impl crate::RegisterSpec for UART2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart2_conf::R`](R) reader structure"]
impl crate::Readable for UART2_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart2_conf::W`](W) writer structure"]
impl crate::Writable for UART2_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART2_CONF to value 0x05"]
impl crate::Resettable for UART2_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
