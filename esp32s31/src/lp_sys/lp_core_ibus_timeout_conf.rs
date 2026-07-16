#[doc = "Register `LP_CORE_IBUS_TIMEOUT_CONF` reader"]
pub type R = crate::R<LP_CORE_IBUS_TIMEOUT_CONF_SPEC>;
#[doc = "Register `LP_CORE_IBUS_TIMEOUT_CONF` writer"]
pub type W = crate::W<LP_CORE_IBUS_TIMEOUT_CONF_SPEC>;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_PROTECT_EN` reader - "]
pub type LP_CORE_IBUS_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_PROTECT_EN` writer - "]
pub type LP_CORE_IBUS_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_THRES` reader - "]
pub type LP_CORE_IBUS_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_THRES` writer - "]
pub type LP_CORE_IBUS_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_protect_en(&self) -> LP_CORE_IBUS_TIMEOUT_PROTECT_EN_R {
        LP_CORE_IBUS_TIMEOUT_PROTECT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_thres(&self) -> LP_CORE_IBUS_TIMEOUT_THRES_R {
        LP_CORE_IBUS_TIMEOUT_THRES_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CORE_IBUS_TIMEOUT_CONF")
            .field(
                "lp_core_ibus_timeout_protect_en",
                &self.lp_core_ibus_timeout_protect_en(),
            )
            .field(
                "lp_core_ibus_timeout_thres",
                &self.lp_core_ibus_timeout_thres(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_protect_en(
        &mut self,
    ) -> LP_CORE_IBUS_TIMEOUT_PROTECT_EN_W<'_, LP_CORE_IBUS_TIMEOUT_CONF_SPEC> {
        LP_CORE_IBUS_TIMEOUT_PROTECT_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:16"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_thres(
        &mut self,
    ) -> LP_CORE_IBUS_TIMEOUT_THRES_W<'_, LP_CORE_IBUS_TIMEOUT_CONF_SPEC> {
        LP_CORE_IBUS_TIMEOUT_THRES_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_ibus_timeout_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_ibus_timeout_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CORE_IBUS_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for LP_CORE_IBUS_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_core_ibus_timeout_conf::R`](R) reader structure"]
impl crate::Readable for LP_CORE_IBUS_TIMEOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_core_ibus_timeout_conf::W`](W) writer structure"]
impl crate::Writable for LP_CORE_IBUS_TIMEOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CORE_IBUS_TIMEOUT_CONF to value 0x0001_ffff"]
impl crate::Resettable for LP_CORE_IBUS_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
