#[doc = "Register `NTIMG_DATE` reader"]
pub type R = crate::R<NTIMG_DATE_SPEC>;
#[doc = "Register `NTIMG_DATE` writer"]
pub type W = crate::W<NTIMG_DATE_SPEC>;
#[doc = "Field `NTIMGS_DATE` reader - reg_ntimers_date."]
pub type NTIMGS_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `NTIMGS_DATE` writer - reg_ntimers_date."]
pub type NTIMGS_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - reg_ntimers_date."]
    #[inline(always)]
    pub fn ntimgs_date(&self) -> NTIMGS_DATE_R {
        NTIMGS_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NTIMG_DATE")
            .field(
                "ntimgs_date",
                &format_args!("{}", self.ntimgs_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<NTIMG_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - reg_ntimers_date."]
    #[inline(always)]
    #[must_use]
    pub fn ntimgs_date(&mut self) -> NTIMGS_DATE_W<NTIMG_DATE_SPEC> {
        NTIMGS_DATE_W::new(self, 0)
    }
}
#[doc = "TIMG_NTIMG_DATE_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ntimg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NTIMG_DATE_SPEC;
impl crate::RegisterSpec for NTIMG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ntimg_date::R`](R) reader structure"]
impl crate::Readable for NTIMG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ntimg_date::W`](W) writer structure"]
impl crate::Writable for NTIMG_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NTIMG_DATE to value 0x0200_6191"]
impl crate::Resettable for NTIMG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0200_6191;
}
