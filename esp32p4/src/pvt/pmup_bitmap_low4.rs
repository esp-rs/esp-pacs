///Register `PMUP_BITMAP_LOW4` reader
pub type R = crate::R<PMUP_BITMAP_LOW4_SPEC>;
///Register `PMUP_BITMAP_LOW4` writer
pub type W = crate::W<PMUP_BITMAP_LOW4_SPEC>;
///Field `PUMP_BITMAP_LOW4` reader - select valid low channel4
pub type PUMP_BITMAP_LOW4_R = crate::FieldReader<u32>;
///Field `PUMP_BITMAP_LOW4` writer - select valid low channel4
pub type PUMP_BITMAP_LOW4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - select valid low channel4
    #[inline(always)]
    pub fn pump_bitmap_low4(&self) -> PUMP_BITMAP_LOW4_R {
        PUMP_BITMAP_LOW4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMUP_BITMAP_LOW4")
            .field("pump_bitmap_low4", &self.pump_bitmap_low4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - select valid low channel4
    #[inline(always)]
    #[must_use]
    pub fn pump_bitmap_low4(&mut self) -> PUMP_BITMAP_LOW4_W<PMUP_BITMAP_LOW4_SPEC> {
        PUMP_BITMAP_LOW4_W::new(self, 0)
    }
}
/**select valid pvt channel

You can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_low4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_low4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PMUP_BITMAP_LOW4_SPEC;
impl crate::RegisterSpec for PMUP_BITMAP_LOW4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pmup_bitmap_low4::R`](R) reader structure
impl crate::Readable for PMUP_BITMAP_LOW4_SPEC {}
///`write(|w| ..)` method takes [`pmup_bitmap_low4::W`](W) writer structure
impl crate::Writable for PMUP_BITMAP_LOW4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PMUP_BITMAP_LOW4 to value 0
impl crate::Resettable for PMUP_BITMAP_LOW4_SPEC {
    const RESET_VALUE: u32 = 0;
}
