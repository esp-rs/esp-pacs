#[doc = "Register `DOEPCTL` reader"]
pub type R = crate::R<DOEPCTL_SPEC>;
#[doc = "Register `DOEPCTL` writer"]
pub type W = crate::W<DOEPCTL_SPEC>;
#[doc = "Field `MPS` reader - "]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - "]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
#[doc = "Field `USBACTEP` reader - "]
pub type USBACTEP_R = crate::BitReader;
#[doc = "Field `USBACTEP` writer - "]
pub type USBACTEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKSTS` reader - "]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - "]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - "]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `SNP` reader - "]
pub type SNP_R = crate::BitReader;
#[doc = "Field `SNP` writer - "]
pub type SNP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - "]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - "]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - "]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - "]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PID` writer - "]
pub type SETD0PID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PID` writer - "]
pub type SETD1PID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - "]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDIS` writer - "]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - "]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - "]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL")
            .field("mps", &format_args!("{}", self.mps().bits()))
            .field("usbactep", &format_args!("{}", self.usbactep().bit()))
            .field("naksts", &format_args!("{}", self.naksts().bit()))
            .field("eptype", &format_args!("{}", self.eptype().bits()))
            .field("snp", &format_args!("{}", self.snp().bit()))
            .field("stall", &format_args!("{}", self.stall().bit()))
            .field("epdis", &format_args!("{}", self.epdis().bit()))
            .field("epena", &format_args!("{}", self.epena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DOEPCTL_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn usbactep(&mut self) -> USBACTEP_W<DOEPCTL_SPEC> {
        USBACTEP_W::new(self, 15)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DOEPCTL_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<DOEPCTL_SPEC> {
        SNP_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEPCTL_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEPCTL_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEPCTL_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn setd0pid(&mut self) -> SETD0PID_W<DOEPCTL_SPEC> {
        SETD0PID_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn setd1pid(&mut self) -> SETD1PID_W<DOEPCTL_SPEC> {
        SETD1PID_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<DOEPCTL_SPEC> {
        EPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DOEPCTL_SPEC> {
        EPENA_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL_SPEC;
impl crate::RegisterSpec for DOEPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl::R`](R) reader structure"]
impl crate::Readable for DOEPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl::W`](W) writer structure"]
impl crate::Writable for DOEPCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL to value 0x8000"]
impl crate::Resettable for DOEPCTL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
