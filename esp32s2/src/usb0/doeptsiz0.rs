#[doc = "Register `DOEPTSIZ0` reader"]
pub type R = crate::R<DOEPTSIZ0_SPEC>;
#[doc = "Register `DOEPTSIZ0` writer"]
pub type W = crate::W<DOEPTSIZ0_SPEC>;
#[doc = "Field `XFERSIZE0` reader - "]
pub type XFERSIZE0_R = crate::FieldReader;
#[doc = "Field `XFERSIZE0` writer - "]
pub type XFERSIZE0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT0` reader - "]
pub type PKTCNT0_R = crate::BitReader;
#[doc = "Field `PKTCNT0` writer - "]
pub type PKTCNT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPCNT0` reader - "]
pub type SUPCNT0_R = crate::FieldReader;
#[doc = "Field `SUPCNT0` writer - "]
pub type SUPCNT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize0(&self) -> XFERSIZE0_R {
        XFERSIZE0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt0(&self) -> PKTCNT0_R {
        PKTCNT0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt0(&self) -> SUPCNT0_R {
        SUPCNT0_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ0")
            .field("xfersize0", &format_args!("{}", self.xfersize0().bits()))
            .field("pktcnt0", &format_args!("{}", self.pktcnt0().bit()))
            .field("supcnt0", &format_args!("{}", self.supcnt0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize0(&mut self) -> XFERSIZE0_W<DOEPTSIZ0_SPEC> {
        XFERSIZE0_W::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt0(&mut self) -> PKTCNT0_W<DOEPTSIZ0_SPEC> {
        PKTCNT0_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt0(&mut self) -> SUPCNT0_W<DOEPTSIZ0_SPEC> {
        SUPCNT0_W::new(self, 29)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ0_SPEC;
impl crate::RegisterSpec for DOEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz0::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz0::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for DOEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
