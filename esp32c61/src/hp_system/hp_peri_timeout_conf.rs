#[doc = "Register `HP_PERI_TIMEOUT_CONF` reader"]
pub type R = crate::R<HP_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "Register `HP_PERI_TIMEOUT_CONF` writer"]
pub type W = crate::W<HP_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "Field `HP_PERI_TIMEOUT_THRES` reader - Configures the timeout threshold for bus access for accessing HP peripheral register, corresponding to the number of clock cycles of the clock domain."]
pub type HP_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `HP_PERI_TIMEOUT_THRES` writer - Configures the timeout threshold for bus access for accessing HP peripheral register, corresponding to the number of clock cycles of the clock domain."]
pub type HP_PERI_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HP_PERI_TIMEOUT_INT_CLEAR` writer - Configures whether or not to clear timeout interrupt.\\\\ 0: No effect\\\\ 1: Clear timeout interrupt\\\\"]
pub type HP_PERI_TIMEOUT_INT_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_PERI_TIMEOUT_PROTECT_EN` reader - Configures whether or not to enable timeout protection for accessing HP peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type HP_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `HP_PERI_TIMEOUT_PROTECT_EN` writer - Configures whether or not to enable timeout protection for accessing HP peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type HP_PERI_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Configures the timeout threshold for bus access for accessing HP peripheral register, corresponding to the number of clock cycles of the clock domain."]
    #[inline(always)]
    pub fn hp_peri_timeout_thres(&self) -> HP_PERI_TIMEOUT_THRES_R {
        HP_PERI_TIMEOUT_THRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable timeout protection for accessing HP peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn hp_peri_timeout_protect_en(&self) -> HP_PERI_TIMEOUT_PROTECT_EN_R {
        HP_PERI_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PERI_TIMEOUT_CONF")
            .field("hp_peri_timeout_thres", &self.hp_peri_timeout_thres())
            .field(
                "hp_peri_timeout_protect_en",
                &self.hp_peri_timeout_protect_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the timeout threshold for bus access for accessing HP peripheral register, corresponding to the number of clock cycles of the clock domain."]
    #[inline(always)]
    pub fn hp_peri_timeout_thres(&mut self) -> HP_PERI_TIMEOUT_THRES_W<HP_PERI_TIMEOUT_CONF_SPEC> {
        HP_PERI_TIMEOUT_THRES_W::new(self, 0)
    }
    #[doc = "Bit 16 - Configures whether or not to clear timeout interrupt.\\\\ 0: No effect\\\\ 1: Clear timeout interrupt\\\\"]
    #[inline(always)]
    pub fn hp_peri_timeout_int_clear(
        &mut self,
    ) -> HP_PERI_TIMEOUT_INT_CLEAR_W<HP_PERI_TIMEOUT_CONF_SPEC> {
        HP_PERI_TIMEOUT_INT_CLEAR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable timeout protection for accessing HP peripheral registers.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn hp_peri_timeout_protect_en(
        &mut self,
    ) -> HP_PERI_TIMEOUT_PROTECT_EN_W<HP_PERI_TIMEOUT_CONF_SPEC> {
        HP_PERI_TIMEOUT_PROTECT_EN_W::new(self, 17)
    }
}
#[doc = "HP_PERI_TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri_timeout_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri_timeout_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PERI_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for HP_PERI_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_peri_timeout_conf::R`](R) reader structure"]
impl crate::Readable for HP_PERI_TIMEOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_peri_timeout_conf::W`](W) writer structure"]
impl crate::Writable for HP_PERI_TIMEOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_PERI_TIMEOUT_CONF to value 0x0002_ffff"]
impl crate::Resettable for HP_PERI_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0002_ffff;
}
