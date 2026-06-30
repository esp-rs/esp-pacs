#[doc = "Register `FUNC63_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC63_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC63_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC63_OUT_SEL_CFG_SPEC>;
#[doc = "Field `FUNC63_OUT_SEL` reader - Configures to select a signal 0 (0 <= 0 < 256) from 256 peripheral signals to be output from GPIO0.\\\\ 0: Select signal 0\\\\ 1: Select signal 1\\\\ ......\\\\ 254: Select signal 254\\\\ 255: Select signal 255\\\\ Or\\\\ 256: Bit 0 of GPIO_OUT_REG and GPIO_ENABLE_REG are selected as the output value and output enable. For the detailed signal list, see Table <a href=tab:iomuxgpio-periph-signals-via-gpio-matrix\">link</a>. \""]
pub type FUNC63_OUT_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `FUNC63_OUT_SEL` writer - Configures to select a signal 0 (0 <= 0 < 256) from 256 peripheral signals to be output from GPIO0.\\\\ 0: Select signal 0\\\\ 1: Select signal 1\\\\ ......\\\\ 254: Select signal 254\\\\ 255: Select signal 255\\\\ Or\\\\ 256: Bit 0 of GPIO_OUT_REG and GPIO_ENABLE_REG are selected as the output value and output enable. For the detailed signal list, see Table <a href=tab:iomuxgpio-periph-signals-via-gpio-matrix\">link</a>. \""]
pub type FUNC63_OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FUNC63_OUT_INV_SEL` reader - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC63_OUT_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC63_OUT_INV_SEL` writer - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC63_OUT_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNC63_OE_SEL` reader - Configures to select the source of output enable signal.\\\\ 0: Use output enable signal from peripheral.\\\\ 1: Force the output enable signal to be sourced from bit 10 of GPIO_ENABLE_REG. \\\\"]
pub type FUNC63_OE_SEL_R = crate::BitReader;
#[doc = "Field `FUNC63_OE_SEL` writer - Configures to select the source of output enable signal.\\\\ 0: Use output enable signal from peripheral.\\\\ 1: Force the output enable signal to be sourced from bit 10 of GPIO_ENABLE_REG. \\\\"]
pub type FUNC63_OE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNC63_OE_INV_SEL` reader - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC63_OE_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC63_OE_INV_SEL` writer - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC63_OE_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Configures to select a signal 0 (0 <= 0 < 256) from 256 peripheral signals to be output from GPIO0.\\\\ 0: Select signal 0\\\\ 1: Select signal 1\\\\ ......\\\\ 254: Select signal 254\\\\ 255: Select signal 255\\\\ Or\\\\ 256: Bit 0 of GPIO_OUT_REG and GPIO_ENABLE_REG are selected as the output value and output enable. For the detailed signal list, see Table <a href=tab:iomuxgpio-periph-signals-via-gpio-matrix\">link</a>. \""]
    #[inline(always)]
    pub fn func63_out_sel(&self) -> FUNC63_OUT_SEL_R {
        FUNC63_OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func63_out_inv_sel(&self) -> FUNC63_OUT_INV_SEL_R {
        FUNC63_OUT_INV_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures to select the source of output enable signal.\\\\ 0: Use output enable signal from peripheral.\\\\ 1: Force the output enable signal to be sourced from bit 10 of GPIO_ENABLE_REG. \\\\"]
    #[inline(always)]
    pub fn func63_oe_sel(&self) -> FUNC63_OE_SEL_R {
        FUNC63_OE_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func63_oe_inv_sel(&self) -> FUNC63_OE_INV_SEL_R {
        FUNC63_OE_INV_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC63_OUT_SEL_CFG")
            .field("func63_out_sel", &self.func63_out_sel())
            .field("func63_out_inv_sel", &self.func63_out_inv_sel())
            .field("func63_oe_sel", &self.func63_oe_sel())
            .field("func63_oe_inv_sel", &self.func63_oe_inv_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Configures to select a signal 0 (0 <= 0 < 256) from 256 peripheral signals to be output from GPIO0.\\\\ 0: Select signal 0\\\\ 1: Select signal 1\\\\ ......\\\\ 254: Select signal 254\\\\ 255: Select signal 255\\\\ Or\\\\ 256: Bit 0 of GPIO_OUT_REG and GPIO_ENABLE_REG are selected as the output value and output enable. For the detailed signal list, see Table <a href=tab:iomuxgpio-periph-signals-via-gpio-matrix\">link</a>. \""]
    #[inline(always)]
    pub fn func63_out_sel(&mut self) -> FUNC63_OUT_SEL_W<'_, FUNC63_OUT_SEL_CFG_SPEC> {
        FUNC63_OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - Configures whether or not to invert the output value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func63_out_inv_sel(&mut self) -> FUNC63_OUT_INV_SEL_W<'_, FUNC63_OUT_SEL_CFG_SPEC> {
        FUNC63_OUT_INV_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures to select the source of output enable signal.\\\\ 0: Use output enable signal from peripheral.\\\\ 1: Force the output enable signal to be sourced from bit 10 of GPIO_ENABLE_REG. \\\\"]
    #[inline(always)]
    pub fn func63_oe_sel(&mut self) -> FUNC63_OE_SEL_W<'_, FUNC63_OUT_SEL_CFG_SPEC> {
        FUNC63_OE_SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to invert the output enable signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func63_oe_inv_sel(&mut self) -> FUNC63_OE_INV_SEL_W<'_, FUNC63_OUT_SEL_CFG_SPEC> {
        FUNC63_OE_INV_SEL_W::new(self, 11)
    }
}
#[doc = "Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func63_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func63_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC63_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC63_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func63_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC63_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func63_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC63_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC63_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for FUNC63_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
