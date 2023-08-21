#[doc = "Register `DIEPTSIZ6` reader"]
pub type R = crate::R<DIEPTSIZ6_SPEC>;
#[doc = "Register `DIEPTSIZ6` writer"]
pub type W = crate::W<DIEPTSIZ6_SPEC>;
#[doc = "Field `D_XFERSIZE6` reader - "]
pub type D_XFERSIZE6_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE6` writer - "]
pub type D_XFERSIZE6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `D_PKTCNT6` reader - "]
pub type D_PKTCNT6_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT6` writer - "]
pub type D_PKTCNT6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize6(&self) -> D_XFERSIZE6_R {
        D_XFERSIZE6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt6(&self) -> D_PKTCNT6_R {
        D_PKTCNT6_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ6")
            .field(
                "d_xfersize6",
                &format_args!("{}", self.d_xfersize6().bits()),
            )
            .field("d_pktcnt6", &format_args!("{}", self.d_pktcnt6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize6(&mut self) -> D_XFERSIZE6_W<DIEPTSIZ6_SPEC, 0> {
        D_XFERSIZE6_W::new(self)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt6(&mut self) -> D_PKTCNT6_W<DIEPTSIZ6_SPEC, 19> {
        D_PKTCNT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ6_SPEC;
impl crate::RegisterSpec for DIEPTSIZ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz6::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz6::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ6 to value 0"]
impl crate::Resettable for DIEPTSIZ6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
