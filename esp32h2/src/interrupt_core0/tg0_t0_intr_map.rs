///Register `TG0_T0_INTR_MAP` reader
pub type R = crate::R<TG0_T0_INTR_MAP_SPEC>;
///Register `TG0_T0_INTR_MAP` writer
pub type W = crate::W<TG0_T0_INTR_MAP_SPEC>;
///Field `TG0_T0_INTR_MAP` reader - CORE0_TG0_T0_INTR mapping register
pub type TG0_T0_INTR_MAP_R = crate::FieldReader;
///Field `TG0_T0_INTR_MAP` writer - CORE0_TG0_T0_INTR mapping register
pub type TG0_T0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - CORE0_TG0_T0_INTR mapping register
    #[inline(always)]
    pub fn tg0_t0_intr_map(&self) -> TG0_T0_INTR_MAP_R {
        TG0_T0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TG0_T0_INTR_MAP")
            .field("tg0_t0_intr_map", &self.tg0_t0_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - CORE0_TG0_T0_INTR mapping register
    #[inline(always)]
    #[must_use]
    pub fn tg0_t0_intr_map(&mut self) -> TG0_T0_INTR_MAP_W<TG0_T0_INTR_MAP_SPEC> {
        TG0_T0_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`tg0_t0_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg0_t0_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TG0_T0_INTR_MAP_SPEC;
impl crate::RegisterSpec for TG0_T0_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tg0_t0_intr_map::R`](R) reader structure
impl crate::Readable for TG0_T0_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`tg0_t0_intr_map::W`](W) writer structure
impl crate::Writable for TG0_T0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TG0_T0_INTR_MAP to value 0
impl crate::Resettable for TG0_T0_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
