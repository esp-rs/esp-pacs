#[doc = "Register `GNPTXFSIZ` reader"]
pub type R = crate::R<GNPTXFSIZ_SPEC>;
#[doc = "Register `GNPTXFSIZ` writer"]
pub type W = crate::W<GNPTXFSIZ_SPEC>;
#[doc = "Field `NPTXFSTADDR` reader - "]
pub type NPTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSTADDR` writer - "]
pub type NPTXFSTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `NPTXFDEP` reader - "]
pub type NPTXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFDEP` writer - "]
pub type NPTXFDEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn nptxfdep(&self) -> NPTXFDEP_R {
        NPTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXFSIZ")
            .field(
                "nptxfstaddr",
                &format_args!("{}", self.nptxfstaddr().bits()),
            )
            .field("nptxfdep", &format_args!("{}", self.nptxfdep().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GNPTXFSIZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfstaddr(&mut self) -> NPTXFSTADDR_W<GNPTXFSIZ_SPEC, 0> {
        NPTXFSTADDR_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfdep(&mut self) -> NPTXFDEP_W<GNPTXFSIZ_SPEC, 16> {
        NPTXFDEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0100_0100"]
impl crate::Resettable for GNPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
