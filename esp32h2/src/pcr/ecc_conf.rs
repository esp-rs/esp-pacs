#[doc = "Register `ECC_CONF` reader"]
pub type R = crate::R<ECC_CONF_SPEC>;
#[doc = "Register `ECC_CONF` writer"]
pub type W = crate::W<ECC_CONF_SPEC>;
#[doc = "Field `ECC_CLK_EN` reader - Set 1 to enable ecc clock"]
pub type ECC_CLK_EN_R = crate::BitReader;
#[doc = "Field `ECC_CLK_EN` writer - Set 1 to enable ecc clock"]
pub type ECC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_RST_EN` reader - Set 0 to reset ecc module"]
pub type ECC_RST_EN_R = crate::BitReader;
#[doc = "Field `ECC_RST_EN` writer - Set 0 to reset ecc module"]
pub type ECC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_READY` reader - Query this field after reset ecc module"]
pub type ECC_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable ecc clock"]
    #[inline(always)]
    pub fn ecc_clk_en(&self) -> ECC_CLK_EN_R {
        ECC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ecc module"]
    #[inline(always)]
    pub fn ecc_rst_en(&self) -> ECC_RST_EN_R {
        ECC_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset ecc module"]
    #[inline(always)]
    pub fn ecc_ready(&self) -> ECC_READY_R {
        ECC_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_CONF")
            .field("ecc_clk_en", &self.ecc_clk_en())
            .field("ecc_rst_en", &self.ecc_rst_en())
            .field("ecc_ready", &self.ecc_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable ecc clock"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_clk_en(&mut self) -> ECC_CLK_EN_W<ECC_CONF_SPEC> {
        ECC_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ecc module"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_rst_en(&mut self) -> ECC_RST_EN_W<ECC_CONF_SPEC> {
        ECC_RST_EN_W::new(self, 1)
    }
}
#[doc = "ECC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_CONF_SPEC;
impl crate::RegisterSpec for ECC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_conf::R`](R) reader structure"]
impl crate::Readable for ECC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc_conf::W`](W) writer structure"]
impl crate::Writable for ECC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_CONF to value 0x05"]
impl crate::Resettable for ECC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
