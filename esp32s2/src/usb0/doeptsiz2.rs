#[doc = "Register `DOEPTSIZ2` reader"]
pub type R = crate::R<DOEPTSIZ2_SPEC>;
#[doc = "Register `DOEPTSIZ2` writer"]
pub type W = crate::W<DOEPTSIZ2_SPEC>;
#[doc = "Field `XFERSIZE2` reader - "]
pub type XFERSIZE2_R = crate::FieldReader;
#[doc = "Field `XFERSIZE2` writer - "]
pub type XFERSIZE2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT2` reader - "]
pub type PKTCNT2_R = crate::BitReader;
#[doc = "Field `PKTCNT2` writer - "]
pub type PKTCNT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPCNT2` reader - "]
pub type SUPCNT2_R = crate::FieldReader;
#[doc = "Field `SUPCNT2` writer - "]
pub type SUPCNT2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize2(&self) -> XFERSIZE2_R {
        XFERSIZE2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt2(&self) -> PKTCNT2_R {
        PKTCNT2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt2(&self) -> SUPCNT2_R {
        SUPCNT2_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ2")
            .field("xfersize2", &format_args!("{}", self.xfersize2().bits()))
            .field("pktcnt2", &format_args!("{}", self.pktcnt2().bit()))
            .field("supcnt2", &format_args!("{}", self.supcnt2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize2(&mut self) -> XFERSIZE2_W<DOEPTSIZ2_SPEC> {
        XFERSIZE2_W::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt2(&mut self) -> PKTCNT2_W<DOEPTSIZ2_SPEC> {
        PKTCNT2_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt2(&mut self) -> SUPCNT2_W<DOEPTSIZ2_SPEC> {
        SUPCNT2_W::new(self, 29)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ2_SPEC;
impl crate::RegisterSpec for DOEPTSIZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz2::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz2::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ2 to value 0"]
impl crate::Resettable for DOEPTSIZ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
