#[doc = "Register `DOEPTSIZ5` reader"]
pub type R = crate::R<DOEPTSIZ5_SPEC>;
#[doc = "Register `DOEPTSIZ5` writer"]
pub type W = crate::W<DOEPTSIZ5_SPEC>;
#[doc = "Field `XFERSIZE5` reader - "]
pub type XFERSIZE5_R = crate::FieldReader;
#[doc = "Field `XFERSIZE5` writer - "]
pub type XFERSIZE5_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT5` reader - "]
pub type PKTCNT5_R = crate::BitReader;
#[doc = "Field `PKTCNT5` writer - "]
pub type PKTCNT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPCNT5` reader - "]
pub type SUPCNT5_R = crate::FieldReader;
#[doc = "Field `SUPCNT5` writer - "]
pub type SUPCNT5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize5(&self) -> XFERSIZE5_R {
        XFERSIZE5_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt5(&self) -> PKTCNT5_R {
        PKTCNT5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt5(&self) -> SUPCNT5_R {
        SUPCNT5_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ5")
            .field("xfersize5", &format_args!("{}", self.xfersize5().bits()))
            .field("pktcnt5", &format_args!("{}", self.pktcnt5().bit()))
            .field("supcnt5", &format_args!("{}", self.supcnt5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize5(&mut self) -> XFERSIZE5_W<DOEPTSIZ5_SPEC> {
        XFERSIZE5_W::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt5(&mut self) -> PKTCNT5_W<DOEPTSIZ5_SPEC> {
        PKTCNT5_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt5(&mut self) -> SUPCNT5_W<DOEPTSIZ5_SPEC> {
        SUPCNT5_W::new(self, 29)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ5_SPEC;
impl crate::RegisterSpec for DOEPTSIZ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz5::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz5::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ5 to value 0"]
impl crate::Resettable for DOEPTSIZ5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
