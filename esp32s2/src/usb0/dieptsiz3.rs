#[doc = "Register `DIEPTSIZ3` reader"]
pub type R = crate::R<DIEPTSIZ3_SPEC>;
#[doc = "Register `DIEPTSIZ3` writer"]
pub type W = crate::W<DIEPTSIZ3_SPEC>;
#[doc = "Field `D_XFERSIZE3` reader - "]
pub type D_XFERSIZE3_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE3` writer - "]
pub type D_XFERSIZE3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `D_PKTCNT3` reader - "]
pub type D_PKTCNT3_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT3` writer - "]
pub type D_PKTCNT3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize3(&self) -> D_XFERSIZE3_R {
        D_XFERSIZE3_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt3(&self) -> D_PKTCNT3_R {
        D_PKTCNT3_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ3")
            .field(
                "d_xfersize3",
                &format_args!("{}", self.d_xfersize3().bits()),
            )
            .field("d_pktcnt3", &format_args!("{}", self.d_pktcnt3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize3(&mut self) -> D_XFERSIZE3_W<DIEPTSIZ3_SPEC> {
        D_XFERSIZE3_W::new(self, 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt3(&mut self) -> D_PKTCNT3_W<DIEPTSIZ3_SPEC> {
        D_PKTCNT3_W::new(self, 19)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ3_SPEC;
impl crate::RegisterSpec for DIEPTSIZ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz3::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz3::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ3 to value 0"]
impl crate::Resettable for DIEPTSIZ3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
