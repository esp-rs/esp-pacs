#[doc = "Register `UART1_CONF` reader"]
pub type R = crate::R<UART1_CONF_SPEC>;
#[doc = "Register `UART1_CONF` writer"]
pub type W = crate::W<UART1_CONF_SPEC>;
#[doc = "Field `UART1_CLK_EN` reader - Set 1 to enable uart1 apb clock"]
pub type UART1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART1_CLK_EN` writer - Set 1 to enable uart1 apb clock"]
pub type UART1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RST_EN` reader - Set 0 to reset uart1 module"]
pub type UART1_RST_EN_R = crate::BitReader;
#[doc = "Field `UART1_RST_EN` writer - Set 0 to reset uart1 module"]
pub type UART1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_READY` reader - Query this field after reset uart1 module"]
pub type UART1_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable uart1 apb clock"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset uart1 module"]
    #[inline(always)]
    pub fn uart1_rst_en(&self) -> UART1_RST_EN_R {
        UART1_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset uart1 module"]
    #[inline(always)]
    pub fn uart1_ready(&self) -> UART1_READY_R {
        UART1_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1_CONF")
            .field("uart1_clk_en", &self.uart1_clk_en())
            .field("uart1_rst_en", &self.uart1_rst_en())
            .field("uart1_ready", &self.uart1_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable uart1 apb clock"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W<UART1_CONF_SPEC> {
        UART1_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset uart1 module"]
    #[inline(always)]
    pub fn uart1_rst_en(&mut self) -> UART1_RST_EN_W<UART1_CONF_SPEC> {
        UART1_RST_EN_W::new(self, 1)
    }
}
#[doc = "UART1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART1_CONF_SPEC;
impl crate::RegisterSpec for UART1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_conf::R`](R) reader structure"]
impl crate::Readable for UART1_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart1_conf::W`](W) writer structure"]
impl crate::Writable for UART1_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_CONF to value 0x05"]
impl crate::Resettable for UART1_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
