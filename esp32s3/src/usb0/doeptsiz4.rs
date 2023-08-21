#[doc = "Register `DOEPTSIZ4` reader"]
pub type R = crate::R<DOEPTSIZ4_SPEC>;
#[doc = "Register `DOEPTSIZ4` writer"]
pub type W = crate::W<DOEPTSIZ4_SPEC>;
#[doc = "Field `XFERSIZE4` reader - "]
pub type XFERSIZE4_R = crate::FieldReader;
#[doc = "Field `XFERSIZE4` writer - "]
pub type XFERSIZE4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PKTCNT4` reader - "]
pub type PKTCNT4_R = crate::BitReader;
#[doc = "Field `PKTCNT4` writer - "]
pub type PKTCNT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUPCNT4` reader - "]
pub type SUPCNT4_R = crate::FieldReader;
#[doc = "Field `SUPCNT4` writer - "]
pub type SUPCNT4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize4(&self) -> XFERSIZE4_R {
        XFERSIZE4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt4(&self) -> PKTCNT4_R {
        PKTCNT4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt4(&self) -> SUPCNT4_R {
        SUPCNT4_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ4")
            .field("xfersize4", &format_args!("{}", self.xfersize4().bits()))
            .field("pktcnt4", &format_args!("{}", self.pktcnt4().bit()))
            .field("supcnt4", &format_args!("{}", self.supcnt4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize4(&mut self) -> XFERSIZE4_W<DOEPTSIZ4_SPEC, 0> {
        XFERSIZE4_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt4(&mut self) -> PKTCNT4_W<DOEPTSIZ4_SPEC, 19> {
        PKTCNT4_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt4(&mut self) -> SUPCNT4_W<DOEPTSIZ4_SPEC, 29> {
        SUPCNT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ4_SPEC;
impl crate::RegisterSpec for DOEPTSIZ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz4::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz4::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ4 to value 0"]
impl crate::Resettable for DOEPTSIZ4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
