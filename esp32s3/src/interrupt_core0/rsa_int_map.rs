#[doc = "Register `RSA_INT_MAP` reader"]
pub type R = crate::R<RSA_INT_MAP_SPEC>;
#[doc = "Register `RSA_INT_MAP` writer"]
pub type W = crate::W<RSA_INT_MAP_SPEC>;
#[doc = "Field `RSA_INT_MAP` reader - this register used to map rsa interrupt to one of core0's external interrupt"]
pub type RSA_INT_MAP_R = crate::FieldReader;
#[doc = "Field `RSA_INT_MAP` writer - this register used to map rsa interrupt to one of core0's external interrupt"]
pub type RSA_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map rsa interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn rsa_int_map(&self) -> RSA_INT_MAP_R {
        RSA_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA_INT_MAP")
            .field("rsa_int_map", &self.rsa_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map rsa interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn rsa_int_map(&mut self) -> RSA_INT_MAP_W<RSA_INT_MAP_SPEC> {
        RSA_INT_MAP_W::new(self, 0)
    }
}
#[doc = "rsa interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSA_INT_MAP_SPEC;
impl crate::RegisterSpec for RSA_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsa_int_map::R`](R) reader structure"]
impl crate::Readable for RSA_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsa_int_map::W`](W) writer structure"]
impl crate::Writable for RSA_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSA_INT_MAP to value 0x10"]
impl crate::Resettable for RSA_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
