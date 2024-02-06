#[doc = "Register `L2_BYPASS_CACHE_CONF` reader"]
pub type R = crate::R<L2_BYPASS_CACHE_CONF_SPEC>;
#[doc = "Register `L2_BYPASS_CACHE_CONF` writer"]
pub type W = crate::W<L2_BYPASS_CACHE_CONF_SPEC>;
#[doc = "Field `BYPASS_L2_CACHE_EN` reader - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L2_CACHE_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L2_CACHE_EN` writer - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L2_CACHE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l2_cache_en(&self) -> BYPASS_L2_CACHE_EN_R {
        BYPASS_L2_CACHE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_BYPASS_CACHE_CONF")
            .field(
                "bypass_l2_cache_en",
                &format_args!("{}", self.bypass_l2_cache_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_BYPASS_CACHE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 5 - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_l2_cache_en(&mut self) -> BYPASS_L2_CACHE_EN_W<L2_BYPASS_CACHE_CONF_SPEC> {
        BYPASS_L2_CACHE_EN_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bypass Cache configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_bypass_cache_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_bypass_cache_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_BYPASS_CACHE_CONF_SPEC;
impl crate::RegisterSpec for L2_BYPASS_CACHE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_bypass_cache_conf::R`](R) reader structure"]
impl crate::Readable for L2_BYPASS_CACHE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_bypass_cache_conf::W`](W) writer structure"]
impl crate::Writable for L2_BYPASS_CACHE_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_BYPASS_CACHE_CONF to value 0"]
impl crate::Resettable for L2_BYPASS_CACHE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
