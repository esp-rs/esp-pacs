#[doc = "Register `RSA_CONF` reader"]
pub type R = crate::R<RSA_CONF_SPEC>;
#[doc = "Field `RSA_CLK_EN` reader - Set 1 to enable rsa clock"]
pub type RSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `RSA_RST_EN` reader - Set 1 to reset rsa module"]
pub type RSA_RST_EN_R = crate::BitReader;
#[doc = "Field `RSA_READY` reader - Query this field after reset rsa module"]
pub type RSA_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable rsa clock"]
    #[inline(always)]
    pub fn rsa_clk_en(&self) -> RSA_CLK_EN_R {
        RSA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset rsa module"]
    #[inline(always)]
    pub fn rsa_rst_en(&self) -> RSA_RST_EN_R {
        RSA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset rsa module"]
    #[inline(always)]
    pub fn rsa_ready(&self) -> RSA_READY_R {
        RSA_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA_CONF")
            .field("rsa_clk_en", &self.rsa_clk_en())
            .field("rsa_rst_en", &self.rsa_rst_en())
            .field("rsa_ready", &self.rsa_ready())
            .finish()
    }
}
#[doc = "RSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSA_CONF_SPEC;
impl crate::RegisterSpec for RSA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsa_conf::R`](R) reader structure"]
impl crate::Readable for RSA_CONF_SPEC {}
#[doc = "`reset()` method sets RSA_CONF to value 0x04"]
impl crate::Resettable for RSA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
