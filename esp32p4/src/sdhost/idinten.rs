#[doc = "Register `IDINTEN` reader"]
pub type R = crate::R<IDINTEN_SPEC>;
#[doc = "Register `IDINTEN` writer"]
pub type W = crate::W<IDINTEN_SPEC>;
#[doc = "Field `TI` reader - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
pub type TI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
pub type FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
pub type DU_R = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
pub type DU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CES` reader - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
pub type CES_R = crate::BitReader;
#[doc = "Field `CES` writer - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
pub type CES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NI` reader - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
pub type NI_R = crate::BitReader;
#[doc = "Field `NI` writer - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
pub type NI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AI` reader - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
pub type AI_R = crate::BitReader;
#[doc = "Field `AI` writer - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
pub type AI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
    #[inline(always)]
    pub fn ni(&self) -> NI_R {
        NI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDINTEN")
            .field("ti", &self.ti())
            .field("ri", &self.ri())
            .field("fbe", &self.fbe())
            .field("du", &self.du())
            .field("ces", &self.ces())
            .field("ni", &self.ni())
            .field("ai", &self.ai())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transmit Interrupt is enabled. When reset, Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<IDINTEN_SPEC> {
        TI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive Interrupt is enabled. When reset, Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<IDINTEN_SPEC> {
        RI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fatal Bus Error Interrupt is enabled. When reset, Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<IDINTEN_SPEC> {
        FBE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary Enable, the DU interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<IDINTEN_SPEC> {
        DU_W::new(self, 4)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable. When set, it enables the Card Interrupt summary."]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CES_W<IDINTEN_SPEC> {
        CES_W::new(self, 5)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When reset, a normal interrupt is disabled. This bit enables the following bits: IDINTEN\\[0\\]: Transmit Interrupt; IDINTEN\\[1\\]: Receive Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ni(&mut self) -> NI_W<IDINTEN_SPEC> {
        NI_W::new(self, 8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. This bit enables the following bits: IDINTEN\\[2\\]: Fatal Bus Error Interrupt; IDINTEN\\[4\\]: DU Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<IDINTEN_SPEC> {
        AI_W::new(self, 9)
    }
}
#[doc = "IDMAC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`idinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDINTEN_SPEC;
impl crate::RegisterSpec for IDINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idinten::R`](R) reader structure"]
impl crate::Readable for IDINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idinten::W`](W) writer structure"]
impl crate::Writable for IDINTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDINTEN to value 0"]
impl crate::Resettable for IDINTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
