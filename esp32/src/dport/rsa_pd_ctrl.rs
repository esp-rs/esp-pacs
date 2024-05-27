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
    #[must_use]
    pub fn rsa_pd(&mut self) -> RSA_PD_W<RSA_PD_CTRL_SPEC> {
        RSA_PD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_pd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSA_PD_CTRL_SPEC;
impl crate::RegisterSpec for RSA_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsa_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for RSA_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsa_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for RSA_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSA_PD_CTRL to value 0"]
impl crate::Resettable for RSA_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
