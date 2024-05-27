#[doc = "Register `AF_GEN_TH_CTRL` reader"]
pub type R = crate::R<AF_GEN_TH_CTRL_SPEC>;
#[doc = "Register `AF_GEN_TH_CTRL` writer"]
pub type W = crate::W<AF_GEN_TH_CTRL_SPEC>;
#[doc = "Field `AF_GEN_THRESHOLD_MIN` reader - this field configures min threshold when use auto_threshold"]
pub type AF_GEN_THRESHOLD_MIN_R = crate::FieldReader<u16>;
#[doc = "Field `AF_GEN_THRESHOLD_MIN` writer - this field configures min threshold when use auto_threshold"]
pub type AF_GEN_THRESHOLD_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AF_GEN_THRESHOLD_MAX` reader - this field configures max threshold when use auto_threshold"]
pub type AF_GEN_THRESHOLD_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `AF_GEN_THRESHOLD_MAX` writer - this field configures max threshold when use auto_threshold"]
pub type AF_GEN_THRESHOLD_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - this field configures min threshold when use auto_threshold"]
    #[inline(always)]
    pub fn af_gen_threshold_min(&self) -> AF_GEN_THRESHOLD_MIN_R {
        AF_GEN_THRESHOLD_MIN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - this field configures max threshold when use auto_threshold"]
    #[inline(always)]
    pub fn af_gen_threshold_max(&self) -> AF_GEN_THRESHOLD_MAX_R {
        AF_GEN_THRESHOLD_MAX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_GEN_TH_CTRL")
            .field("af_gen_threshold_min", &self.af_gen_threshold_min())
            .field("af_gen_threshold_max", &self.af_gen_threshold_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - this field configures min threshold when use auto_threshold"]
    #[inline(always)]
    #[must_use]
    pub fn af_gen_threshold_min(&mut self) -> AF_GEN_THRESHOLD_MIN_W<AF_GEN_TH_CTRL_SPEC> {
        AF_GEN_THRESHOLD_MIN_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - this field configures max threshold when use auto_threshold"]
    #[inline(always)]
    #[must_use]
    pub fn af_gen_threshold_max(&mut self) -> AF_GEN_THRESHOLD_MAX_W<AF_GEN_TH_CTRL_SPEC> {
        AF_GEN_THRESHOLD_MAX_W::new(self, 16)
    }
}
#[doc = "af gen threshold control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_gen_th_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_gen_th_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_GEN_TH_CTRL_SPEC;
impl crate::RegisterSpec for AF_GEN_TH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_gen_th_ctrl::R`](R) reader structure"]
impl crate::Readable for AF_GEN_TH_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_gen_th_ctrl::W`](W) writer structure"]
impl crate::Writable for AF_GEN_TH_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF_GEN_TH_CTRL to value 0x0440_0080"]
impl crate::Resettable for AF_GEN_TH_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0440_0080;
}
