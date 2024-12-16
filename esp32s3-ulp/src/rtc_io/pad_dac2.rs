#[doc = "Register `PAD_DAC2` reader"]
pub type R = crate::R<PAD_DAC2_SPEC>;
#[doc = "Register `PAD_DAC2` writer"]
pub type W = crate::W<PAD_DAC2_SPEC>;
#[doc = "Field `PDAC2_DAC` reader - PDAC2_DAC"]
pub type PDAC2_DAC_R = crate::FieldReader;
#[doc = "Field `PDAC2_DAC` writer - PDAC2_DAC"]
pub type PDAC2_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PDAC2_XPD_DAC` reader - PDAC2_XPD_DAC"]
pub type PDAC2_XPD_DAC_R = crate::BitReader;
#[doc = "Field `PDAC2_XPD_DAC` writer - PDAC2_XPD_DAC"]
pub type PDAC2_XPD_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_DAC_XPD_FORCE` reader - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
pub type PDAC2_DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC2_DAC_XPD_FORCE` writer - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
pub type PDAC2_DAC_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_FUN_IE` reader - input enable in work mode"]
pub type PDAC2_FUN_IE_R = crate::BitReader;
#[doc = "Field `PDAC2_FUN_IE` writer - input enable in work mode"]
pub type PDAC2_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_SLP_OE` reader - output enable in sleep mode"]
pub type PDAC2_SLP_OE_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_OE` writer - output enable in sleep mode"]
pub type PDAC2_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_SLP_IE` reader - input enable in sleep mode"]
pub type PDAC2_SLP_IE_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_IE` writer - input enable in sleep mode"]
pub type PDAC2_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type PDAC2_SLP_SEL_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type PDAC2_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_FUN_SEL` reader - PDAC1 function sel"]
pub type PDAC2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `PDAC2_FUN_SEL` writer - PDAC1 function sel"]
pub type PDAC2_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PDAC2_MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub type PDAC2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `PDAC2_MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub type PDAC2_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_RUE` reader - PDAC2_RUE"]
pub type PDAC2_RUE_R = crate::BitReader;
#[doc = "Field `PDAC2_RUE` writer - PDAC2_RUE"]
pub type PDAC2_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_RDE` reader - PDAC2_RDE"]
pub type PDAC2_RDE_R = crate::BitReader;
#[doc = "Field `PDAC2_RDE` writer - PDAC2_RDE"]
pub type PDAC2_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_DRV` reader - PDAC2_DRV"]
pub type PDAC2_DRV_R = crate::FieldReader;
#[doc = "Field `PDAC2_DRV` writer - PDAC2_DRV"]
pub type PDAC2_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 3:10 - PDAC2_DAC"]
    #[inline(always)]
    pub fn pdac2_dac(&self) -> PDAC2_DAC_R {
        PDAC2_DAC_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&self) -> PDAC2_XPD_DAC_R {
        PDAC2_XPD_DAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&self) -> PDAC2_DAC_XPD_FORCE_R {
        PDAC2_DAC_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&self) -> PDAC2_FUN_IE_R {
        PDAC2_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&self) -> PDAC2_SLP_OE_R {
        PDAC2_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&self) -> PDAC2_SLP_IE_R {
        PDAC2_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&self) -> PDAC2_SLP_SEL_R {
        PDAC2_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&self) -> PDAC2_FUN_SEL_R {
        PDAC2_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&self) -> PDAC2_MUX_SEL_R {
        PDAC2_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - PDAC2_RUE"]
    #[inline(always)]
    pub fn pdac2_rue(&self) -> PDAC2_RUE_R {
        PDAC2_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PDAC2_RDE"]
    #[inline(always)]
    pub fn pdac2_rde(&self) -> PDAC2_RDE_R {
        PDAC2_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - PDAC2_DRV"]
    #[inline(always)]
    pub fn pdac2_drv(&self) -> PDAC2_DRV_R {
        PDAC2_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC2")
            .field("pdac2_dac", &self.pdac2_dac())
            .field("pdac2_xpd_dac", &self.pdac2_xpd_dac())
            .field("pdac2_dac_xpd_force", &self.pdac2_dac_xpd_force())
            .field("pdac2_fun_ie", &self.pdac2_fun_ie())
            .field("pdac2_slp_oe", &self.pdac2_slp_oe())
            .field("pdac2_slp_ie", &self.pdac2_slp_ie())
            .field("pdac2_slp_sel", &self.pdac2_slp_sel())
            .field("pdac2_fun_sel", &self.pdac2_fun_sel())
            .field("pdac2_mux_sel", &self.pdac2_mux_sel())
            .field("pdac2_rue", &self.pdac2_rue())
            .field("pdac2_rde", &self.pdac2_rde())
            .field("pdac2_drv", &self.pdac2_drv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:10 - PDAC2_DAC"]
    #[inline(always)]
    pub fn pdac2_dac(&mut self) -> PDAC2_DAC_W<PAD_DAC2_SPEC> {
        PDAC2_DAC_W::new(self, 3)
    }
    #[doc = "Bit 11 - PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&mut self) -> PDAC2_XPD_DAC_W<PAD_DAC2_SPEC> {
        PDAC2_XPD_DAC_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: use reg_pdac2_xpd_dac to control PDAC2_XPD_DAC,0: use SAR ADC FSM to control PDAC2_XPD_DAC"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&mut self) -> PDAC2_DAC_XPD_FORCE_W<PAD_DAC2_SPEC> {
        PDAC2_DAC_XPD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&mut self) -> PDAC2_FUN_IE_W<PAD_DAC2_SPEC> {
        PDAC2_FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&mut self) -> PDAC2_SLP_OE_W<PAD_DAC2_SPEC> {
        PDAC2_SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&mut self) -> PDAC2_SLP_IE_W<PAD_DAC2_SPEC> {
        PDAC2_SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&mut self) -> PDAC2_SLP_SEL_W<PAD_DAC2_SPEC> {
        PDAC2_SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&mut self) -> PDAC2_FUN_SEL_W<PAD_DAC2_SPEC> {
        PDAC2_FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&mut self) -> PDAC2_MUX_SEL_W<PAD_DAC2_SPEC> {
        PDAC2_MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 27 - PDAC2_RUE"]
    #[inline(always)]
    pub fn pdac2_rue(&mut self) -> PDAC2_RUE_W<PAD_DAC2_SPEC> {
        PDAC2_RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - PDAC2_RDE"]
    #[inline(always)]
    pub fn pdac2_rde(&mut self) -> PDAC2_RDE_W<PAD_DAC2_SPEC> {
        PDAC2_RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - PDAC2_DRV"]
    #[inline(always)]
    pub fn pdac2_drv(&mut self) -> PDAC2_DRV_W<PAD_DAC2_SPEC> {
        PDAC2_DRV_W::new(self, 29)
    }
}
#[doc = "configure RTC PAD18\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_dac2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_dac2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_DAC2_SPEC;
impl crate::RegisterSpec for PAD_DAC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_dac2::R`](R) reader structure"]
impl crate::Readable for PAD_DAC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_dac2::W`](W) writer structure"]
impl crate::Writable for PAD_DAC2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_DAC2 to value 0x4000_0000"]
impl crate::Resettable for PAD_DAC2_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
