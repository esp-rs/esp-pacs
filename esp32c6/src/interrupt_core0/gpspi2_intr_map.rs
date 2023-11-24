#[doc = "Register `GPSPI2_INTR_MAP` reader"]
pub type R = crate::R<GPSPI2_INTR_MAP_SPEC>;
#[doc = "Register `GPSPI2_INTR_MAP` writer"]
pub type W = crate::W<GPSPI2_INTR_MAP_SPEC>;
#[doc = "Field `GPSPI2_INTR_MAP` reader - Need add description"]
pub type GPSPI2_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `GPSPI2_INTR_MAP` writer - Need add description"]
pub type GPSPI2_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn gpspi2_intr_map(&self) -> GPSPI2_INTR_MAP_R {
        GPSPI2_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPSPI2_INTR_MAP")
            .field(
                "gpspi2_intr_map",
                &format_args!("{}", self.gpspi2_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPSPI2_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn gpspi2_intr_map(&mut self) -> GPSPI2_INTR_MAP_W<GPSPI2_INTR_MAP_SPEC> {
        GPSPI2_INTR_MAP_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpspi2_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpspi2_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPSPI2_INTR_MAP_SPEC;
impl crate::RegisterSpec for GPSPI2_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpspi2_intr_map::R`](R) reader structure"]
impl crate::Readable for GPSPI2_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpspi2_intr_map::W`](W) writer structure"]
impl crate::Writable for GPSPI2_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPSPI2_INTR_MAP to value 0"]
impl crate::Resettable for GPSPI2_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
