#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Field `OUT_SEL` reader - Selection control for GPIO output n. If a value s (0&lt;=s&lt;256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
pub type OUT_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `OUT_SEL` writer - Selection control for GPIO output n. If a value s (0&lt;=s&lt;256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
pub type OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `INV_SEL` reader - 0: Do not invert the output value; 1: Invert the output value."]
pub type INV_SEL_R = crate::BitReader;
#[doc = "Field `INV_SEL` writer - 0: Do not invert the output value; 1: Invert the output value."]
pub type INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_SEL` reader - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
pub type OEN_SEL_R = crate::BitReader;
#[doc = "Field `OEN_SEL` writer - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
pub type OEN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN_INV_SEL` reader - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
pub type OEN_INV_SEL_R = crate::BitReader;
#[doc = "Field `OEN_INV_SEL` writer - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
pub type OEN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Selection control for GPIO output n. If a value s (0&lt;=s&lt;256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - 0: Do not invert the output value; 1: Invert the output value."]
    #[inline(always)]
    pub fn inv_sel(&self) -> INV_SEL_R {
        INV_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_OUT_SEL_CFG")
            .field("out_sel", &self.out_sel())
            .field("inv_sel", &self.inv_sel())
            .field("oen_sel", &self.oen_sel())
            .field("oen_inv_sel", &self.oen_inv_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Selection control for GPIO output n. If a value s (0&lt;=s&lt;256) is written to this field, the peripheral output signal s will be connected to GPIO output n. If a value 256 is written to this field, bit n of GPIO_OUT_REG/GPIO_OUT1_REG and GPIO_ENABLE_REG/GPIO_ENABLE1_REG will be selected as the output value and output enable."]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - 0: Do not invert the output value; 1: Invert the output value."]
    #[inline(always)]
    #[must_use]
    pub fn inv_sel(&mut self) -> INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        INV_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - 0: Use output enable signal from peripheral; 1: Force the output enable signal to be sourced from bit n of GPIO_ENABLE_REG."]
    #[inline(always)]
    #[must_use]
    pub fn oen_sel(&mut self) -> OEN_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OEN_SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - 0: Do not invert the output enable signal; 1: Invert the output enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W<FUNC_OUT_SEL_CFG_SPEC> {
        OEN_INV_SEL_W::new(self, 11)
    }
}
#[doc = "Peripheral output selection for GPIO %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
