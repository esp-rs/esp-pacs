///Register `HMAC_CONF` reader
pub type R = crate::R<HMAC_CONF_SPEC>;
///Register `HMAC_CONF` writer
pub type W = crate::W<HMAC_CONF_SPEC>;
///Field `HMAC_CLK_EN` reader - Set 1 to enable hmac clock
pub type HMAC_CLK_EN_R = crate::BitReader;
///Field `HMAC_CLK_EN` writer - Set 1 to enable hmac clock
pub type HMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HMAC_RST_EN` reader - Set 0 to reset hmac module
pub type HMAC_RST_EN_R = crate::BitReader;
///Field `HMAC_RST_EN` writer - Set 0 to reset hmac module
pub type HMAC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable hmac clock
    #[inline(always)]
    pub fn hmac_clk_en(&self) -> HMAC_CLK_EN_R {
        HMAC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset hmac module
    #[inline(always)]
    pub fn hmac_rst_en(&self) -> HMAC_RST_EN_R {
        HMAC_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_CONF")
            .field("hmac_clk_en", &self.hmac_clk_en())
            .field("hmac_rst_en", &self.hmac_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable hmac clock
    #[inline(always)]
    #[must_use]
    pub fn hmac_clk_en(&mut self) -> HMAC_CLK_EN_W<HMAC_CONF_SPEC> {
        HMAC_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset hmac module
    #[inline(always)]
    #[must_use]
    pub fn hmac_rst_en(&mut self) -> HMAC_RST_EN_W<HMAC_CONF_SPEC> {
        HMAC_RST_EN_W::new(self, 1)
    }
}
/**HMAC configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`hmac_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hmac_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HMAC_CONF_SPEC;
impl crate::RegisterSpec for HMAC_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hmac_conf::R`](R) reader structure
impl crate::Readable for HMAC_CONF_SPEC {}
///`write(|w| ..)` method takes [`hmac_conf::W`](W) writer structure
impl crate::Writable for HMAC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HMAC_CONF to value 0x01
impl crate::Resettable for HMAC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
