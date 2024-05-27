///Register `PAD_DAC1` reader
pub type R = crate::R<PAD_DAC1_SPEC>;
///Register `PAD_DAC1` writer
pub type W = crate::W<PAD_DAC1_SPEC>;
///Field `PDAC1_DAC_XPD_FORCE` reader - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
pub type PDAC1_DAC_XPD_FORCE_R = crate::BitReader;
///Field `PDAC1_DAC_XPD_FORCE` writer - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
pub type PDAC1_DAC_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_FUN_IE` reader - the input enable of the pad
pub type PDAC1_FUN_IE_R = crate::BitReader;
///Field `PDAC1_FUN_IE` writer - the input enable of the pad
pub type PDAC1_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_SLP_OE` reader - the output enable of the pad in sleep status
pub type PDAC1_SLP_OE_R = crate::BitReader;
///Field `PDAC1_SLP_OE` writer - the output enable of the pad in sleep status
pub type PDAC1_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_SLP_IE` reader - the input enable of the pad in sleep status
pub type PDAC1_SLP_IE_R = crate::BitReader;
///Field `PDAC1_SLP_IE` writer - the input enable of the pad in sleep status
pub type PDAC1_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_SLP_SEL` reader - the sleep status selection signal of the pad
pub type PDAC1_SLP_SEL_R = crate::BitReader;
///Field `PDAC1_SLP_SEL` writer - the sleep status selection signal of the pad
pub type PDAC1_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_FUN_SEL` reader - the functional selection signal of the pad
pub type PDAC1_FUN_SEL_R = crate::FieldReader;
///Field `PDAC1_FUN_SEL` writer - the functional selection signal of the pad
pub type PDAC1_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PDAC1_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function
pub type PDAC1_MUX_SEL_R = crate::BitReader;
///Field `PDAC1_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function
pub type PDAC1_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_XPD_DAC` reader - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
pub type PDAC1_XPD_DAC_R = crate::BitReader;
///Field `PDAC1_XPD_DAC` writer - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
pub type PDAC1_XPD_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_DAC` reader - PAD DAC1 control code.
pub type PDAC1_DAC_R = crate::FieldReader;
///Field `PDAC1_DAC` writer - PAD DAC1 control code.
pub type PDAC1_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PDAC1_RUE` reader - the pull up enable of the pad
pub type PDAC1_RUE_R = crate::BitReader;
///Field `PDAC1_RUE` writer - the pull up enable of the pad
pub type PDAC1_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_RDE` reader - the pull down enable of the pad
pub type PDAC1_RDE_R = crate::BitReader;
///Field `PDAC1_RDE` writer - the pull down enable of the pad
pub type PDAC1_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó
pub type PDAC1_HOLD_R = crate::BitReader;
///Field `PDAC1_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó
pub type PDAC1_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDAC1_DRV` reader - the driver strength of the pad
pub type PDAC1_DRV_R = crate::FieldReader;
///Field `PDAC1_DRV` writer - the driver strength of the pad
pub type PDAC1_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
    #[inline(always)]
    pub fn pdac1_dac_xpd_force(&self) -> PDAC1_DAC_XPD_FORCE_R {
        PDAC1_DAC_XPD_FORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - the input enable of the pad
    #[inline(always)]
    pub fn pdac1_fun_ie(&self) -> PDAC1_FUN_IE_R {
        PDAC1_FUN_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - the output enable of the pad in sleep status
    #[inline(always)]
    pub fn pdac1_slp_oe(&self) -> PDAC1_SLP_OE_R {
        PDAC1_SLP_OE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - the input enable of the pad in sleep status
    #[inline(always)]
    pub fn pdac1_slp_ie(&self) -> PDAC1_SLP_IE_R {
        PDAC1_SLP_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - the sleep status selection signal of the pad
    #[inline(always)]
    pub fn pdac1_slp_sel(&self) -> PDAC1_SLP_SEL_R {
        PDAC1_SLP_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:16 - the functional selection signal of the pad
    #[inline(always)]
    pub fn pdac1_fun_sel(&self) -> PDAC1_FUN_SEL_R {
        PDAC1_FUN_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function
    #[inline(always)]
    pub fn pdac1_mux_sel(&self) -> PDAC1_MUX_SEL_R {
        PDAC1_MUX_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
    #[inline(always)]
    pub fn pdac1_xpd_dac(&self) -> PDAC1_XPD_DAC_R {
        PDAC1_XPD_DAC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:26 - PAD DAC1 control code.
    #[inline(always)]
    pub fn pdac1_dac(&self) -> PDAC1_DAC_R {
        PDAC1_DAC_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    ///Bit 27 - the pull up enable of the pad
    #[inline(always)]
    pub fn pdac1_rue(&self) -> PDAC1_RUE_R {
        PDAC1_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - the pull down enable of the pad
    #[inline(always)]
    pub fn pdac1_rde(&self) -> PDAC1_RDE_R {
        PDAC1_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - hold the current value of the output when setting the hold to Ò1Ó
    #[inline(always)]
    pub fn pdac1_hold(&self) -> PDAC1_HOLD_R {
        PDAC1_HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - the driver strength of the pad
    #[inline(always)]
    pub fn pdac1_drv(&self) -> PDAC1_DRV_R {
        PDAC1_DRV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC1")
            .field("pdac1_dac_xpd_force", &self.pdac1_dac_xpd_force())
            .field("pdac1_fun_ie", &self.pdac1_fun_ie())
            .field("pdac1_slp_oe", &self.pdac1_slp_oe())
            .field("pdac1_slp_ie", &self.pdac1_slp_ie())
            .field("pdac1_slp_sel", &self.pdac1_slp_sel())
            .field("pdac1_fun_sel", &self.pdac1_fun_sel())
            .field("pdac1_mux_sel", &self.pdac1_mux_sel())
            .field("pdac1_xpd_dac", &self.pdac1_xpd_dac())
            .field("pdac1_dac", &self.pdac1_dac())
            .field("pdac1_rue", &self.pdac1_rue())
            .field("pdac1_rde", &self.pdac1_rde())
            .field("pdac1_hold", &self.pdac1_hold())
            .field("pdac1_drv", &self.pdac1_drv())
            .finish()
    }
}
impl W {
    ///Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
    #[inline(always)]
    #[must_use]
    pub fn pdac1_dac_xpd_force(&mut self) -> PDAC1_DAC_XPD_FORCE_W<PAD_DAC1_SPEC> {
        PDAC1_DAC_XPD_FORCE_W::new(self, 10)
    }
    ///Bit 11 - the input enable of the pad
    #[inline(always)]
    #[must_use]
    pub fn pdac1_fun_ie(&mut self) -> PDAC1_FUN_IE_W<PAD_DAC1_SPEC> {
        PDAC1_FUN_IE_W::new(self, 11)
    }
    ///Bit 12 - the output enable of the pad in sleep status
    #[inline(always)]
    #[must_use]
    pub fn pdac1_slp_oe(&mut self) -> PDAC1_SLP_OE_W<PAD_DAC1_SPEC> {
        PDAC1_SLP_OE_W::new(self, 12)
    }
    ///Bit 13 - the input enable of the pad in sleep status
    #[inline(always)]
    #[must_use]
    pub fn pdac1_slp_ie(&mut self) -> PDAC1_SLP_IE_W<PAD_DAC1_SPEC> {
        PDAC1_SLP_IE_W::new(self, 13)
    }
    ///Bit 14 - the sleep status selection signal of the pad
    #[inline(always)]
    #[must_use]
    pub fn pdac1_slp_sel(&mut self) -> PDAC1_SLP_SEL_W<PAD_DAC1_SPEC> {
        PDAC1_SLP_SEL_W::new(self, 14)
    }
    ///Bits 15:16 - the functional selection signal of the pad
    #[inline(always)]
    #[must_use]
    pub fn pdac1_fun_sel(&mut self) -> PDAC1_FUN_SEL_W<PAD_DAC1_SPEC> {
        PDAC1_FUN_SEL_W::new(self, 15)
    }
    ///Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function
    #[inline(always)]
    #[must_use]
    pub fn pdac1_mux_sel(&mut self) -> PDAC1_MUX_SEL_W<PAD_DAC1_SPEC> {
        PDAC1_MUX_SEL_W::new(self, 17)
    }
    ///Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0
    #[inline(always)]
    #[must_use]
    pub fn pdac1_xpd_dac(&mut self) -> PDAC1_XPD_DAC_W<PAD_DAC1_SPEC> {
        PDAC1_XPD_DAC_W::new(self, 18)
    }
    ///Bits 19:26 - PAD DAC1 control code.
    #[inline(always)]
    #[must_use]
    pub fn pdac1_dac(&mut self) -> PDAC1_DAC_W<PAD_DAC1_SPEC> {
        PDAC1_DAC_W::new(self, 19)
    }
    ///Bit 27 - the pull up enable of the pad
    #[inline(always)]
    #[must_use]
    pub fn pdac1_rue(&mut self) -> PDAC1_RUE_W<PAD_DAC1_SPEC> {
        PDAC1_RUE_W::new(self, 27)
    }
    ///Bit 28 - the pull down enable of the pad
    #[inline(always)]
    #[must_use]
    pub fn pdac1_rde(&mut self) -> PDAC1_RDE_W<PAD_DAC1_SPEC> {
        PDAC1_RDE_W::new(self, 28)
    }
    ///Bit 29 - hold the current value of the output when setting the hold to Ò1Ó
    #[inline(always)]
    #[must_use]
    pub fn pdac1_hold(&mut self) -> PDAC1_HOLD_W<PAD_DAC1_SPEC> {
        PDAC1_HOLD_W::new(self, 29)
    }
    ///Bits 30:31 - the driver strength of the pad
    #[inline(always)]
    #[must_use]
    pub fn pdac1_drv(&mut self) -> PDAC1_DRV_W<PAD_DAC1_SPEC> {
        PDAC1_DRV_W::new(self, 30)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pad_dac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PAD_DAC1_SPEC;
impl crate::RegisterSpec for PAD_DAC1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pad_dac1::R`](R) reader structure
impl crate::Readable for PAD_DAC1_SPEC {}
///`write(|w| ..)` method takes [`pad_dac1::W`](W) writer structure
impl crate::Writable for PAD_DAC1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAD_DAC1 to value 0x8000_0000
impl crate::Resettable for PAD_DAC1_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
