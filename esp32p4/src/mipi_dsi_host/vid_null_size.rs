#[doc = "Register `VID_NULL_SIZE` reader"]
pub type R = crate::R<VID_NULL_SIZE_SPEC>;
#[doc = "Register `VID_NULL_SIZE` writer"]
pub type W = crate::W<VID_NULL_SIZE_SPEC>;
#[doc = "Field `VID_NULL_SIZE` reader - NA"]
pub type VID_NULL_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `VID_NULL_SIZE` writer - NA"]
pub type VID_NULL_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - NA"]
    #[inline(always)]
    pub fn vid_null_size(&self) -> VID_NULL_SIZE_R {
        VID_NULL_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_NULL_SIZE")
            .field("vid_null_size", &self.vid_null_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - NA"]
    #[inline(always)]
    pub fn vid_null_size(&mut self) -> VID_NULL_SIZE_W<'_, VID_NULL_SIZE_SPEC> {
        VID_NULL_SIZE_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_null_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_null_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_NULL_SIZE_SPEC;
impl crate::RegisterSpec for VID_NULL_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_null_size::R`](R) reader structure"]
impl crate::Readable for VID_NULL_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_null_size::W`](W) writer structure"]
impl crate::Writable for VID_NULL_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_NULL_SIZE to value 0"]
impl crate::Resettable for VID_NULL_SIZE_SPEC {}
