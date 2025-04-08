#[doc = "Register `PAD_DAC1` reader"]
pub type R = crate::R<PAD_DAC1_SPEC>;
#[doc = "Register `PAD_DAC1` writer"]
pub type W = crate::W<PAD_DAC1_SPEC>;
#[doc = "Field `DAC` reader - PDAC1_DAC"]
pub type DAC_R = crate::FieldReader;
#[doc = "Field `DAC` writer - PDAC1_DAC"]
pub type DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XPD_DAC` reader - PDAC1_XPD_DAC"]
pub type XPD_DAC_R = crate::BitReader;
#[doc = "Field `XPD_DAC` writer - PDAC1_XPD_DAC"]
pub type XPD_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_XPD_FORCE` reader - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
pub type DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `DAC_XPD_FORCE` writer - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
pub type DAC_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - input enable in work mode"]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - input enable in work mode"]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_OE` reader - output enable in sleep mode"]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - output enable in sleep mode"]
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_IE` reader - input enable in sleep mode"]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - input enable in sleep mode"]
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_SEL` reader - PDAC1 function sel"]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - PDAC1 function sel"]
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - PDAC1_RUE"]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - PDAC1_RUE"]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDE` reader - PDAC1_RDE"]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - PDAC1_RDE"]
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - PDAC1_DRV"]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - PDAC1_DRV"]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 3:10 - PDAC1_DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn xpd_dac(&self) -> XPD_DAC_R {
        XPD_DAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn dac_xpd_force(&self) -> DAC_XPD_FORCE_R {
        DAC_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - PDAC1_RUE"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PDAC1_RDE"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - PDAC1_DRV"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC1")
            .field("dac", &self.dac())
            .field("xpd_dac", &self.xpd_dac())
            .field("dac_xpd_force", &self.dac_xpd_force())
            .field("fun_ie", &self.fun_ie())
            .field("slp_oe", &self.slp_oe())
            .field("slp_ie", &self.slp_ie())
            .field("slp_sel", &self.slp_sel())
            .field("fun_sel", &self.fun_sel())
            .field("mux_sel", &self.mux_sel())
            .field("rue", &self.rue())
            .field("rde", &self.rde())
            .field("drv", &self.drv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:10 - PDAC1_DAC"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<PAD_DAC1_SPEC> {
        DAC_W::new(self, 3)
    }
    #[doc = "Bit 11 - PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn xpd_dac(&mut self) -> XPD_DAC_W<PAD_DAC1_SPEC> {
        XPD_DAC_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn dac_xpd_force(&mut self) -> DAC_XPD_FORCE_W<PAD_DAC1_SPEC> {
        DAC_XPD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<PAD_DAC1_SPEC> {
        FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W<PAD_DAC1_SPEC> {
        SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W<PAD_DAC1_SPEC> {
        SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<PAD_DAC1_SPEC> {
        SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<PAD_DAC1_SPEC> {
        FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<PAD_DAC1_SPEC> {
        MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 27 - PDAC1_RUE"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W<PAD_DAC1_SPEC> {
        RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - PDAC1_RDE"]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W<PAD_DAC1_SPEC> {
        RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - PDAC1_DRV"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<PAD_DAC1_SPEC> {
        DRV_W::new(self, 29)
    }
}
#[doc = "configure RTC PAD17\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_dac1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_dac1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_DAC1_SPEC;
impl crate::RegisterSpec for PAD_DAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_dac1::R`](R) reader structure"]
impl crate::Readable for PAD_DAC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_dac1::W`](W) writer structure"]
impl crate::Writable for PAD_DAC1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_DAC1 to value 0x4000_0000"]
impl crate::Resettable for PAD_DAC1_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
