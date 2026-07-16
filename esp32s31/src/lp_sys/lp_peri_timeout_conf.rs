#[doc = "Register `LP_PERI_TIMEOUT_CONF` reader"]
pub type R = crate::R<LP_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "Register `LP_PERI_TIMEOUT_CONF` writer"]
pub type W = crate::W<LP_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "Field `LP_PERI_TIMEOUT_THRES` reader - "]
pub type LP_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `LP_PERI_TIMEOUT_THRES` writer - "]
pub type LP_PERI_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LP_PERI_TIMEOUT_INT_CLEAR` writer - "]
pub type LP_PERI_TIMEOUT_INT_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PERI_TIMEOUT_PROTECT_EN` reader - "]
pub type LP_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `LP_PERI_TIMEOUT_PROTECT_EN` writer - "]
pub type LP_PERI_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lp_peri_timeout_thres(&self) -> LP_PERI_TIMEOUT_THRES_R {
        LP_PERI_TIMEOUT_THRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lp_peri_timeout_protect_en(&self) -> LP_PERI_TIMEOUT_PROTECT_EN_R {
        LP_PERI_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PERI_TIMEOUT_CONF")
            .field("lp_peri_timeout_thres", &self.lp_peri_timeout_thres())
            .field(
                "lp_peri_timeout_protect_en",
                &self.lp_peri_timeout_protect_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lp_peri_timeout_thres(
        &mut self,
    ) -> LP_PERI_TIMEOUT_THRES_W<'_, LP_PERI_TIMEOUT_CONF_SPEC> {
        LP_PERI_TIMEOUT_THRES_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lp_peri_timeout_int_clear(
        &mut self,
    ) -> LP_PERI_TIMEOUT_INT_CLEAR_W<'_, LP_PERI_TIMEOUT_CONF_SPEC> {
        LP_PERI_TIMEOUT_INT_CLEAR_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lp_peri_timeout_protect_en(
        &mut self,
    ) -> LP_PERI_TIMEOUT_PROTECT_EN_W<'_, LP_PERI_TIMEOUT_CONF_SPEC> {
        LP_PERI_TIMEOUT_PROTECT_EN_W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri_timeout_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PERI_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for LP_PERI_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_peri_timeout_conf::R`](R) reader structure"]
impl crate::Readable for LP_PERI_TIMEOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_peri_timeout_conf::W`](W) writer structure"]
impl crate::Writable for LP_PERI_TIMEOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PERI_TIMEOUT_CONF to value 0x0002_ffff"]
impl crate::Resettable for LP_PERI_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0002_ffff;
}
