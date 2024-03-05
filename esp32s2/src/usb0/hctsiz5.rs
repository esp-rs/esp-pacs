#[doc = "Register `HCTSIZ5` reader"]
pub type R = crate::R<HCTSIZ5_SPEC>;
#[doc = "Register `HCTSIZ5` writer"]
pub type W = crate::W<HCTSIZ5_SPEC>;
#[doc = "Field `H_XFERSIZE5` reader - "]
pub type H_XFERSIZE5_R = crate::FieldReader<u32>;
#[doc = "Field `H_XFERSIZE5` writer - "]
pub type H_XFERSIZE5_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `H_PKTCNT5` reader - "]
pub type H_PKTCNT5_R = crate::FieldReader<u16>;
#[doc = "Field `H_PKTCNT5` writer - "]
pub type H_PKTCNT5_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `H_PID5` reader - "]
pub type H_PID5_R = crate::FieldReader;
#[doc = "Field `H_PID5` writer - "]
pub type H_PID5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_DOPNG5` reader - "]
pub type H_DOPNG5_R = crate::BitReader;
#[doc = "Field `H_DOPNG5` writer - "]
pub type H_DOPNG5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize5(&self) -> H_XFERSIZE5_R {
        H_XFERSIZE5_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt5(&self) -> H_PKTCNT5_R {
        H_PKTCNT5_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid5(&self) -> H_PID5_R {
        H_PID5_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng5(&self) -> H_DOPNG5_R {
        H_DOPNG5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ5")
            .field(
                "h_xfersize5",
                &format_args!("{}", self.h_xfersize5().bits()),
            )
            .field("h_pktcnt5", &format_args!("{}", self.h_pktcnt5().bits()))
            .field("h_pid5", &format_args!("{}", self.h_pid5().bits()))
            .field("h_dopng5", &format_args!("{}", self.h_dopng5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCTSIZ5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfersize5(&mut self) -> H_XFERSIZE5_W<HCTSIZ5_SPEC> {
        H_XFERSIZE5_W::new(self, 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_pktcnt5(&mut self) -> H_PKTCNT5_W<HCTSIZ5_SPEC> {
        H_PKTCNT5_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn h_pid5(&mut self) -> H_PID5_W<HCTSIZ5_SPEC> {
        H_PID5_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dopng5(&mut self) -> H_DOPNG5_W<HCTSIZ5_SPEC> {
        H_DOPNG5_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTSIZ5_SPEC;
impl crate::RegisterSpec for HCTSIZ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz5::R`](R) reader structure"]
impl crate::Readable for HCTSIZ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctsiz5::W`](W) writer structure"]
impl crate::Writable for HCTSIZ5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ5 to value 0"]
impl crate::Resettable for HCTSIZ5_SPEC {
    const RESET_VALUE: u32 = 0;
}
