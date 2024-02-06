#[doc = "Register `DIEPTSIZ1` reader"]
pub type R = crate::R<DIEPTSIZ1_SPEC>;
#[doc = "Register `DIEPTSIZ1` writer"]
pub type W = crate::W<DIEPTSIZ1_SPEC>;
#[doc = "Field `D_XFERSIZE1` reader - "]
pub type D_XFERSIZE1_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE1` writer - "]
pub type D_XFERSIZE1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `D_PKTCNT1` reader - "]
pub type D_PKTCNT1_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT1` writer - "]
pub type D_PKTCNT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize1(&self) -> D_XFERSIZE1_R {
        D_XFERSIZE1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt1(&self) -> D_PKTCNT1_R {
        D_PKTCNT1_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ1")
            .field(
                "d_xfersize1",
                &format_args!("{}", self.d_xfersize1().bits()),
            )
            .field("d_pktcnt1", &format_args!("{}", self.d_pktcnt1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize1(&mut self) -> D_XFERSIZE1_W<DIEPTSIZ1_SPEC> {
        D_XFERSIZE1_W::new(self, 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt1(&mut self) -> D_PKTCNT1_W<DIEPTSIZ1_SPEC> {
        D_PKTCNT1_W::new(self, 19)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ1_SPEC;
impl crate::RegisterSpec for DIEPTSIZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz1::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz1::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ1 to value 0"]
impl crate::Resettable for DIEPTSIZ1_SPEC {
    const RESET_VALUE: u32 = 0;
}
