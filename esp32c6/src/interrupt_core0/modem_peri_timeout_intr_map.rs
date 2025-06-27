#[doc = "Register `MODEM_PERI_TIMEOUT_INTR_MAP` reader"]
pub type R = crate::R<MODEM_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "Register `MODEM_PERI_TIMEOUT_INTR_MAP` writer"]
pub type W = crate::W<MODEM_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = "Field `MODEM_PERI_TIMEOUT_INTR_MAP` reader - Need add description"]
pub type MODEM_PERI_TIMEOUT_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `MODEM_PERI_TIMEOUT_INTR_MAP` writer - Need add description"]
pub type MODEM_PERI_TIMEOUT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn modem_peri_timeout_intr_map(&self) -> MODEM_PERI_TIMEOUT_INTR_MAP_R {
        MODEM_PERI_TIMEOUT_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_PERI_TIMEOUT_INTR_MAP")
            .field(
                "modem_peri_timeout_intr_map",
                &self.modem_peri_timeout_intr_map(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn modem_peri_timeout_intr_map(
        &mut self,
    ) -> MODEM_PERI_TIMEOUT_INTR_MAP_W<MODEM_PERI_TIMEOUT_INTR_MAP_SPEC> {
        MODEM_PERI_TIMEOUT_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_peri_timeout_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_peri_timeout_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_PERI_TIMEOUT_INTR_MAP_SPEC;
impl crate::RegisterSpec for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_peri_timeout_intr_map::R`](R) reader structure"]
impl crate::Readable for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_peri_timeout_intr_map::W`](W) writer structure"]
impl crate::Writable for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_PERI_TIMEOUT_INTR_MAP to value 0"]
impl crate::Resettable for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {}
