///Register `TRACE_CONF` reader
pub type R = crate::R<TRACE_CONF_SPEC>;
///Register `TRACE_CONF` writer
pub type W = crate::W<TRACE_CONF_SPEC>;
///Field `TRACE_CLK_EN` reader - Set 1 to enable trace clock
pub type TRACE_CLK_EN_R = crate::BitReader;
///Field `TRACE_CLK_EN` writer - Set 1 to enable trace clock
pub type TRACE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACE_RST_EN` reader - Set 0 to reset trace module
pub type TRACE_RST_EN_R = crate::BitReader;
///Field `TRACE_RST_EN` writer - Set 0 to reset trace module
pub type TRACE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable trace clock
    #[inline(always)]
    pub fn trace_clk_en(&self) -> TRACE_CLK_EN_R {
        TRACE_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset trace module
    #[inline(always)]
    pub fn trace_rst_en(&self) -> TRACE_RST_EN_R {
        TRACE_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_CONF")
            .field("trace_clk_en", &self.trace_clk_en())
            .field("trace_rst_en", &self.trace_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable trace clock
    #[inline(always)]
    #[must_use]
    pub fn trace_clk_en(&mut self) -> TRACE_CLK_EN_W<TRACE_CONF_SPEC> {
        TRACE_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset trace module
    #[inline(always)]
    #[must_use]
    pub fn trace_rst_en(&mut self) -> TRACE_RST_EN_W<TRACE_CONF_SPEC> {
        TRACE_RST_EN_W::new(self, 1)
    }
}
/**TRACE configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`trace_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TRACE_CONF_SPEC;
impl crate::RegisterSpec for TRACE_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`trace_conf::R`](R) reader structure
impl crate::Readable for TRACE_CONF_SPEC {}
///`write(|w| ..)` method takes [`trace_conf::W`](W) writer structure
impl crate::Writable for TRACE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TRACE_CONF to value 0x01
impl crate::Resettable for TRACE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
