#[doc = "Register `TOUCH_FILTER3` reader"]
pub type R = crate::R<TOUCH_FILTER3_SPEC>;
#[doc = "Register `TOUCH_FILTER3` writer"]
pub type W = crate::W<TOUCH_FILTER3_SPEC>;
#[doc = "Field `TOUCH_BASELINE_SW` reader - need_des"]
pub type TOUCH_BASELINE_SW_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_BASELINE_SW` writer - need_des"]
pub type TOUCH_BASELINE_SW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_UPDATE_BASELINE_SW` writer - need_des"]
pub type TOUCH_UPDATE_BASELINE_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_baseline_sw(&self) -> TOUCH_BASELINE_SW_R {
        TOUCH_BASELINE_SW_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_FILTER3")
            .field("touch_baseline_sw", &self.touch_baseline_sw())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_baseline_sw(&mut self) -> TOUCH_BASELINE_SW_W<TOUCH_FILTER3_SPEC> {
        TOUCH_BASELINE_SW_W::new(self, 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_update_baseline_sw(&mut self) -> TOUCH_UPDATE_BASELINE_SW_W<TOUCH_FILTER3_SPEC> {
        TOUCH_UPDATE_BASELINE_SW_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_filter3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_FILTER3_SPEC;
impl crate::RegisterSpec for TOUCH_FILTER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_filter3::R`](R) reader structure"]
impl crate::Readable for TOUCH_FILTER3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_filter3::W`](W) writer structure"]
impl crate::Writable for TOUCH_FILTER3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_FILTER3 to value 0"]
impl crate::Resettable for TOUCH_FILTER3_SPEC {
    const RESET_VALUE: u32 = 0;
}
