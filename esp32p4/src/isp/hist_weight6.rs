#[doc = "Register `HIST_WEIGHT6` reader"]
pub type R = crate::R<HIST_WEIGHT6_SPEC>;
#[doc = "Register `HIST_WEIGHT6` writer"]
pub type W = crate::W<HIST_WEIGHT6_SPEC>;
#[doc = "Field `HIST_WEIGHT_44` reader - this field configures weight of subwindow 44"]
pub type HIST_WEIGHT_44_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_44` writer - this field configures weight of subwindow 44"]
pub type HIST_WEIGHT_44_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 44"]
    #[inline(always)]
    pub fn hist_weight_44(&self) -> HIST_WEIGHT_44_R {
        HIST_WEIGHT_44_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_WEIGHT6")
            .field("hist_weight_44", &self.hist_weight_44())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 44"]
    #[inline(always)]
    pub fn hist_weight_44(&mut self) -> HIST_WEIGHT_44_W<HIST_WEIGHT6_SPEC> {
        HIST_WEIGHT_44_W::new(self, 0)
    }
}
#[doc = "histogram sub-window weight register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_WEIGHT6_SPEC;
impl crate::RegisterSpec for HIST_WEIGHT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight6::R`](R) reader structure"]
impl crate::Readable for HIST_WEIGHT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_weight6::W`](W) writer structure"]
impl crate::Writable for HIST_WEIGHT6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT6 to value 0x01"]
impl crate::Resettable for HIST_WEIGHT6_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
