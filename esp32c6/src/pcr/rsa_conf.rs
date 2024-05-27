///Register `RSA_CONF` reader
pub type R = crate::R<RSA_CONF_SPEC>;
///Register `RSA_CONF` writer
pub type W = crate::W<RSA_CONF_SPEC>;
///Field `RSA_CLK_EN` reader - Set 1 to enable rsa clock
pub type RSA_CLK_EN_R = crate::BitReader;
///Field `RSA_CLK_EN` writer - Set 1 to enable rsa clock
pub type RSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSA_RST_EN` reader - Set 0 to reset rsa module
pub type RSA_RST_EN_R = crate::BitReader;
///Field `RSA_RST_EN` writer - Set 0 to reset rsa module
pub type RSA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable rsa clock
    #[inline(always)]
    pub fn rsa_clk_en(&self) -> RSA_CLK_EN_R {
        RSA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset rsa module
    #[inline(always)]
    pub fn rsa_rst_en(&self) -> RSA_RST_EN_R {
        RSA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA_CONF")
            .field("rsa_clk_en", &self.rsa_clk_en())
            .field("rsa_rst_en", &self.rsa_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable rsa clock
    #[inline(always)]
    #[must_use]
    pub fn rsa_clk_en(&mut self) -> RSA_CLK_EN_W<RSA_CONF_SPEC> {
        RSA_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset rsa module
    #[inline(always)]
    #[must_use]
    pub fn rsa_rst_en(&mut self) -> RSA_RST_EN_W<RSA_CONF_SPEC> {
        RSA_RST_EN_W::new(self, 1)
    }
}
/**RSA configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rsa_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RSA_CONF_SPEC;
impl crate::RegisterSpec for RSA_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rsa_conf::R`](R) reader structure
impl crate::Readable for RSA_CONF_SPEC {}
///`write(|w| ..)` method takes [`rsa_conf::W`](W) writer structure
impl crate::Writable for RSA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RSA_CONF to value 0x01
impl crate::Resettable for RSA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
