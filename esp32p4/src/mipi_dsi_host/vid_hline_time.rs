#[doc = "Register `VID_HLINE_TIME` reader"]
pub type R = crate::R<VID_HLINE_TIME_SPEC>;
#[doc = "Register `VID_HLINE_TIME` writer"]
pub type W = crate::W<VID_HLINE_TIME_SPEC>;
#[doc = "Field `VID_HLINE_TIME` reader - NA"]
pub type VID_HLINE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `VID_HLINE_TIME` writer - NA"]
pub type VID_HLINE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn vid_hline_time(&self) -> VID_HLINE_TIME_R {
        VID_HLINE_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_HLINE_TIME")
            .field("vid_hline_time", &self.vid_hline_time().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_HLINE_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid_hline_time(&mut self) -> VID_HLINE_TIME_W<VID_HLINE_TIME_SPEC> {
        VID_HLINE_TIME_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hline_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hline_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_HLINE_TIME_SPEC;
impl crate::RegisterSpec for VID_HLINE_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hline_time::R`](R) reader structure"]
impl crate::Readable for VID_HLINE_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_hline_time::W`](W) writer structure"]
impl crate::Writable for VID_HLINE_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_HLINE_TIME to value 0"]
impl crate::Resettable for VID_HLINE_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
