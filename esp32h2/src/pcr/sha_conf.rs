#[doc = "Register `SHA_CONF` reader"]
pub type R = crate::R<SHA_CONF_SPEC>;
#[doc = "Register `SHA_CONF` writer"]
pub type W = crate::W<SHA_CONF_SPEC>;
#[doc = "Field `SHA_CLK_EN` reader - Set 1 to enable sha clock"]
pub type SHA_CLK_EN_R = crate::BitReader;
#[doc = "Field `SHA_CLK_EN` writer - Set 1 to enable sha clock"]
pub type SHA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA_RST_EN` reader - Set 0 to reset sha module"]
pub type SHA_RST_EN_R = crate::BitReader;
#[doc = "Field `SHA_RST_EN` writer - Set 0 to reset sha module"]
pub type SHA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA_READY` reader - Query this field after reset sha module"]
pub type SHA_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable sha clock"]
    #[inline(always)]
    pub fn sha_clk_en(&self) -> SHA_CLK_EN_R {
        SHA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset sha module"]
    #[inline(always)]
    pub fn sha_rst_en(&self) -> SHA_RST_EN_R {
        SHA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset sha module"]
    #[inline(always)]
    pub fn sha_ready(&self) -> SHA_READY_R {
        SHA_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA_CONF")
            .field("sha_clk_en", &format_args!("{}", self.sha_clk_en().bit()))
            .field("sha_rst_en", &format_args!("{}", self.sha_rst_en().bit()))
            .field("sha_ready", &format_args!("{}", self.sha_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable sha clock"]
    #[inline(always)]
    #[must_use]
    pub fn sha_clk_en(&mut self) -> SHA_CLK_EN_W<SHA_CONF_SPEC> {
        SHA_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset sha module"]
    #[inline(always)]
    #[must_use]
    pub fn sha_rst_en(&mut self) -> SHA_RST_EN_W<SHA_CONF_SPEC> {
        SHA_RST_EN_W::new(self, 1)
    }
}
#[doc = "SHA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA_CONF_SPEC;
impl crate::RegisterSpec for SHA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_conf::R`](R) reader structure"]
impl crate::Readable for SHA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sha_conf::W`](W) writer structure"]
impl crate::Writable for SHA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA_CONF to value 0x05"]
impl crate::Resettable for SHA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
