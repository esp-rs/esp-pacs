#[doc = "Register `VID_VBP_LINES` reader"]
pub type R = crate::R<VID_VBP_LINES_SPEC>;
#[doc = "Register `VID_VBP_LINES` writer"]
pub type W = crate::W<VID_VBP_LINES_SPEC>;
#[doc = "Field `VBP_LINES` reader - NA"]
pub type VBP_LINES_R = crate::FieldReader<u16>;
#[doc = "Field `VBP_LINES` writer - NA"]
pub type VBP_LINES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    pub fn vbp_lines(&self) -> VBP_LINES_R {
        VBP_LINES_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_VBP_LINES")
            .field("vbp_lines", &format_args!("{}", self.vbp_lines().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_VBP_LINES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vbp_lines(&mut self) -> VBP_LINES_W<VID_VBP_LINES_SPEC> {
        VBP_LINES_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_vbp_lines::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vbp_lines::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_VBP_LINES_SPEC;
impl crate::RegisterSpec for VID_VBP_LINES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_vbp_lines::R`](R) reader structure"]
impl crate::Readable for VID_VBP_LINES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_vbp_lines::W`](W) writer structure"]
impl crate::Writable for VID_VBP_LINES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_VBP_LINES to value 0"]
impl crate::Resettable for VID_VBP_LINES_SPEC {
    const RESET_VALUE: u32 = 0;
}
