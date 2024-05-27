///Register `RMT_CONF` reader
pub type R = crate::R<RMT_CONF_SPEC>;
///Register `RMT_CONF` writer
pub type W = crate::W<RMT_CONF_SPEC>;
///Field `RMT_CLK_EN` reader - Set 1 to enable rmt apb clock
pub type RMT_CLK_EN_R = crate::BitReader;
///Field `RMT_CLK_EN` writer - Set 1 to enable rmt apb clock
pub type RMT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMT_RST_EN` reader - Set 0 to reset rmt module
pub type RMT_RST_EN_R = crate::BitReader;
///Field `RMT_RST_EN` writer - Set 0 to reset rmt module
pub type RMT_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMT_READY` reader - Query this field after reset rmt module
pub type RMT_READY_R = crate::BitReader;
impl R {
    ///Bit 0 - Set 1 to enable rmt apb clock
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset rmt module
    #[inline(always)]
    pub fn rmt_rst_en(&self) -> RMT_RST_EN_R {
        RMT_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Query this field after reset rmt module
    #[inline(always)]
    pub fn rmt_ready(&self) -> RMT_READY_R {
        RMT_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT_CONF")
            .field("rmt_clk_en", &self.rmt_clk_en())
            .field("rmt_rst_en", &self.rmt_rst_en())
            .field("rmt_ready", &self.rmt_ready())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable rmt apb clock
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W<RMT_CONF_SPEC> {
        RMT_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset rmt module
    #[inline(always)]
    #[must_use]
    pub fn rmt_rst_en(&mut self) -> RMT_RST_EN_W<RMT_CONF_SPEC> {
        RMT_RST_EN_W::new(self, 1)
    }
}
/**RMT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rmt_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMT_CONF_SPEC;
impl crate::RegisterSpec for RMT_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rmt_conf::R`](R) reader structure
impl crate::Readable for RMT_CONF_SPEC {}
///`write(|w| ..)` method takes [`rmt_conf::W`](W) writer structure
impl crate::Writable for RMT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RMT_CONF to value 0x05
impl crate::Resettable for RMT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
