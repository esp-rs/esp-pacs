///Register `PRO_TG1_T0_LEVEL_INT_MAP` reader
pub type R = crate::R<PRO_TG1_T0_LEVEL_INT_MAP_SPEC>;
///Register `PRO_TG1_T0_LEVEL_INT_MAP` writer
pub type W = crate::W<PRO_TG1_T0_LEVEL_INT_MAP_SPEC>;
///Field `PRO_TG1_T0_LEVEL_INT_MAP` reader -
pub type PRO_TG1_T0_LEVEL_INT_MAP_R = crate::FieldReader;
///Field `PRO_TG1_T0_LEVEL_INT_MAP` writer -
pub type PRO_TG1_T0_LEVEL_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn pro_tg1_t0_level_int_map(&self) -> PRO_TG1_T0_LEVEL_INT_MAP_R {
        PRO_TG1_T0_LEVEL_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_TG1_T0_LEVEL_INT_MAP")
            .field("pro_tg1_t0_level_int_map", &self.pro_tg1_t0_level_int_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4
    #[inline(always)]
    #[must_use]
    pub fn pro_tg1_t0_level_int_map(
        &mut self,
    ) -> PRO_TG1_T0_LEVEL_INT_MAP_W<PRO_TG1_T0_LEVEL_INT_MAP_SPEC> {
        PRO_TG1_T0_LEVEL_INT_MAP_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t0_level_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t0_level_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_TG1_T0_LEVEL_INT_MAP_SPEC;
impl crate::RegisterSpec for PRO_TG1_T0_LEVEL_INT_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_tg1_t0_level_int_map::R`](R) reader structure
impl crate::Readable for PRO_TG1_T0_LEVEL_INT_MAP_SPEC {}
///`write(|w| ..)` method takes [`pro_tg1_t0_level_int_map::W`](W) writer structure
impl crate::Writable for PRO_TG1_T0_LEVEL_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_TG1_T0_LEVEL_INT_MAP to value 0x10
impl crate::Resettable for PRO_TG1_T0_LEVEL_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
