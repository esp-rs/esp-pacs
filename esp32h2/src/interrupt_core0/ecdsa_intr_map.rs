///Register `ECDSA_INTR_MAP` reader
pub type R = crate::R<ECDSA_INTR_MAP_SPEC>;
///Register `ECDSA_INTR_MAP` writer
pub type W = crate::W<ECDSA_INTR_MAP_SPEC>;
///Field `ECDSA_INTR_MAP` reader - CORE0_ECDSA_INTR mapping register
pub type ECDSA_INTR_MAP_R = crate::FieldReader;
///Field `ECDSA_INTR_MAP` writer - CORE0_ECDSA_INTR mapping register
pub type ECDSA_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - CORE0_ECDSA_INTR mapping register
    #[inline(always)]
    pub fn ecdsa_intr_map(&self) -> ECDSA_INTR_MAP_R {
        ECDSA_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECDSA_INTR_MAP")
            .field("ecdsa_intr_map", &self.ecdsa_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - CORE0_ECDSA_INTR mapping register
    #[inline(always)]
    #[must_use]
    pub fn ecdsa_intr_map(&mut self) -> ECDSA_INTR_MAP_W<ECDSA_INTR_MAP_SPEC> {
        ECDSA_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`ecdsa_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecdsa_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECDSA_INTR_MAP_SPEC;
impl crate::RegisterSpec for ECDSA_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ecdsa_intr_map::R`](R) reader structure
impl crate::Readable for ECDSA_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`ecdsa_intr_map::W`](W) writer structure
impl crate::Writable for ECDSA_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ECDSA_INTR_MAP to value 0
impl crate::Resettable for ECDSA_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
