#[doc = "Register `RX_MODE_CFG` reader"]
pub type R = crate::R<RX_MODE_CFG_SPEC>;
#[doc = "Register `RX_MODE_CFG` writer"]
pub type W = crate::W<RX_MODE_CFG_SPEC>;
#[doc = "Field `RX_EXT_EN_SEL` reader - Configures rx external enable signal selection from IO PAD."]
pub type RX_EXT_EN_SEL_R = crate::FieldReader;
#[doc = "Field `RX_EXT_EN_SEL` writer - Configures rx external enable signal selection from IO PAD."]
pub type RX_EXT_EN_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RX_SW_EN` reader - Set this bit to enable data sampling by software."]
pub type RX_SW_EN_R = crate::BitReader;
#[doc = "Field `RX_SW_EN` writer - Set this bit to enable data sampling by software."]
pub type RX_SW_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_EXT_EN_INV` reader - Set this bit to invert the external enable signal."]
pub type RX_EXT_EN_INV_R = crate::BitReader;
#[doc = "Field `RX_EXT_EN_INV` writer - Set this bit to invert the external enable signal."]
pub type RX_EXT_EN_INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` reader - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
pub type RX_PULSE_SUBMODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` writer - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
pub type RX_PULSE_SUBMODE_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RX_SMP_MODE_SEL` reader - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
pub type RX_SMP_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_SMP_MODE_SEL` writer - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
pub type RX_SMP_MODE_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 21:24 - Configures rx external enable signal selection from IO PAD."]
    #[inline(always)]
    pub fn rx_ext_en_sel(&self) -> RX_EXT_EN_SEL_R {
        RX_EXT_EN_SEL_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Set this bit to enable data sampling by software."]
    #[inline(always)]
    pub fn rx_sw_en(&self) -> RX_SW_EN_R {
        RX_SW_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to invert the external enable signal."]
    #[inline(always)]
    pub fn rx_ext_en_inv(&self) -> RX_EXT_EN_INV_R {
        RX_EXT_EN_INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
    #[inline(always)]
    pub fn rx_pulse_submode_sel(&self) -> RX_PULSE_SUBMODE_SEL_R {
        RX_PULSE_SUBMODE_SEL_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
    #[inline(always)]
    pub fn rx_smp_mode_sel(&self) -> RX_SMP_MODE_SEL_R {
        RX_SMP_MODE_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_MODE_CFG")
            .field(
                "rx_ext_en_sel",
                &format_args!("{}", self.rx_ext_en_sel().bits()),
            )
            .field("rx_sw_en", &format_args!("{}", self.rx_sw_en().bit()))
            .field(
                "rx_ext_en_inv",
                &format_args!("{}", self.rx_ext_en_inv().bit()),
            )
            .field(
                "rx_pulse_submode_sel",
                &format_args!("{}", self.rx_pulse_submode_sel().bits()),
            )
            .field(
                "rx_smp_mode_sel",
                &format_args!("{}", self.rx_smp_mode_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_MODE_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 21:24 - Configures rx external enable signal selection from IO PAD."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ext_en_sel(&mut self) -> RX_EXT_EN_SEL_W<RX_MODE_CFG_SPEC, 21> {
        RX_EXT_EN_SEL_W::new(self)
    }
    #[doc = "Bit 25 - Set this bit to enable data sampling by software."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sw_en(&mut self) -> RX_SW_EN_W<RX_MODE_CFG_SPEC, 25> {
        RX_SW_EN_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to invert the external enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ext_en_inv(&mut self) -> RX_EXT_EN_INV_W<RX_MODE_CFG_SPEC, 26> {
        RX_EXT_EN_INV_W::new(self)
    }
    #[doc = "Bits 27:29 - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pulse_submode_sel(&mut self) -> RX_PULSE_SUBMODE_SEL_W<RX_MODE_CFG_SPEC, 27> {
        RX_PULSE_SUBMODE_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_smp_mode_sel(&mut self) -> RX_SMP_MODE_SEL_W<RX_MODE_CFG_SPEC, 30> {
        RX_SMP_MODE_SEL_W::new(self)
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
#[doc = "Parallel RX Sampling mode configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_MODE_CFG_SPEC;
impl crate::RegisterSpec for RX_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_mode_cfg::R`](R) reader structure"]
impl crate::Readable for RX_MODE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_mode_cfg::W`](W) writer structure"]
impl crate::Writable for RX_MODE_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_MODE_CFG to value 0x00e0_0000"]
impl crate::Resettable for RX_MODE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x00e0_0000;
}
