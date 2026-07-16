#[doc = "Register `PERI1_TIMEOUT_CONF` reader"]
pub type R = crate::R<PERI1_TIMEOUT_CONF_SPEC>;
#[doc = "Register `PERI1_TIMEOUT_CONF` writer"]
pub type W = crate::W<PERI1_TIMEOUT_CONF_SPEC>;
#[doc = "Field `PERI1_TIMEOUT_THRES` reader - Configures the timeout threshold for bus access for accessing CPU peripheral register in the number of clock cycles of the clock domain."]
pub type PERI1_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `PERI1_TIMEOUT_THRES` writer - Configures the timeout threshold for bus access for accessing CPU peripheral register in the number of clock cycles of the clock domain."]
pub type PERI1_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PERI1_TIMEOUT_INT_CLR` writer - Write 1 to clear timeout interrupt."]
pub type PERI1_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERI1_TIMEOUT_PROTECT_EN` reader - Configures whether or not to enable timeout protection for accessing CPU peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type PERI1_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `PERI1_TIMEOUT_PROTECT_EN` writer - Configures whether or not to enable timeout protection for accessing CPU peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type PERI1_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Configures the timeout threshold for bus access for accessing CPU peripheral register in the number of clock cycles of the clock domain."]
    #[inline(always)]
    pub fn peri1_timeout_thres(&self) -> PERI1_TIMEOUT_THRES_R {
        PERI1_TIMEOUT_THRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable timeout protection for accessing CPU peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn peri1_timeout_protect_en(&self) -> PERI1_TIMEOUT_PROTECT_EN_R {
        PERI1_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI1_TIMEOUT_CONF")
            .field("peri1_timeout_thres", &self.peri1_timeout_thres())
            .field("peri1_timeout_protect_en", &self.peri1_timeout_protect_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the timeout threshold for bus access for accessing CPU peripheral register in the number of clock cycles of the clock domain."]
    #[inline(always)]
    pub fn peri1_timeout_thres(&mut self) -> PERI1_TIMEOUT_THRES_W<'_, PERI1_TIMEOUT_CONF_SPEC> {
        PERI1_TIMEOUT_THRES_W::new(self, 0)
    }
    #[doc = "Bit 16 - Write 1 to clear timeout interrupt."]
    #[inline(always)]
    pub fn peri1_timeout_int_clr(
        &mut self,
    ) -> PERI1_TIMEOUT_INT_CLR_W<'_, PERI1_TIMEOUT_CONF_SPEC> {
        PERI1_TIMEOUT_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable timeout protection for accessing CPU peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn peri1_timeout_protect_en(
        &mut self,
    ) -> PERI1_TIMEOUT_PROTECT_EN_W<'_, PERI1_TIMEOUT_CONF_SPEC> {
        PERI1_TIMEOUT_PROTECT_EN_W::new(self, 17)
    }
}
#[doc = "HP_PERI1_TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`peri1_timeout_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri1_timeout_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI1_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for PERI1_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri1_timeout_conf::R`](R) reader structure"]
impl crate::Readable for PERI1_TIMEOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri1_timeout_conf::W`](W) writer structure"]
impl crate::Writable for PERI1_TIMEOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI1_TIMEOUT_CONF to value 0x0002_ffff"]
impl crate::Resettable for PERI1_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0002_ffff;
}
