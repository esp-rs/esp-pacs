#[doc = "Register `ECC_INT_MAP` reader"]
pub type R = crate::R<ECC_INT_MAP_SPEC>;
#[doc = "Register `ECC_INT_MAP` writer"]
pub type W = crate::W<ECC_INT_MAP_SPEC>;
#[doc = "Field `ECC_INT_MAP` reader - Need add description"]
pub type ECC_INT_MAP_R = crate::FieldReader;
#[doc = "Field `ECC_INT_MAP` writer - Need add description"]
pub type ECC_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn ecc_int_map(&self) -> ECC_INT_MAP_R {
        ECC_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_INT_MAP")
            .field("ecc_int_map", &self.ecc_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn ecc_int_map(&mut self) -> ECC_INT_MAP_W<ECC_INT_MAP_SPEC> {
        ECC_INT_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_INT_MAP_SPEC;
impl crate::RegisterSpec for ECC_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_int_map::R`](R) reader structure"]
impl crate::Readable for ECC_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc_int_map::W`](W) writer structure"]
impl crate::Writable for ECC_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_INT_MAP to value 0"]
impl crate::Resettable for ECC_INT_MAP_SPEC {}
