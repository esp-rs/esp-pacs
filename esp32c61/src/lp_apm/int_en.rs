#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `M0_APM_INT_EN` reader - Configures to enable APM M0 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type M0_APM_INT_EN_R = crate::BitReader;
#[doc = "Field `M0_APM_INT_EN` writer - Configures to enable APM M0 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
pub type M0_APM_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures to enable APM M0 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m0_apm_int_en(&self) -> M0_APM_INT_EN_R {
        M0_APM_INT_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_EN")
            .field("m0_apm_int_en", &self.m0_apm_int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures to enable APM M0 interrupt.\\\\ 0: disable \\\\ 1: enable \\\\"]
    #[inline(always)]
    pub fn m0_apm_int_en(&mut self) -> M0_APM_INT_EN_W<INT_EN_SPEC> {
        M0_APM_INT_EN_W::new(self, 0)
    }
}
#[doc = "APM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {}
