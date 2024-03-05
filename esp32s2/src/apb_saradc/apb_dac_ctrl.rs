#[doc = "Register `APB_DAC_CTRL` reader"]
pub type R = crate::R<APB_DAC_CTRL_SPEC>;
#[doc = "Register `APB_DAC_CTRL` writer"]
pub type W = crate::W<APB_DAC_CTRL_SPEC>;
#[doc = "Field `DAC_TIMER_TARGET` reader - Set DAC timer target."]
pub type DAC_TIMER_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `DAC_TIMER_TARGET` writer - Set DAC timer target."]
pub type DAC_TIMER_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DAC_TIMER_EN` reader - Enable read dac data."]
pub type DAC_TIMER_EN_R = crate::BitReader;
#[doc = "Field `DAC_TIMER_EN` writer - Enable read dac data."]
pub type DAC_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_DAC_ALTER_MODE` reader - Enable DAC alter mode."]
pub type APB_DAC_ALTER_MODE_R = crate::BitReader;
#[doc = "Field `APB_DAC_ALTER_MODE` writer - Enable DAC alter mode."]
pub type APB_DAC_ALTER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_DAC_TRANS` reader - Enable DMA_DAC."]
pub type APB_DAC_TRANS_R = crate::BitReader;
#[doc = "Field `APB_DAC_TRANS` writer - Enable DMA_DAC."]
pub type APB_DAC_TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_RESET_FIFO` reader - Reset DIG DAC FIFO."]
pub type DAC_RESET_FIFO_R = crate::BitReader;
#[doc = "Field `DAC_RESET_FIFO` writer - Reset DIG DAC FIFO."]
pub type DAC_RESET_FIFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_DAC_RST` reader - Reset DIG DAC by software."]
pub type APB_DAC_RST_R = crate::BitReader;
#[doc = "Field `APB_DAC_RST` writer - Reset DIG DAC by software."]
pub type APB_DAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Set DAC timer target."]
    #[inline(always)]
    pub fn dac_timer_target(&self) -> DAC_TIMER_TARGET_R {
        DAC_TIMER_TARGET_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Enable read dac data."]
    #[inline(always)]
    pub fn dac_timer_en(&self) -> DAC_TIMER_EN_R {
        DAC_TIMER_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable DAC alter mode."]
    #[inline(always)]
    pub fn apb_dac_alter_mode(&self) -> APB_DAC_ALTER_MODE_R {
        APB_DAC_ALTER_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable DMA_DAC."]
    #[inline(always)]
    pub fn apb_dac_trans(&self) -> APB_DAC_TRANS_R {
        APB_DAC_TRANS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset DIG DAC FIFO."]
    #[inline(always)]
    pub fn dac_reset_fifo(&self) -> DAC_RESET_FIFO_R {
        DAC_RESET_FIFO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset DIG DAC by software."]
    #[inline(always)]
    pub fn apb_dac_rst(&self) -> APB_DAC_RST_R {
        APB_DAC_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_DAC_CTRL")
            .field(
                "dac_timer_target",
                &format_args!("{}", self.dac_timer_target().bits()),
            )
            .field(
                "dac_timer_en",
                &format_args!("{}", self.dac_timer_en().bit()),
            )
            .field(
                "apb_dac_alter_mode",
                &format_args!("{}", self.apb_dac_alter_mode().bit()),
            )
            .field(
                "apb_dac_trans",
                &format_args!("{}", self.apb_dac_trans().bit()),
            )
            .field(
                "dac_reset_fifo",
                &format_args!("{}", self.dac_reset_fifo().bit()),
            )
            .field("apb_dac_rst", &format_args!("{}", self.apb_dac_rst().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_DAC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Set DAC timer target."]
    #[inline(always)]
    #[must_use]
    pub fn dac_timer_target(&mut self) -> DAC_TIMER_TARGET_W<APB_DAC_CTRL_SPEC> {
        DAC_TIMER_TARGET_W::new(self, 0)
    }
    #[doc = "Bit 12 - Enable read dac data."]
    #[inline(always)]
    #[must_use]
    pub fn dac_timer_en(&mut self) -> DAC_TIMER_EN_W<APB_DAC_CTRL_SPEC> {
        DAC_TIMER_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable DAC alter mode."]
    #[inline(always)]
    #[must_use]
    pub fn apb_dac_alter_mode(&mut self) -> APB_DAC_ALTER_MODE_W<APB_DAC_CTRL_SPEC> {
        APB_DAC_ALTER_MODE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable DMA_DAC."]
    #[inline(always)]
    #[must_use]
    pub fn apb_dac_trans(&mut self) -> APB_DAC_TRANS_W<APB_DAC_CTRL_SPEC> {
        APB_DAC_TRANS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Reset DIG DAC FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn dac_reset_fifo(&mut self) -> DAC_RESET_FIFO_W<APB_DAC_CTRL_SPEC> {
        DAC_RESET_FIFO_W::new(self, 15)
    }
    #[doc = "Bit 16 - Reset DIG DAC by software."]
    #[inline(always)]
    #[must_use]
    pub fn apb_dac_rst(&mut self) -> APB_DAC_RST_W<APB_DAC_CTRL_SPEC> {
        APB_DAC_RST_W::new(self, 16)
    }
}
#[doc = "Configure DAC settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_dac_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_dac_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_DAC_CTRL_SPEC;
impl crate::RegisterSpec for APB_DAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_dac_ctrl::R`](R) reader structure"]
impl crate::Readable for APB_DAC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_dac_ctrl::W`](W) writer structure"]
impl crate::Writable for APB_DAC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_DAC_CTRL to value 0x2064"]
impl crate::Resettable for APB_DAC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2064;
}
