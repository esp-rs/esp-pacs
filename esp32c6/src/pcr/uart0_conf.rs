///Register `UART0_CONF` reader
pub type R = crate::R<UART0_CONF_SPEC>;
///Register `UART0_CONF` writer
pub type W = crate::W<UART0_CONF_SPEC>;
///Field `UART0_CLK_EN` reader - Set 1 to enable uart0 apb clock
pub type UART0_CLK_EN_R = crate::BitReader;
///Field `UART0_CLK_EN` writer - Set 1 to enable uart0 apb clock
pub type UART0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART0_RST_EN` reader - Set 0 to reset uart0 module
pub type UART0_RST_EN_R = crate::BitReader;
///Field `UART0_RST_EN` writer - Set 0 to reset uart0 module
pub type UART0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable uart0 apb clock
    #[inline(always)]
    pub fn uart0_clk_en(&self) -> UART0_CLK_EN_R {
        UART0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset uart0 module
    #[inline(always)]
    pub fn uart0_rst_en(&self) -> UART0_RST_EN_R {
        UART0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0_CONF")
            .field("uart0_clk_en", &self.uart0_clk_en())
            .field("uart0_rst_en", &self.uart0_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable uart0 apb clock
    #[inline(always)]
    #[must_use]
    pub fn uart0_clk_en(&mut self) -> UART0_CLK_EN_W<UART0_CONF_SPEC> {
        UART0_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset uart0 module
    #[inline(always)]
    #[must_use]
    pub fn uart0_rst_en(&mut self) -> UART0_RST_EN_W<UART0_CONF_SPEC> {
        UART0_RST_EN_W::new(self, 1)
    }
}
/**UART0 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UART0_CONF_SPEC;
impl crate::RegisterSpec for UART0_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`uart0_conf::R`](R) reader structure
impl crate::Readable for UART0_CONF_SPEC {}
///`write(|w| ..)` method takes [`uart0_conf::W`](W) writer structure
impl crate::Writable for UART0_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets UART0_CONF to value 0x01
impl crate::Resettable for UART0_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
