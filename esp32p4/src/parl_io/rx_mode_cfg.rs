#[doc = "Register `RX_MODE_CFG` reader"]
pub type R = crate::R<RX_MODE_CFG_SPEC>;
#[doc = "Register `RX_MODE_CFG` writer"]
pub type W = crate::W<RX_MODE_CFG_SPEC>;
#[doc = "Field `RX_EXT_EN_SEL` reader - Configures rx external enable signal selection from IO PAD."]
pub type RX_EXT_EN_SEL_R = crate::FieldReader;
#[doc = "Field `RX_EXT_EN_SEL` writer - Configures rx external enable signal selection from IO PAD."]
pub type RX_EXT_EN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_SW_EN` reader - Write 1 to enable data sampling by software."]
pub type RX_SW_EN_R = crate::BitReader;
#[doc = "Field `RX_SW_EN` writer - Write 1 to enable data sampling by software."]
pub type RX_SW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EXT_EN_INV` reader - Write 1 to invert the external enable signal."]
pub type RX_EXT_EN_INV_R = crate::BitReader;
#[doc = "Field `RX_EXT_EN_INV` writer - Write 1 to invert the external enable signal."]
pub type RX_EXT_EN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` reader - Configures the rxd pulse sampling submode. 0: positive pulse start(data bit included) && positive pulse end(data bit included) 1: positive pulse start(data bit included) && positive pulse end (data bit excluded) 2: positive pulse start(data bit excluded) && positive pulse end (data bit included) 3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded) 4: positive pulse start(data bit included) && length end 5: positive pulse start(data bit excluded) && length end"]
pub type RX_PULSE_SUBMODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` writer - Configures the rxd pulse sampling submode. 0: positive pulse start(data bit included) && positive pulse end(data bit included) 1: positive pulse start(data bit included) && positive pulse end (data bit excluded) 2: positive pulse start(data bit excluded) && positive pulse end (data bit included) 3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded) 4: positive pulse start(data bit included) && length end 5: positive pulse start(data bit excluded) && length end"]
pub type RX_PULSE_SUBMODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_SMP_MODE_SEL` reader - Configures the rxd sampling mode. 0: external level enable mode 1: external pulse enable mode 2: internal software enable mode"]
pub type RX_SMP_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_SMP_MODE_SEL` writer - Configures the rxd sampling mode. 0: external level enable mode 1: external pulse enable mode 2: internal software enable mode"]
pub type RX_SMP_MODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 21:24 - Configures rx external enable signal selection from IO PAD."]
    #[inline(always)]
    pub fn rx_ext_en_sel(&self) -> RX_EXT_EN_SEL_R {
        RX_EXT_EN_SEL_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Write 1 to enable data sampling by software."]
    #[inline(always)]
    pub fn rx_sw_en(&self) -> RX_SW_EN_R {
        RX_SW_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write 1 to invert the external enable signal."]
    #[inline(always)]
    pub fn rx_ext_en_inv(&self) -> RX_EXT_EN_INV_R {
        RX_EXT_EN_INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Configures the rxd pulse sampling submode. 0: positive pulse start(data bit included) && positive pulse end(data bit included) 1: positive pulse start(data bit included) && positive pulse end (data bit excluded) 2: positive pulse start(data bit excluded) && positive pulse end (data bit included) 3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded) 4: positive pulse start(data bit included) && length end 5: positive pulse start(data bit excluded) && length end"]
    #[inline(always)]
    pub fn rx_pulse_submode_sel(&self) -> RX_PULSE_SUBMODE_SEL_R {
        RX_PULSE_SUBMODE_SEL_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Configures the rxd sampling mode. 0: external level enable mode 1: external pulse enable mode 2: internal software enable mode"]
    #[inline(always)]
    pub fn rx_smp_mode_sel(&self) -> RX_SMP_MODE_SEL_R {
        RX_SMP_MODE_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_MODE_CFG")
            .field("rx_ext_en_sel", &self.rx_ext_en_sel())
            .field("rx_sw_en", &self.rx_sw_en())
            .field("rx_ext_en_inv", &self.rx_ext_en_inv())
            .field("rx_pulse_submode_sel", &self.rx_pulse_submode_sel())
            .field("rx_smp_mode_sel", &self.rx_smp_mode_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 21:24 - Configures rx external enable signal selection from IO PAD."]
    #[inline(always)]
    pub fn rx_ext_en_sel(&mut self) -> RX_EXT_EN_SEL_W<'_, RX_MODE_CFG_SPEC> {
        RX_EXT_EN_SEL_W::new(self, 21)
    }
    #[doc = "Bit 25 - Write 1 to enable data sampling by software."]
    #[inline(always)]
    pub fn rx_sw_en(&mut self) -> RX_SW_EN_W<'_, RX_MODE_CFG_SPEC> {
        RX_SW_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Write 1 to invert the external enable signal."]
    #[inline(always)]
    pub fn rx_ext_en_inv(&mut self) -> RX_EXT_EN_INV_W<'_, RX_MODE_CFG_SPEC> {
        RX_EXT_EN_INV_W::new(self, 26)
    }
    #[doc = "Bits 27:29 - Configures the rxd pulse sampling submode. 0: positive pulse start(data bit included) && positive pulse end(data bit included) 1: positive pulse start(data bit included) && positive pulse end (data bit excluded) 2: positive pulse start(data bit excluded) && positive pulse end (data bit included) 3: positive pulse start(data bit excluded) && positive pulse end (data bit excluded) 4: positive pulse start(data bit included) && length end 5: positive pulse start(data bit excluded) && length end"]
    #[inline(always)]
    pub fn rx_pulse_submode_sel(&mut self) -> RX_PULSE_SUBMODE_SEL_W<'_, RX_MODE_CFG_SPEC> {
        RX_PULSE_SUBMODE_SEL_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - Configures the rxd sampling mode. 0: external level enable mode 1: external pulse enable mode 2: internal software enable mode"]
    #[inline(always)]
    pub fn rx_smp_mode_sel(&mut self) -> RX_SMP_MODE_SEL_W<'_, RX_MODE_CFG_SPEC> {
        RX_SMP_MODE_SEL_W::new(self, 30)
    }
}
#[doc = "Parallel RX Sampling mode configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_mode_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_mode_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_MODE_CFG_SPEC;
impl crate::RegisterSpec for RX_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_mode_cfg::R`](R) reader structure"]
impl crate::Readable for RX_MODE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_mode_cfg::W`](W) writer structure"]
impl crate::Writable for RX_MODE_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_MODE_CFG to value 0x00e0_0000"]
impl crate::Resettable for RX_MODE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x00e0_0000;
}
