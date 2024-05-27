///Register `MODEM_PERI_TIMEOUT_INTR_MAP` reader
pub type R = crate::R<MODEM_PERI_TIMEOUT_INTR_MAP_SPEC>;
///Register `MODEM_PERI_TIMEOUT_INTR_MAP` writer
pub type W = crate::W<MODEM_PERI_TIMEOUT_INTR_MAP_SPEC>;
///Field `MODEM_PERI_TIMEOUT_INTR_MAP` reader - Need add description
pub type MODEM_PERI_TIMEOUT_INTR_MAP_R = crate::FieldReader;
///Field `MODEM_PERI_TIMEOUT_INTR_MAP` writer - Need add description
pub type MODEM_PERI_TIMEOUT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Need add description
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
    ///Bits 0:4 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn modem_peri_timeout_intr_map(
        &mut self,
    ) -> MODEM_PERI_TIMEOUT_INTR_MAP_W<MODEM_PERI_TIMEOUT_INTR_MAP_SPEC> {
        MODEM_PERI_TIMEOUT_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`modem_peri_timeout_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_peri_timeout_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MODEM_PERI_TIMEOUT_INTR_MAP_SPEC;
impl crate::RegisterSpec for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`modem_peri_timeout_intr_map::R`](R) reader structure
impl crate::Readable for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`modem_peri_timeout_intr_map::W`](W) writer structure
impl crate::Writable for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODEM_PERI_TIMEOUT_INTR_MAP to value 0
impl crate::Resettable for MODEM_PERI_TIMEOUT_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
