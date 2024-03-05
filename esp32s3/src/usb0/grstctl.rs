#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTL_SPEC>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTL_SPEC>;
#[doc = "Field `CSFTRST` reader - "]
pub type CSFTRST_R = crate::BitReader;
#[doc = "Field `CSFTRST` writer - "]
pub type CSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIUFSSFTRST` reader - "]
pub type PIUFSSFTRST_R = crate::BitReader;
#[doc = "Field `PIUFSSFTRST` writer - "]
pub type PIUFSSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMCNTRRST` reader - "]
pub type FRMCNTRRST_R = crate::BitReader;
#[doc = "Field `FRMCNTRRST` writer - "]
pub type FRMCNTRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - "]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - "]
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - "]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - "]
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - "]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - "]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMAREQ` reader - "]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDLE` reader - "]
pub type AHBIDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PIUFSSFTRST_R {
        PIUFSSFTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frmcntrrst(&self) -> FRMCNTRRST_R {
        FRMCNTRRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csftrst", &format_args!("{}", self.csftrst().bit()))
            .field("piufssftrst", &format_args!("{}", self.piufssftrst().bit()))
            .field("frmcntrrst", &format_args!("{}", self.frmcntrrst().bit()))
            .field("rxfflsh", &format_args!("{}", self.rxfflsh().bit()))
            .field("txfflsh", &format_args!("{}", self.txfflsh().bit()))
            .field("txfnum", &format_args!("{}", self.txfnum().bits()))
            .field("dmareq", &format_args!("{}", self.dmareq().bit()))
            .field("ahbidle", &format_args!("{}", self.ahbidle().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GRSTCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn csftrst(&mut self) -> CSFTRST_W<GRSTCTL_SPEC> {
        CSFTRST_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn piufssftrst(&mut self) -> PIUFSSFTRST_W<GRSTCTL_SPEC> {
        PIUFSSFTRST_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn frmcntrrst(&mut self) -> FRMCNTRRST_W<GRSTCTL_SPEC> {
        FRMCNTRRST_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<GRSTCTL_SPEC> {
        RXFFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<GRSTCTL_SPEC> {
        TXFFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<GRSTCTL_SPEC> {
        TXFNUM_W::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
