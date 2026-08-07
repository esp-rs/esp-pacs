#[doc = "Register `SLP1` reader"]
pub type R = crate::R<SLP1_SPEC>;
#[doc = "Register `SLP1` writer"]
pub type W = crate::W<SLP1_SPEC>;
#[doc = "Field `TOUCH_SLP_TH2` reader - need_des"]
pub type TOUCH_SLP_TH2_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLP_TH2` writer - need_des"]
pub type TOUCH_SLP_TH2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_SLP_TH1` reader - need_des"]
pub type TOUCH_SLP_TH1_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLP_TH1` writer - need_des"]
pub type TOUCH_SLP_TH1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th2(&self) -> TOUCH_SLP_TH2_R {
        TOUCH_SLP_TH2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th1(&self) -> TOUCH_SLP_TH1_R {
        TOUCH_SLP_TH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP1")
            .field("touch_slp_th2", &self.touch_slp_th2())
            .field("touch_slp_th1", &self.touch_slp_th1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th2(&mut self) -> TOUCH_SLP_TH2_W<'_, SLP1_SPEC> {
        TOUCH_SLP_TH2_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn touch_slp_th1(&mut self) -> TOUCH_SLP_TH1_W<'_, SLP1_SPEC> {
        TOUCH_SLP_TH1_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP1_SPEC;
impl crate::RegisterSpec for SLP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp1::R`](R) reader structure"]
impl crate::Readable for SLP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp1::W`](W) writer structure"]
impl crate::Writable for SLP1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP1 to value 0"]
impl crate::Resettable for SLP1_SPEC {}
