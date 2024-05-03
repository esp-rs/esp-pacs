#[doc = "Register `ECC_INTR_MAP` reader"]
pub type R = crate::R<ECC_INTR_MAP_SPEC>;
#[doc = "Register `ECC_INTR_MAP` writer"]
pub type W = crate::W<ECC_INTR_MAP_SPEC>;
#[doc = "Field `ECC_INTR_MAP` reader - Need add description"]
pub type ECC_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `ECC_INTR_MAP` writer - Need add description"]
pub type ECC_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn ecc_intr_map(&self) -> ECC_INTR_MAP_R {
        ECC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_INTR_MAP")
            .field("ecc_intr_map", &self.ecc_intr_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECC_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_intr_map(&mut self) -> ECC_INTR_MAP_W<ECC_INTR_MAP_SPEC> {
        ECC_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_INTR_MAP_SPEC;
impl crate::RegisterSpec for ECC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_intr_map::R`](R) reader structure"]
impl crate::Readable for ECC_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc_intr_map::W`](W) writer structure"]
impl crate::Writable for ECC_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_INTR_MAP to value 0"]
impl crate::Resettable for ECC_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
