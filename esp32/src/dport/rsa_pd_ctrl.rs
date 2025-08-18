#[doc = "Register `RSA_PD_CTRL` reader"]
pub type R = crate::R<RSA_PD_CTRL_SPEC>;
#[doc = "Register `RSA_PD_CTRL` writer"]
pub type W = crate::W<RSA_PD_CTRL_SPEC>;
#[doc = "Field `RSA_PD` reader - "]
pub type RSA_PD_R = crate::BitReader;
#[doc = "Field `RSA_PD` writer - "]
pub type RSA_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsa_pd(&self) -> RSA_PD_R {
        RSA_PD_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA_PD_CTRL")
            .field("rsa_pd", &self.rsa_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsa_pd(&mut self) -> RSA_PD_W<'_, RSA_PD_CTRL_SPEC> {
        RSA_PD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSA_PD_CTRL_SPEC;
impl crate::RegisterSpec for RSA_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsa_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for RSA_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsa_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for RSA_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSA_PD_CTRL to value 0"]
impl crate::Resettable for RSA_PD_CTRL_SPEC {}
