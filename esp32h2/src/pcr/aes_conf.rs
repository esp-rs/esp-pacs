///Register `AES_CONF` reader
pub type R = crate::R<AES_CONF_SPEC>;
///Register `AES_CONF` writer
pub type W = crate::W<AES_CONF_SPEC>;
///Field `AES_CLK_EN` reader - Set 1 to enable aes clock
pub type AES_CLK_EN_R = crate::BitReader;
///Field `AES_CLK_EN` writer - Set 1 to enable aes clock
pub type AES_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AES_RST_EN` reader - Set 0 to reset aes module
pub type AES_RST_EN_R = crate::BitReader;
///Field `AES_RST_EN` writer - Set 0 to reset aes module
pub type AES_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AES_READY` reader - Query this field after reset aes module
pub type AES_READY_R = crate::BitReader;
impl R {
    ///Bit 0 - Set 1 to enable aes clock
    #[inline(always)]
    pub fn aes_clk_en(&self) -> AES_CLK_EN_R {
        AES_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset aes module
    #[inline(always)]
    pub fn aes_rst_en(&self) -> AES_RST_EN_R {
        AES_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Query this field after reset aes module
    #[inline(always)]
    pub fn aes_ready(&self) -> AES_READY_R {
        AES_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_CONF")
            .field("aes_clk_en", &self.aes_clk_en())
            .field("aes_rst_en", &self.aes_rst_en())
            .field("aes_ready", &self.aes_ready())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable aes clock
    #[inline(always)]
    #[must_use]
    pub fn aes_clk_en(&mut self) -> AES_CLK_EN_W<AES_CONF_SPEC> {
        AES_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset aes module
    #[inline(always)]
    #[must_use]
    pub fn aes_rst_en(&mut self) -> AES_RST_EN_W<AES_CONF_SPEC> {
        AES_RST_EN_W::new(self, 1)
    }
}
/**AES configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`aes_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AES_CONF_SPEC;
impl crate::RegisterSpec for AES_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`aes_conf::R`](R) reader structure
impl crate::Readable for AES_CONF_SPEC {}
///`write(|w| ..)` method takes [`aes_conf::W`](W) writer structure
impl crate::Writable for AES_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_CONF to value 0x05
impl crate::Resettable for AES_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
