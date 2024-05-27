///Register `ETM_CONF` reader
pub type R = crate::R<ETM_CONF_SPEC>;
///Register `ETM_CONF` writer
pub type W = crate::W<ETM_CONF_SPEC>;
///Field `ETM_CLK_EN` reader - Set 1 to enable etm clock
pub type ETM_CLK_EN_R = crate::BitReader;
///Field `ETM_CLK_EN` writer - Set 1 to enable etm clock
pub type ETM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETM_RST_EN` reader - Set 0 to reset etm module
pub type ETM_RST_EN_R = crate::BitReader;
///Field `ETM_RST_EN` writer - Set 0 to reset etm module
pub type ETM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETM_READY` reader - Query this field after reset etm module
pub type ETM_READY_R = crate::BitReader;
impl R {
    ///Bit 0 - Set 1 to enable etm clock
    #[inline(always)]
    pub fn etm_clk_en(&self) -> ETM_CLK_EN_R {
        ETM_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset etm module
    #[inline(always)]
    pub fn etm_rst_en(&self) -> ETM_RST_EN_R {
        ETM_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Query this field after reset etm module
    #[inline(always)]
    pub fn etm_ready(&self) -> ETM_READY_R {
        ETM_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CONF")
            .field("etm_clk_en", &self.etm_clk_en())
            .field("etm_rst_en", &self.etm_rst_en())
            .field("etm_ready", &self.etm_ready())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable etm clock
    #[inline(always)]
    #[must_use]
    pub fn etm_clk_en(&mut self) -> ETM_CLK_EN_W<ETM_CONF_SPEC> {
        ETM_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset etm module
    #[inline(always)]
    #[must_use]
    pub fn etm_rst_en(&mut self) -> ETM_RST_EN_W<ETM_CONF_SPEC> {
        ETM_RST_EN_W::new(self, 1)
    }
}
/**ETM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`etm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ETM_CONF_SPEC;
impl crate::RegisterSpec for ETM_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`etm_conf::R`](R) reader structure
impl crate::Readable for ETM_CONF_SPEC {}
///`write(|w| ..)` method takes [`etm_conf::W`](W) writer structure
impl crate::Writable for ETM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETM_CONF to value 0x05
impl crate::Resettable for ETM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
