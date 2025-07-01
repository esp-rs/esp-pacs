#[doc = "Register `VID_VSA_LINES` reader"]
pub type R = crate::R<VID_VSA_LINES_SPEC>;
#[doc = "Register `VID_VSA_LINES` writer"]
pub type W = crate::W<VID_VSA_LINES_SPEC>;
#[doc = "Field `VSA_LINES` reader - NA"]
pub type VSA_LINES_R = crate::FieldReader<u16>;
#[doc = "Field `VSA_LINES` writer - NA"]
pub type VSA_LINES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vsa_lines(&self) -> VSA_LINES_R {
        VSA_LINES_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_VSA_LINES")
            .field("vsa_lines", &self.vsa_lines())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vsa_lines(&mut self) -> VSA_LINES_W<VID_VSA_LINES_SPEC> {
        VSA_LINES_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_vsa_lines::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_vsa_lines::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_VSA_LINES_SPEC;
impl crate::RegisterSpec for VID_VSA_LINES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vsa_lines::R`](R) reader structure"]
impl crate::Readable for VID_VSA_LINES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_vsa_lines::W`](W) writer structure"]
impl crate::Writable for VID_VSA_LINES_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_VSA_LINES to value 0"]
impl crate::Resettable for VID_VSA_LINES_SPEC {}
