#[doc = "Register `DIEPTSIZ5` reader"]
pub type R = crate::R<DIEPTSIZ5_SPEC>;
#[doc = "Register `DIEPTSIZ5` writer"]
pub type W = crate::W<DIEPTSIZ5_SPEC>;
#[doc = "Field `D_XFERSIZE5` reader - "]
pub type D_XFERSIZE5_R = crate::FieldReader;
#[doc = "Field `D_XFERSIZE5` writer - "]
pub type D_XFERSIZE5_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `D_PKTCNT5` reader - "]
pub type D_PKTCNT5_R = crate::FieldReader;
#[doc = "Field `D_PKTCNT5` writer - "]
pub type D_PKTCNT5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn d_xfersize5(&self) -> D_XFERSIZE5_R {
        D_XFERSIZE5_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn d_pktcnt5(&self) -> D_PKTCNT5_R {
        D_PKTCNT5_R::new(((self.bits >> 19) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ5")
            .field(
                "d_xfersize5",
                &format_args!("{}", self.d_xfersize5().bits()),
            )
            .field("d_pktcnt5", &format_args!("{}", self.d_pktcnt5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfersize5(&mut self) -> D_XFERSIZE5_W<DIEPTSIZ5_SPEC> {
        D_XFERSIZE5_W::new(self, 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktcnt5(&mut self) -> D_PKTCNT5_W<DIEPTSIZ5_SPEC> {
        D_PKTCNT5_W::new(self, 19)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ5_SPEC;
impl crate::RegisterSpec for DIEPTSIZ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz5::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz5::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ5 to value 0"]
impl crate::Resettable for DIEPTSIZ5_SPEC {
    const RESET_VALUE: u32 = 0;
}
