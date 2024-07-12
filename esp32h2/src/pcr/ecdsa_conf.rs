#[doc = "Register `ECDSA_CONF` reader"]
pub type R = crate::R<ECDSA_CONF_SPEC>;
#[doc = "Register `ECDSA_CONF` writer"]
pub type W = crate::W<ECDSA_CONF_SPEC>;
#[doc = "Field `ECDSA_CLK_EN` reader - Set 1 to enable ecdsa clock"]
pub type ECDSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `ECDSA_CLK_EN` writer - Set 1 to enable ecdsa clock"]
pub type ECDSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECDSA_RST_EN` reader - Set 0 to reset ecdsa module"]
pub type ECDSA_RST_EN_R = crate::BitReader;
#[doc = "Field `ECDSA_RST_EN` writer - Set 0 to reset ecdsa module"]
pub type ECDSA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECDSA_READY` reader - Query this field after reset ecdsa module"]
pub type ECDSA_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable ecdsa clock"]
    #[inline(always)]
    pub fn ecdsa_clk_en(&self) -> ECDSA_CLK_EN_R {
        ECDSA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ecdsa module"]
    #[inline(always)]
    pub fn ecdsa_rst_en(&self) -> ECDSA_RST_EN_R {
        ECDSA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset ecdsa module"]
    #[inline(always)]
    pub fn ecdsa_ready(&self) -> ECDSA_READY_R {
        ECDSA_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECDSA_CONF")
            .field("ecdsa_clk_en", &self.ecdsa_clk_en())
            .field("ecdsa_rst_en", &self.ecdsa_rst_en())
            .field("ecdsa_ready", &self.ecdsa_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable ecdsa clock"]
    #[inline(always)]
    #[must_use]
    pub fn ecdsa_clk_en(&mut self) -> ECDSA_CLK_EN_W<ECDSA_CONF_SPEC> {
        ECDSA_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ecdsa module"]
    #[inline(always)]
    #[must_use]
    pub fn ecdsa_rst_en(&mut self) -> ECDSA_RST_EN_W<ECDSA_CONF_SPEC> {
        ECDSA_RST_EN_W::new(self, 1)
    }
}
#[doc = "ECDSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecdsa_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecdsa_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECDSA_CONF_SPEC;
impl crate::RegisterSpec for ECDSA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecdsa_conf::R`](R) reader structure"]
impl crate::Readable for ECDSA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecdsa_conf::W`](W) writer structure"]
impl crate::Writable for ECDSA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECDSA_CONF to value 0x05"]
impl crate::Resettable for ECDSA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
