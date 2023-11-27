#[doc = "Register `VID_HSA_TIME` reader"]
pub type R = crate::R<VID_HSA_TIME_SPEC>;
#[doc = "Register `VID_HSA_TIME` writer"]
pub type W = crate::W<VID_HSA_TIME_SPEC>;
#[doc = "Field `VID_HSA_TIME` reader - NA"]
pub type VID_HSA_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `VID_HSA_TIME` writer - NA"]
pub type VID_HSA_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    pub fn vid_hsa_time(&self) -> VID_HSA_TIME_R {
        VID_HSA_TIME_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_HSA_TIME")
            .field(
                "vid_hsa_time",
                &format_args!("{}", self.vid_hsa_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_HSA_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid_hsa_time(&mut self) -> VID_HSA_TIME_W<VID_HSA_TIME_SPEC> {
        VID_HSA_TIME_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_hsa_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hsa_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_HSA_TIME_SPEC;
impl crate::RegisterSpec for VID_HSA_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_hsa_time::R`](R) reader structure"]
impl crate::Readable for VID_HSA_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_hsa_time::W`](W) writer structure"]
impl crate::Writable for VID_HSA_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VID_HSA_TIME to value 0"]
impl crate::Resettable for VID_HSA_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
