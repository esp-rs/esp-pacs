#[doc = "Register `PMUP_BITMAP_HIGH1` reader"]
pub type R = crate::R<PMUP_BITMAP_HIGH1_SPEC>;
#[doc = "Register `PMUP_BITMAP_HIGH1` writer"]
pub type W = crate::W<PMUP_BITMAP_HIGH1_SPEC>;
#[doc = "Field `PUMP_BITMAP_HIGH1` reader - select valid high channel1"]
pub type PUMP_BITMAP_HIGH1_R = crate::FieldReader<u32>;
#[doc = "Field `PUMP_BITMAP_HIGH1` writer - select valid high channel1"]
pub type PUMP_BITMAP_HIGH1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - select valid high channel1"]
    #[inline(always)]
    pub fn pump_bitmap_high1(&self) -> PUMP_BITMAP_HIGH1_R {
        PUMP_BITMAP_HIGH1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMUP_BITMAP_HIGH1")
            .field("pump_bitmap_high1", &self.pump_bitmap_high1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - select valid high channel1"]
    #[inline(always)]
    #[must_use]
    pub fn pump_bitmap_high1(&mut self) -> PUMP_BITMAP_HIGH1_W<PMUP_BITMAP_HIGH1_SPEC> {
        PUMP_BITMAP_HIGH1_W::new(self, 0)
    }
}
#[doc = "select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_high1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_high1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUP_BITMAP_HIGH1_SPEC;
impl crate::RegisterSpec for PMUP_BITMAP_HIGH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_bitmap_high1::R`](R) reader structure"]
impl crate::Readable for PMUP_BITMAP_HIGH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmup_bitmap_high1::W`](W) writer structure"]
impl crate::Writable for PMUP_BITMAP_HIGH1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUP_BITMAP_HIGH1 to value 0"]
impl crate::Resettable for PMUP_BITMAP_HIGH1_SPEC {
    const RESET_VALUE: u32 = 0;
}
