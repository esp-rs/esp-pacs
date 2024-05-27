///Register `TOUCH_PAD6` reader
pub type R = crate::R<TOUCH_PAD6_SPEC>;
///Register `TOUCH_PAD6` writer
pub type W = crate::W<TOUCH_PAD6_SPEC>;
///Field `TO_GPIO` reader - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS
pub type TO_GPIO_R = crate::BitReader;
///Field `TO_GPIO` writer - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS
pub type TO_GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_IE` reader - the input enable of the pad
pub type FUN_IE_R = crate::BitReader;
///Field `FUN_IE` writer - the input enable of the pad
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_OE` reader - the output enable of the pad in sleep status
pub type SLP_OE_R = crate::BitReader;
///Field `SLP_OE` writer - the output enable of the pad in sleep status
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_IE` reader - the input enable of the pad in sleep status
pub type SLP_IE_R = crate::BitReader;
///Field `SLP_IE` writer - the input enable of the pad in sleep status
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_SEL` reader - the sleep status selection signal of the pad
pub type SLP_SEL_R = crate::BitReader;
///Field `SLP_SEL` writer - the sleep status selection signal of the pad
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_SEL` reader - the functional selection signal of the pad
pub type FUN_SEL_R = crate::FieldReader;
///Field `FUN_SEL` writer - the functional selection signal of the pad
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function
pub type MUX_SEL_R = crate::BitReader;
///Field `MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XPD` reader - touch sensor power on.
pub type XPD_R = crate::BitReader;
///Field `XPD` writer - touch sensor power on.
pub type XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE_OPT` reader - default touch sensor tie option. 0: tie low 1: tie high.
pub type TIE_OPT_R = crate::BitReader;
///Field `TIE_OPT` writer - default touch sensor tie option. 0: tie low 1: tie high.
pub type TIE_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - start touch sensor.
pub type START_R = crate::BitReader;
///Field `START` writer - start touch sensor.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC` reader - touch sensor slope control. 3-bit for each touch panel default 100.
pub type DAC_R = crate::FieldReader;
///Field `DAC` writer - touch sensor slope control. 3-bit for each touch panel default 100.
pub type DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RUE` reader - the pull up enable of the pad
pub type RUE_R = crate::BitReader;
///Field `RUE` writer - the pull up enable of the pad
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDE` reader - the pull down enable of the pad
pub type RDE_R = crate::BitReader;
///Field `RDE` writer - the pull down enable of the pad
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRV` reader - the driver strength of the pad
pub type DRV_R = crate::FieldReader;
///Field `DRV` writer - the driver strength of the pad
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó
pub type HOLD_R = crate::BitReader;
///Field `HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó
pub type HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12 - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS
    #[inline(always)]
    pub fn to_gpio(&self) -> TO_GPIO_R {
        TO_GPIO_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - the input enable of the pad
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - the output enable of the pad in sleep status
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - the input enable of the pad in sleep status
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - the sleep status selection signal of the pad
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - the functional selection signal of the pad
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Ò1Ó select the digital function Ó0Óslection the rtc function
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - touch sensor power on.
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - default touch sensor tie option. 0: tie low 1: tie high.
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - start touch sensor.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100.
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 27 - the pull up enable of the pad
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - the pull down enable of the pad
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:30 - the driver strength of the pad
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - hold the current value of the output when setting the hold to Ò1Ó
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD6")
            .field("to_gpio", &self.to_gpio())
            .field("fun_ie", &self.fun_ie())
            .field("slp_oe", &self.slp_oe())
            .field("slp_ie", &self.slp_ie())
            .field("slp_sel", &self.slp_sel())
            .field("fun_sel", &self.fun_sel())
            .field("mux_sel", &self.mux_sel())
            .field("xpd", &self.xpd())
            .field("tie_opt", &self.tie_opt())
            .field("start", &self.start())
            .field("dac", &self.dac())
            .field("rue", &self.rue())
            .field("rde", &self.rde())
            .field("drv", &self.drv())
            .field("hold", &self.hold())
            .finish()
    }
}
impl W {
    ///Bit 12 - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTMS
    #[inline(always)]
    #[must_use]
    pub fn to_gpio(&mut self) -> TO_GPIO_W<TOUCH_PAD6_SPEC> {
        TO_GPIO_W::new(self, 12)
    }
    ///Bit 13 - the input enable of the pad
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<TOUCH_PAD6_SPEC> {
        FUN_IE_W::new(self, 13)
    }
    ///Bit 14 - the output enable of the pad in sleep status
    #[inline(always)]
    #[must_use]
    pub fn slp_oe(&mut self) -> SLP_OE_W<TOUCH_PAD6_SPEC> {
        SLP_OE_W::new(self, 14)
    }
    ///Bit 15 - the input enable of the pad in sleep status
    #[inline(always)]
    #[must_use]
    pub fn slp_ie(&mut self) -> SLP_IE_W<TOUCH_PAD6_SPEC> {
        SLP_IE_W::new(self, 15)
    }
    ///Bit 16 - the sleep status selection signal of the pad
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<TOUCH_PAD6_SPEC> {
        SLP_SEL_W::new(self, 16)
    }
    ///Bits 17:18 - the functional selection signal of the pad
    #[inline(always)]
    #[must_use]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<TOUCH_PAD6_SPEC> {
        FUN_SEL_W::new(self, 17)
    }
    ///Bit 19 - Ò1Ó select the digital function Ó0Óslection the rtc function
    #[inline(always)]
    #[must_use]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<TOUCH_PAD6_SPEC> {
        MUX_SEL_W::new(self, 19)
    }
    ///Bit 20 - touch sensor power on.
    #[inline(always)]
    #[must_use]
    pub fn xpd(&mut self) -> XPD_W<TOUCH_PAD6_SPEC> {
        XPD_W::new(self, 20)
    }
    ///Bit 21 - default touch sensor tie option. 0: tie low 1: tie high.
    #[inline(always)]
    #[must_use]
    pub fn tie_opt(&mut self) -> TIE_OPT_W<TOUCH_PAD6_SPEC> {
        TIE_OPT_W::new(self, 21)
    }
    ///Bit 22 - start touch sensor.
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<TOUCH_PAD6_SPEC> {
        START_W::new(self, 22)
    }
    ///Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100.
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<TOUCH_PAD6_SPEC> {
        DAC_W::new(self, 23)
    }
    ///Bit 27 - the pull up enable of the pad
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RUE_W<TOUCH_PAD6_SPEC> {
        RUE_W::new(self, 27)
    }
    ///Bit 28 - the pull down enable of the pad
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RDE_W<TOUCH_PAD6_SPEC> {
        RDE_W::new(self, 28)
    }
    ///Bits 29:30 - the driver strength of the pad
    #[inline(always)]
    #[must_use]
    pub fn drv(&mut self) -> DRV_W<TOUCH_PAD6_SPEC> {
        DRV_W::new(self, 29)
    }
    ///Bit 31 - hold the current value of the output when setting the hold to Ò1Ó
    #[inline(always)]
    #[must_use]
    pub fn hold(&mut self) -> HOLD_W<TOUCH_PAD6_SPEC> {
        HOLD_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_PAD6_SPEC;
impl crate::RegisterSpec for TOUCH_PAD6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_pad6::R`](R) reader structure
impl crate::Readable for TOUCH_PAD6_SPEC {}
///`write(|w| ..)` method takes [`touch_pad6::W`](W) writer structure
impl crate::Writable for TOUCH_PAD6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_PAD6 to value 0x4a00_0000
impl crate::Resettable for TOUCH_PAD6_SPEC {
    const RESET_VALUE: u32 = 0x4a00_0000;
}
