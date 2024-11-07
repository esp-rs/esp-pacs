#[doc = "Register `PAD_DAC1` reader"]
pub type R = crate::R<PAD_DAC1_SPEC>;
#[doc = "Register `PAD_DAC1` writer"]
pub type W = crate::W<PAD_DAC1_SPEC>;
#[doc = "Field `PDAC1_DAC` reader - PDAC1_DAC"]
pub type PDAC1_DAC_R = crate::FieldReader;
#[doc = "Field `PDAC1_DAC` writer - PDAC1_DAC"]
pub type PDAC1_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PDAC1_XPD_DAC` reader - PDAC1_XPD_DAC"]
pub type PDAC1_XPD_DAC_R = crate::BitReader;
#[doc = "Field `PDAC1_XPD_DAC` writer - PDAC1_XPD_DAC"]
pub type PDAC1_XPD_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_DAC_XPD_FORCE` reader - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
pub type PDAC1_DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC1_DAC_XPD_FORCE` writer - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
pub type PDAC1_DAC_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_FUN_IE` reader - input enable in work mode"]
pub type PDAC1_FUN_IE_R = crate::BitReader;
#[doc = "Field `PDAC1_FUN_IE` writer - input enable in work mode"]
pub type PDAC1_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_SLP_OE` reader - output enable in sleep mode"]
pub type PDAC1_SLP_OE_R = crate::BitReader;
#[doc = "Field `PDAC1_SLP_OE` writer - output enable in sleep mode"]
pub type PDAC1_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_SLP_IE` reader - input enable in sleep mode"]
pub type PDAC1_SLP_IE_R = crate::BitReader;
#[doc = "Field `PDAC1_SLP_IE` writer - input enable in sleep mode"]
pub type PDAC1_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type PDAC1_SLP_SEL_R = crate::BitReader;
#[doc = "Field `PDAC1_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type PDAC1_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_FUN_SEL` reader - PDAC1 function sel"]
pub type PDAC1_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `PDAC1_FUN_SEL` writer - PDAC1 function sel"]
pub type PDAC1_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PDAC1_MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub type PDAC1_MUX_SEL_R = crate::BitReader;
#[doc = "Field `PDAC1_MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub type PDAC1_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_RUE` reader - PDAC1_RUE"]
pub type PDAC1_RUE_R = crate::BitReader;
#[doc = "Field `PDAC1_RUE` writer - PDAC1_RUE"]
pub type PDAC1_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_RDE` reader - PDAC1_RDE"]
pub type PDAC1_RDE_R = crate::BitReader;
#[doc = "Field `PDAC1_RDE` writer - PDAC1_RDE"]
pub type PDAC1_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_DRV` reader - PDAC1_DRV"]
pub type PDAC1_DRV_R = crate::FieldReader;
#[doc = "Field `PDAC1_DRV` writer - PDAC1_DRV"]
pub type PDAC1_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 3:10 - PDAC1_DAC"]
    #[inline(always)]
    pub fn pdac1_dac(&self) -> PDAC1_DAC_R {
        PDAC1_DAC_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn pdac1_xpd_dac(&self) -> PDAC1_XPD_DAC_R {
        PDAC1_XPD_DAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn pdac1_dac_xpd_force(&self) -> PDAC1_DAC_XPD_FORCE_R {
        PDAC1_DAC_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn pdac1_fun_ie(&self) -> PDAC1_FUN_IE_R {
        PDAC1_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_oe(&self) -> PDAC1_SLP_OE_R {
        PDAC1_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_ie(&self) -> PDAC1_SLP_IE_R {
        PDAC1_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_sel(&self) -> PDAC1_SLP_SEL_R {
        PDAC1_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn pdac1_fun_sel(&self) -> PDAC1_FUN_SEL_R {
        PDAC1_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac1_mux_sel(&self) -> PDAC1_MUX_SEL_R {
        PDAC1_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - PDAC1_RUE"]
    #[inline(always)]
    pub fn pdac1_rue(&self) -> PDAC1_RUE_R {
        PDAC1_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PDAC1_RDE"]
    #[inline(always)]
    pub fn pdac1_rde(&self) -> PDAC1_RDE_R {
        PDAC1_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - PDAC1_DRV"]
    #[inline(always)]
    pub fn pdac1_drv(&self) -> PDAC1_DRV_R {
        PDAC1_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC1")
            .field("pdac1_dac", &self.pdac1_dac())
            .field("pdac1_xpd_dac", &self.pdac1_xpd_dac())
            .field("pdac1_dac_xpd_force", &self.pdac1_dac_xpd_force())
            .field("pdac1_fun_ie", &self.pdac1_fun_ie())
            .field("pdac1_slp_oe", &self.pdac1_slp_oe())
            .field("pdac1_slp_ie", &self.pdac1_slp_ie())
            .field("pdac1_slp_sel", &self.pdac1_slp_sel())
            .field("pdac1_fun_sel", &self.pdac1_fun_sel())
            .field("pdac1_mux_sel", &self.pdac1_mux_sel())
            .field("pdac1_rue", &self.pdac1_rue())
            .field("pdac1_rde", &self.pdac1_rde())
            .field("pdac1_drv", &self.pdac1_drv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:10 - PDAC1_DAC"]
    #[inline(always)]
    pub fn pdac1_dac(&mut self) -> PDAC1_DAC_W<PAD_DAC1_SPEC> {
        PDAC1_DAC_W::new(self, 3)
    }
    #[doc = "Bit 11 - PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn pdac1_xpd_dac(&mut self) -> PDAC1_XPD_DAC_W<PAD_DAC1_SPEC> {
        PDAC1_XPD_DAC_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: use reg_pdac1_xpd_dac to control PDAC1_XPD_DAC,0: use SAR ADC FSM to control PDAC1_XPD_DAC"]
    #[inline(always)]
    pub fn pdac1_dac_xpd_force(&mut self) -> PDAC1_DAC_XPD_FORCE_W<PAD_DAC1_SPEC> {
        PDAC1_DAC_XPD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn pdac1_fun_ie(&mut self) -> PDAC1_FUN_IE_W<PAD_DAC1_SPEC> {
        PDAC1_FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_oe(&mut self) -> PDAC1_SLP_OE_W<PAD_DAC1_SPEC> {
        PDAC1_SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_ie(&mut self) -> PDAC1_SLP_IE_W<PAD_DAC1_SPEC> {
        PDAC1_SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn pdac1_slp_sel(&mut self) -> PDAC1_SLP_SEL_W<PAD_DAC1_SPEC> {
        PDAC1_SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - PDAC1 function sel"]
    #[inline(always)]
    pub fn pdac1_fun_sel(&mut self) -> PDAC1_FUN_SEL_W<PAD_DAC1_SPEC> {
        PDAC1_FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn pdac1_mux_sel(&mut self) -> PDAC1_MUX_SEL_W<PAD_DAC1_SPEC> {
        PDAC1_MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 27 - PDAC1_RUE"]
    #[inline(always)]
    pub fn pdac1_rue(&mut self) -> PDAC1_RUE_W<PAD_DAC1_SPEC> {
        PDAC1_RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - PDAC1_RDE"]
    #[inline(always)]
    pub fn pdac1_rde(&mut self) -> PDAC1_RDE_W<PAD_DAC1_SPEC> {
        PDAC1_RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - PDAC1_DRV"]
    #[inline(always)]
    pub fn pdac1_drv(&mut self) -> PDAC1_DRV_W<PAD_DAC1_SPEC> {
        PDAC1_DRV_W::new(self, 29)
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
