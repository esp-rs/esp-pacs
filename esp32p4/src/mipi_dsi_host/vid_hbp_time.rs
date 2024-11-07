#[doc = "Register `VID_HBP_TIME` reader"]
pub type R = crate::R<VID_HBP_TIME_SPEC>;
#[doc = "Register `VID_HBP_TIME` writer"]
pub type W = crate::W<VID_HBP_TIME_SPEC>;
#[doc = "Field `VID_HBP_TIME` reader - NA"]
pub type VID_HBP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `VID_HBP_TIME` writer - NA"]
pub type VID_HBP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    pub fn vid_hbp_time(&self) -> VID_HBP_TIME_R {
        VID_HBP_TIME_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_HBP_TIME")
            .field("vid_hbp_time", &self.vid_hbp_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    pub fn vid_hbp_time(&mut self) -> VID_HBP_TIME_W<VID_HBP_TIME_SPEC> {
        VID_HBP_TIME_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_hbp_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_hbp_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_HBP_TIME_SPEC;
impl crate::RegisterSpec for VID_HBP_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hbp_time::R`](R) reader structure"]
impl crate::Readable for VID_HBP_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_hbp_time::W`](W) writer structure"]
impl crate::Writable for VID_HBP_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_HBP_TIME to value 0"]
impl crate::Resettable for VID_HBP_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
