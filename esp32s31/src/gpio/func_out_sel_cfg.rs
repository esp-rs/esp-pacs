#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Field `OUT_SEL` reader - Configures to select a signal %s to be outputted from the GPIO."]
pub type OUT_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `OUT_SEL` writer - Configures to select a signal %s to be outputted from the GPIO."]
pub type OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `OUT_INV_SEL` reader - Configures whether or not to invert the output value. 0: Not invert. 1: Invert."]
pub type OUT_INV_SEL_R = crate::BitReader;
#[doc = "Field `OUT_INV_SEL` writer - Configures whether or not to invert the output value. 0: Not invert. 1: Invert."]
pub type OUT_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE_SEL` reader - Configures to select the source of output enable signal. 0: Use output enable signal from peripheral. 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
pub type OE_SEL_R = crate::BitReader;
#[doc = "Field `OE_SEL` writer - Configures to select the source of output enable signal. 0: Use output enable signal from peripheral. 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
pub type OE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE_INV_SEL` reader - Configures whether or not to invert the output enable signal. 0: Not invert. 1: Invert."]
pub type OE_INV_SEL_R = crate::BitReader;
#[doc = "Field `OE_INV_SEL` writer - Configures whether or not to invert the output enable signal. 0: Not invert. 1: Invert."]
pub type OE_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Configures to select a signal %s to be outputted from the GPIO."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Configures whether or not to invert the output value. 0: Not invert. 1: Invert."]
    #[inline(always)]
    pub fn out_inv_sel(&self) -> OUT_INV_SEL_R {
        OUT_INV_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures to select the source of output enable signal. 0: Use output enable signal from peripheral. 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
    #[inline(always)]
    pub fn oe_sel(&self) -> OE_SEL_R {
        OE_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to invert the output enable signal. 0: Not invert. 1: Invert."]
    #[inline(always)]
    pub fn oe_inv_sel(&self) -> OE_INV_SEL_R {
        OE_INV_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("out_sel", &self.out_sel())
            .field("out_inv_sel", &self.out_inv_sel())
            .field("oe_sel", &self.oe_sel())
            .field("oe_inv_sel", &self.oe_inv_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Configures to select a signal %s to be outputted from the GPIO."]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - Configures whether or not to invert the output value. 0: Not invert. 1: Invert."]
    #[inline(always)]
    pub fn out_inv_sel(&mut self) -> OUT_INV_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        OUT_INV_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures to select the source of output enable signal. 0: Use output enable signal from peripheral. 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
    #[inline(always)]
    pub fn oe_sel(&mut self) -> OE_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        OE_SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to invert the output enable signal. 0: Not invert. 1: Invert."]
    #[inline(always)]
    pub fn oe_inv_sel(&mut self) -> OE_INV_SEL_W<'_, FUNC_OUT_SEL_CFG_SPEC> {
        OE_INV_SEL_W::new(self, 11)
    }
}
#[doc = "GPIO output function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {}
