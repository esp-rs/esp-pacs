#[doc = "Register `DOEPTSIZ1` reader"]
pub type R = crate::R<DOEPTSIZ1_SPEC>;
#[doc = "Register `DOEPTSIZ1` writer"]
pub type W = crate::W<DOEPTSIZ1_SPEC>;
#[doc = "Field `XFERSIZE1` reader - "]
pub type XFERSIZE1_R = crate::FieldReader;
#[doc = "Field `XFERSIZE1` writer - "]
pub type XFERSIZE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PKTCNT1` reader - "]
pub type PKTCNT1_R = crate::BitReader;
#[doc = "Field `PKTCNT1` writer - "]
pub type PKTCNT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUPCNT1` reader - "]
pub type SUPCNT1_R = crate::FieldReader;
#[doc = "Field `SUPCNT1` writer - "]
pub type SUPCNT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize1(&self) -> XFERSIZE1_R {
        XFERSIZE1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt1(&self) -> PKTCNT1_R {
        PKTCNT1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt1(&self) -> SUPCNT1_R {
        SUPCNT1_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ1")
            .field("xfersize1", &format_args!("{}", self.xfersize1().bits()))
            .field("pktcnt1", &format_args!("{}", self.pktcnt1().bit()))
            .field("supcnt1", &format_args!("{}", self.supcnt1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize1(&mut self) -> XFERSIZE1_W<DOEPTSIZ1_SPEC, 0> {
        XFERSIZE1_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt1(&mut self) -> PKTCNT1_W<DOEPTSIZ1_SPEC, 19> {
        PKTCNT1_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt1(&mut self) -> SUPCNT1_W<DOEPTSIZ1_SPEC, 29> {
        SUPCNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ1_SPEC;
impl crate::RegisterSpec for DOEPTSIZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz1::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz1::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ1 to value 0"]
impl crate::Resettable for DOEPTSIZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
