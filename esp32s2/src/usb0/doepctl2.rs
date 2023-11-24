#[doc = "Register `DOEPCTL2` reader"]
pub type R = crate::R<DOEPCTL2_SPEC>;
#[doc = "Register `DOEPCTL2` writer"]
pub type W = crate::W<DOEPCTL2_SPEC>;
#[doc = "Field `MPS2` reader - "]
pub type MPS2_R = crate::FieldReader<u16>;
#[doc = "Field `USBACTEP2` reader - "]
pub type USBACTEP2_R = crate::BitReader;
#[doc = "Field `NAKSTS2` reader - "]
pub type NAKSTS2_R = crate::BitReader;
#[doc = "Field `EPTYPE2` reader - "]
pub type EPTYPE2_R = crate::FieldReader;
#[doc = "Field `SNP2` reader - "]
pub type SNP2_R = crate::BitReader;
#[doc = "Field `SNP2` writer - "]
pub type SNP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL2` reader - "]
pub type STALL2_R = crate::BitReader;
#[doc = "Field `STALL2` writer - "]
pub type STALL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK2` writer - "]
pub type CNAK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_SNAK2` writer - "]
pub type DO_SNAK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_SETD0PID2` writer - "]
pub type DO_SETD0PID2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_SETD1PID2` writer - "]
pub type DO_SETD1PID2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS2` reader - "]
pub type EPDIS2_R = crate::BitReader;
#[doc = "Field `EPENA2` reader - "]
pub type EPENA2_R = crate::BitReader;
#[doc = "Field `EPENA2` writer - "]
pub type EPENA2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps2(&self) -> MPS2_R {
        MPS2_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep2(&self) -> USBACTEP2_R {
        USBACTEP2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts2(&self) -> NAKSTS2_R {
        NAKSTS2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype2(&self) -> EPTYPE2_R {
        EPTYPE2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp2(&self) -> SNP2_R {
        SNP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall2(&self) -> STALL2_R {
        STALL2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis2(&self) -> EPDIS2_R {
        EPDIS2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena2(&self) -> EPENA2_R {
        EPENA2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL2")
            .field("mps2", &format_args!("{}", self.mps2().bits()))
            .field("usbactep2", &format_args!("{}", self.usbactep2().bit()))
            .field("naksts2", &format_args!("{}", self.naksts2().bit()))
            .field("eptype2", &format_args!("{}", self.eptype2().bits()))
            .field("snp2", &format_args!("{}", self.snp2().bit()))
            .field("stall2", &format_args!("{}", self.stall2().bit()))
            .field("epdis2", &format_args!("{}", self.epdis2().bit()))
            .field("epena2", &format_args!("{}", self.epena2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp2(&mut self) -> SNP2_W<DOEPCTL2_SPEC> {
        SNP2_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall2(&mut self) -> STALL2_W<DOEPCTL2_SPEC> {
        STALL2_W::new(self, 21)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak2(&mut self) -> CNAK2_W<DOEPCTL2_SPEC> {
        CNAK2_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak2(&mut self) -> DO_SNAK2_W<DOEPCTL2_SPEC> {
        DO_SNAK2_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd0pid2(&mut self) -> DO_SETD0PID2_W<DOEPCTL2_SPEC> {
        DO_SETD0PID2_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd1pid2(&mut self) -> DO_SETD1PID2_W<DOEPCTL2_SPEC> {
        DO_SETD1PID2_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena2(&mut self) -> EPENA2_W<DOEPCTL2_SPEC> {
        EPENA2_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL2_SPEC;
impl crate::RegisterSpec for DOEPCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl2::R`](R) reader structure"]
impl crate::Readable for DOEPCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl2::W`](W) writer structure"]
impl crate::Writable for DOEPCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL2 to value 0x8000"]
impl crate::Resettable for DOEPCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
