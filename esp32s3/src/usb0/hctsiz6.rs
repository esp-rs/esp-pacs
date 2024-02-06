#[doc = "Register `HCTSIZ6` reader"]
pub type R = crate::R<HCTSIZ6_SPEC>;
#[doc = "Register `HCTSIZ6` writer"]
pub type W = crate::W<HCTSIZ6_SPEC>;
#[doc = "Field `H_XFERSIZE6` reader - "]
pub type H_XFERSIZE6_R = crate::FieldReader<u32>;
#[doc = "Field `H_XFERSIZE6` writer - "]
pub type H_XFERSIZE6_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `H_PKTCNT6` reader - "]
pub type H_PKTCNT6_R = crate::FieldReader<u16>;
#[doc = "Field `H_PKTCNT6` writer - "]
pub type H_PKTCNT6_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `H_PID6` reader - "]
pub type H_PID6_R = crate::FieldReader;
#[doc = "Field `H_PID6` writer - "]
pub type H_PID6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_DOPNG6` reader - "]
pub type H_DOPNG6_R = crate::BitReader;
#[doc = "Field `H_DOPNG6` writer - "]
pub type H_DOPNG6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize6(&self) -> H_XFERSIZE6_R {
        H_XFERSIZE6_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt6(&self) -> H_PKTCNT6_R {
        H_PKTCNT6_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid6(&self) -> H_PID6_R {
        H_PID6_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng6(&self) -> H_DOPNG6_R {
        H_DOPNG6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ6")
            .field(
                "h_xfersize6",
                &format_args!("{}", self.h_xfersize6().bits()),
            )
            .field("h_pktcnt6", &format_args!("{}", self.h_pktcnt6().bits()))
            .field("h_pid6", &format_args!("{}", self.h_pid6().bits()))
            .field("h_dopng6", &format_args!("{}", self.h_dopng6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCTSIZ6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfersize6(&mut self) -> H_XFERSIZE6_W<HCTSIZ6_SPEC> {
        H_XFERSIZE6_W::new(self, 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_pktcnt6(&mut self) -> H_PKTCNT6_W<HCTSIZ6_SPEC> {
        H_PKTCNT6_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn h_pid6(&mut self) -> H_PID6_W<HCTSIZ6_SPEC> {
        H_PID6_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dopng6(&mut self) -> H_DOPNG6_W<HCTSIZ6_SPEC> {
        H_DOPNG6_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ6_SPEC;
impl crate::RegisterSpec for HCTSIZ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz6::R`](R) reader structure"]
impl crate::Readable for HCTSIZ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz6::W`](W) writer structure"]
impl crate::Writable for HCTSIZ6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ6 to value 0"]
impl crate::Resettable for HCTSIZ6_SPEC {
    const RESET_VALUE: u32 = 0;
}
