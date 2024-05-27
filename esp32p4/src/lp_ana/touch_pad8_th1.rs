#[doc = "Register `TOUCH_PAD8_TH1` reader"]
pub type R = crate::R<TOUCH_PAD8_TH1_SPEC>;
#[doc = "Register `TOUCH_PAD8_TH1` writer"]
pub type W = crate::W<TOUCH_PAD8_TH1_SPEC>;
#[doc = "Field `TOUCH_PAD8_TH1` reader - Reserved"]
pub type TOUCH_PAD8_TH1_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD8_TH1` writer - Reserved"]
pub type TOUCH_PAD8_TH1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad8_th1(&self) -> TOUCH_PAD8_TH1_R {
        TOUCH_PAD8_TH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD8_TH1")
            .field("touch_pad8_th1", &self.touch_pad8_th1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad8_th1(&mut self) -> TOUCH_PAD8_TH1_W<TOUCH_PAD8_TH1_SPEC> {
        TOUCH_PAD8_TH1_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad8_th1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad8_th1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_PAD8_TH1_SPEC;
impl crate::RegisterSpec for TOUCH_PAD8_TH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pad8_th1::R`](R) reader structure"]
impl crate::Readable for TOUCH_PAD8_TH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_pad8_th1::W`](W) writer structure"]
impl crate::Writable for TOUCH_PAD8_TH1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_PAD8_TH1 to value 0"]
impl crate::Resettable for TOUCH_PAD8_TH1_SPEC {
    const RESET_VALUE: u32 = 0;
}
