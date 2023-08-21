#[doc = "Register `FOSC_CNTL` reader"]
pub type R = crate::R<FOSC_CNTL_SPEC>;
#[doc = "Register `FOSC_CNTL` writer"]
pub type W = crate::W<FOSC_CNTL_SPEC>;
#[doc = "Field `FOSC_DFREQ` reader - need_des"]
pub type FOSC_DFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `FOSC_DFREQ` writer - need_des"]
pub type FOSC_DFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn fosc_dfreq(&self) -> FOSC_DFREQ_R {
        FOSC_DFREQ_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FOSC_CNTL")
            .field("fosc_dfreq", &format_args!("{}", self.fosc_dfreq().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FOSC_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn fosc_dfreq(&mut self) -> FOSC_DFREQ_W<FOSC_CNTL_SPEC, 22> {
        FOSC_DFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fosc_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fosc_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FOSC_CNTL_SPEC;
impl crate::RegisterSpec for FOSC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fosc_cntl::R`](R) reader structure"]
impl crate::Readable for FOSC_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fosc_cntl::W`](W) writer structure"]
impl crate::Writable for FOSC_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FOSC_CNTL to value 0x2b00_0000"]
impl crate::Resettable for FOSC_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b00_0000;
}
