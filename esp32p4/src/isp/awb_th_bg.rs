#[doc = "Register `AWB_TH_BG` reader"]
pub type R = crate::R<AWB_TH_BG_SPEC>;
#[doc = "Register `AWB_TH_BG` writer"]
pub type W = crate::W<AWB_TH_BG_SPEC>;
#[doc = "Field `AWB_MIN_BG` reader - this field configures lower threshold of b/g, 2bit integer and 8bit fraction"]
pub type AWB_MIN_BG_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_MIN_BG` writer - this field configures lower threshold of b/g, 2bit integer and 8bit fraction"]
pub type AWB_MIN_BG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `AWB_MAX_BG` reader - this field configures upper threshold of b/g, 2bit integer and 8bit fraction"]
pub type AWB_MAX_BG_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_MAX_BG` writer - this field configures upper threshold of b/g, 2bit integer and 8bit fraction"]
pub type AWB_MAX_BG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - this field configures lower threshold of b/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    pub fn awb_min_bg(&self) -> AWB_MIN_BG_R {
        AWB_MIN_BG_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - this field configures upper threshold of b/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    pub fn awb_max_bg(&self) -> AWB_MAX_BG_R {
        AWB_MAX_BG_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB_TH_BG")
            .field("awb_min_bg", &self.awb_min_bg().bits())
            .field("awb_max_bg", &self.awb_max_bg().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AWB_TH_BG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - this field configures lower threshold of b/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    #[must_use]
    pub fn awb_min_bg(&mut self) -> AWB_MIN_BG_W<AWB_TH_BG_SPEC> {
        AWB_MIN_BG_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - this field configures upper threshold of b/g, 2bit integer and 8bit fraction"]
    #[inline(always)]
    #[must_use]
    pub fn awb_max_bg(&mut self) -> AWB_MAX_BG_W<AWB_TH_BG_SPEC> {
        AWB_MAX_BG_W::new(self, 16)
    }
}
#[doc = "awb b/g threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_th_bg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_th_bg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB_TH_BG_SPEC;
impl crate::RegisterSpec for AWB_TH_BG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_th_bg::R`](R) reader structure"]
impl crate::Readable for AWB_TH_BG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awb_th_bg::W`](W) writer structure"]
impl crate::Writable for AWB_TH_BG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_TH_BG to value 0x03ff_0000"]
impl crate::Resettable for AWB_TH_BG_SPEC {
    const RESET_VALUE: u32 = 0x03ff_0000;
}
