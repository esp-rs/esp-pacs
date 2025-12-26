#[doc = "Register `BLERFTESTCNTL` reader"]
pub type R = crate::R<BLERFTESTCNTL_SPEC>;
#[doc = "Register `BLERFTESTCNTL` writer"]
pub type W = crate::W<BLERFTESTCNTL_SPEC>;
#[doc = "Field `TXLENGTH` reader - TX packet length in number of bytes"]
pub type TXLENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `TXLENGTH` writer - TX packet length in number of bytes"]
pub type TXLENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXPKTCNTEN` reader - Enable TX packet count"]
pub type TXPKTCNTEN_R = crate::BitReader;
#[doc = "Field `TXPKTCNTEN` writer - Enable TX packet count"]
pub type TXPKTCNTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPLDSRC` reader - TX packet payload source (0=Control Structure, 1=PRBS)"]
pub type TXPLDSRC_R = crate::BitReader;
#[doc = "Field `TXPLDSRC` writer - TX packet payload source (0=Control Structure, 1=PRBS)"]
pub type TXPLDSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRBSTYPE` reader - PRBS payload type (0=PRBS9, 1=PRBS15)"]
pub type PRBSTYPE_R = crate::BitReader;
#[doc = "Field `PRBSTYPE` writer - PRBS payload type (0=PRBS9, 1=PRBS15)"]
pub type PRBSTYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLENGTHSRC` reader - TX packet payload length source (0=TXADVLEN, 1=BLERFTESTCNTL.TXLENGTH)"]
pub type TXLENGTHSRC_R = crate::BitReader;
#[doc = "Field `TXLENGTHSRC` writer - TX packet payload length source (0=TXADVLEN, 1=BLERFTESTCNTL.TXLENGTH)"]
pub type TXLENGTHSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFINITETX` reader - Infinite TX packet"]
pub type INFINITETX_R = crate::BitReader;
#[doc = "Field `INFINITETX` writer - Infinite TX packet"]
pub type INFINITETX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPKTCNTEN` reader - Enable RX packet count"]
pub type RXPKTCNTEN_R = crate::BitReader;
#[doc = "Field `RXPKTCNTEN` writer - Enable RX packet count"]
pub type RXPKTCNTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFINITERX` reader - Infinite RX window"]
pub type INFINITERX_R = crate::BitReader;
#[doc = "Field `INFINITERX` writer - Infinite RX window"]
pub type INFINITERX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - TX packet length in number of bytes"]
    #[inline(always)]
    pub fn txlength(&self) -> TXLENGTH_R {
        TXLENGTH_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Enable TX packet count"]
    #[inline(always)]
    pub fn txpktcnten(&self) -> TXPKTCNTEN_R {
        TXPKTCNTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX packet payload source (0=Control Structure, 1=PRBS)"]
    #[inline(always)]
    pub fn txpldsrc(&self) -> TXPLDSRC_R {
        TXPLDSRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PRBS payload type (0=PRBS9, 1=PRBS15)"]
    #[inline(always)]
    pub fn prbstype(&self) -> PRBSTYPE_R {
        PRBSTYPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TX packet payload length source (0=TXADVLEN, 1=BLERFTESTCNTL.TXLENGTH)"]
    #[inline(always)]
    pub fn txlengthsrc(&self) -> TXLENGTHSRC_R {
        TXLENGTHSRC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Infinite TX packet"]
    #[inline(always)]
    pub fn infinitetx(&self) -> INFINITETX_R {
        INFINITETX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable RX packet count"]
    #[inline(always)]
    pub fn rxpktcnten(&self) -> RXPKTCNTEN_R {
        RXPKTCNTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Infinite RX window"]
    #[inline(always)]
    pub fn infiniterx(&self) -> INFINITERX_R {
        INFINITERX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERFTESTCNTL")
            .field("txlength", &self.txlength())
            .field("txpktcnten", &self.txpktcnten())
            .field("txpldsrc", &self.txpldsrc())
            .field("prbstype", &self.prbstype())
            .field("txlengthsrc", &self.txlengthsrc())
            .field("infinitetx", &self.infinitetx())
            .field("rxpktcnten", &self.rxpktcnten())
            .field("infiniterx", &self.infiniterx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - TX packet length in number of bytes"]
    #[inline(always)]
    pub fn txlength(&mut self) -> TXLENGTH_W<'_, BLERFTESTCNTL_SPEC> {
        TXLENGTH_W::new(self, 0)
    }
    #[doc = "Bit 11 - Enable TX packet count"]
    #[inline(always)]
    pub fn txpktcnten(&mut self) -> TXPKTCNTEN_W<'_, BLERFTESTCNTL_SPEC> {
        TXPKTCNTEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - TX packet payload source (0=Control Structure, 1=PRBS)"]
    #[inline(always)]
    pub fn txpldsrc(&mut self) -> TXPLDSRC_W<'_, BLERFTESTCNTL_SPEC> {
        TXPLDSRC_W::new(self, 12)
    }
    #[doc = "Bit 13 - PRBS payload type (0=PRBS9, 1=PRBS15)"]
    #[inline(always)]
    pub fn prbstype(&mut self) -> PRBSTYPE_W<'_, BLERFTESTCNTL_SPEC> {
        PRBSTYPE_W::new(self, 13)
    }
    #[doc = "Bit 14 - TX packet payload length source (0=TXADVLEN, 1=BLERFTESTCNTL.TXLENGTH)"]
    #[inline(always)]
    pub fn txlengthsrc(&mut self) -> TXLENGTHSRC_W<'_, BLERFTESTCNTL_SPEC> {
        TXLENGTHSRC_W::new(self, 14)
    }
    #[doc = "Bit 15 - Infinite TX packet"]
    #[inline(always)]
    pub fn infinitetx(&mut self) -> INFINITETX_W<'_, BLERFTESTCNTL_SPEC> {
        INFINITETX_W::new(self, 15)
    }
    #[doc = "Bit 27 - Enable RX packet count"]
    #[inline(always)]
    pub fn rxpktcnten(&mut self) -> RXPKTCNTEN_W<'_, BLERFTESTCNTL_SPEC> {
        RXPKTCNTEN_W::new(self, 27)
    }
    #[doc = "Bit 31 - Infinite RX window"]
    #[inline(always)]
    pub fn infiniterx(&mut self) -> INFINITERX_W<'_, BLERFTESTCNTL_SPEC> {
        INFINITERX_W::new(self, 31)
    }
}
#[doc = "Test Mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blerftestcntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerftestcntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLERFTESTCNTL_SPEC;
impl crate::RegisterSpec for BLERFTESTCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blerftestcntl::R`](R) reader structure"]
impl crate::Readable for BLERFTESTCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blerftestcntl::W`](W) writer structure"]
impl crate::Writable for BLERFTESTCNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLERFTESTCNTL to value 0"]
impl crate::Resettable for BLERFTESTCNTL_SPEC {}
