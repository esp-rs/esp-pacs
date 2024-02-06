#[doc = "Register `BROWN_OUT` reader"]
pub type R = crate::R<BROWN_OUT_SPEC>;
#[doc = "Register `BROWN_OUT` writer"]
pub type W = crate::W<BROWN_OUT_SPEC>;
#[doc = "Field `BROWN_OUT2_ENA` reader - Enables the brown_out2 to initiate a chip reset."]
pub type BROWN_OUT2_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT2_ENA` writer - Enables the brown_out2 to initiate a chip reset."]
pub type BROWN_OUT2_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_WAIT` reader - Configures the waiting cycle before sending an interrupt."]
pub type INT_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `INT_WAIT` writer - Configures the waiting cycle before sending an interrupt."]
pub type INT_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CLOSE_FLASH_ENA` reader - Set this bit to enable PD the flash when a brown-out happens."]
pub type CLOSE_FLASH_ENA_R = crate::BitReader;
#[doc = "Field `CLOSE_FLASH_ENA` writer - Set this bit to enable PD the flash when a brown-out happens."]
pub type CLOSE_FLASH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_RF_ENA` reader - Set this bit to enable PD the RF circuits when a brown-out happens."]
pub type PD_RF_ENA_R = crate::BitReader;
#[doc = "Field `PD_RF_ENA` writer - Set this bit to enable PD the RF circuits when a brown-out happens."]
pub type PD_RF_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_WAIT` reader - Configures the waiting cycle before the reset after a brown-out."]
pub type RST_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `RST_WAIT` writer - Configures the waiting cycle before the reset after a brown-out."]
pub type RST_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RST_ENA` reader - Enables to reset brown-out."]
pub type RST_ENA_R = crate::BitReader;
#[doc = "Field `RST_ENA` writer - Enables to reset brown-out."]
pub type RST_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_SEL` reader - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset."]
pub type RST_SEL_R = crate::BitReader;
#[doc = "Field `RST_SEL` writer - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset."]
pub type RST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_CLR` writer - Clears the brown-out counter."]
pub type CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA` reader - Set this bit to enable brown-out detection."]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - Set this bit to enable brown-out detection."]
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DET` reader - Indicates the status of the brown-out signal."]
pub type DET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enables the brown_out2 to initiate a chip reset."]
    #[inline(always)]
    pub fn brown_out2_ena(&self) -> BROWN_OUT2_ENA_R {
        BROWN_OUT2_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:13 - Configures the waiting cycle before sending an interrupt."]
    #[inline(always)]
    pub fn int_wait(&self) -> INT_WAIT_R {
        INT_WAIT_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Set this bit to enable PD the flash when a brown-out happens."]
    #[inline(always)]
    pub fn close_flash_ena(&self) -> CLOSE_FLASH_ENA_R {
        CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable PD the RF circuits when a brown-out happens."]
    #[inline(always)]
    pub fn pd_rf_ena(&self) -> PD_RF_ENA_R {
        PD_RF_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Configures the waiting cycle before the reset after a brown-out."]
    #[inline(always)]
    pub fn rst_wait(&self) -> RST_WAIT_R {
        RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Enables to reset brown-out."]
    #[inline(always)]
    pub fn rst_ena(&self) -> RST_ENA_R {
        RST_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset."]
    #[inline(always)]
    pub fn rst_sel(&self) -> RST_SEL_R {
        RST_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable brown-out detection."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates the status of the brown-out signal."]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BROWN_OUT")
            .field(
                "brown_out2_ena",
                &format_args!("{}", self.brown_out2_ena().bit()),
            )
            .field("int_wait", &format_args!("{}", self.int_wait().bits()))
            .field(
                "close_flash_ena",
                &format_args!("{}", self.close_flash_ena().bit()),
            )
            .field("pd_rf_ena", &format_args!("{}", self.pd_rf_ena().bit()))
            .field("rst_wait", &format_args!("{}", self.rst_wait().bits()))
            .field("rst_ena", &format_args!("{}", self.rst_ena().bit()))
            .field("rst_sel", &format_args!("{}", self.rst_sel().bit()))
            .field("ena", &format_args!("{}", self.ena().bit()))
            .field("det", &format_args!("{}", self.det().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BROWN_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the brown_out2 to initiate a chip reset."]
    #[inline(always)]
    #[must_use]
    pub fn brown_out2_ena(&mut self) -> BROWN_OUT2_ENA_W<BROWN_OUT_SPEC> {
        BROWN_OUT2_ENA_W::new(self, 0)
    }
    #[doc = "Bits 4:13 - Configures the waiting cycle before sending an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn int_wait(&mut self) -> INT_WAIT_W<BROWN_OUT_SPEC> {
        INT_WAIT_W::new(self, 4)
    }
    #[doc = "Bit 14 - Set this bit to enable PD the flash when a brown-out happens."]
    #[inline(always)]
    #[must_use]
    pub fn close_flash_ena(&mut self) -> CLOSE_FLASH_ENA_W<BROWN_OUT_SPEC> {
        CLOSE_FLASH_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to enable PD the RF circuits when a brown-out happens."]
    #[inline(always)]
    #[must_use]
    pub fn pd_rf_ena(&mut self) -> PD_RF_ENA_W<BROWN_OUT_SPEC> {
        PD_RF_ENA_W::new(self, 15)
    }
    #[doc = "Bits 16:25 - Configures the waiting cycle before the reset after a brown-out."]
    #[inline(always)]
    #[must_use]
    pub fn rst_wait(&mut self) -> RST_WAIT_W<BROWN_OUT_SPEC> {
        RST_WAIT_W::new(self, 16)
    }
    #[doc = "Bit 26 - Enables to reset brown-out."]
    #[inline(always)]
    #[must_use]
    pub fn rst_ena(&mut self) -> RST_ENA_W<BROWN_OUT_SPEC> {
        RST_ENA_W::new(self, 26)
    }
    #[doc = "Bit 27 - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst_sel(&mut self) -> RST_SEL_W<BROWN_OUT_SPEC> {
        RST_SEL_W::new(self, 27)
    }
    #[doc = "Bit 29 - Clears the brown-out counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_clr(&mut self) -> CNT_CLR_W<BROWN_OUT_SPEC> {
        CNT_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to enable brown-out detection."]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<BROWN_OUT_SPEC> {
        ENA_W::new(self, 30)
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
#[doc = "Brownout configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brown_out::R`](R) reader structure"]
impl crate::Readable for BROWN_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brown_out::W`](W) writer structure"]
impl crate::Writable for BROWN_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BROWN_OUT to value 0x03ff_2ff1"]
impl crate::Resettable for BROWN_OUT_SPEC {
    const RESET_VALUE: u32 = 0x03ff_2ff1;
}
