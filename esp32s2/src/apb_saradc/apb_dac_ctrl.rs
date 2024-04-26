#[doc = "Register `APB_DAC_CTRL` reader"]
pub type R = crate::R<APB_DAC_CTRL_SPEC>;
#[doc = "Register `APB_DAC_CTRL` writer"]
pub type W = crate::W<APB_DAC_CTRL_SPEC>;
#[doc = "Field `TIMER_TARGET` reader - Set DAC timer target."]
pub type TIMER_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TARGET` writer - Set DAC timer target."]
pub type TIMER_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TIMER_EN` reader - Enable read dac data."]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - Enable read dac data."]
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTER_MODE` reader - Enable DAC alter mode."]
pub type ALTER_MODE_R = crate::BitReader;
#[doc = "Field `ALTER_MODE` writer - Enable DAC alter mode."]
pub type ALTER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS` reader - Enable DMA_DAC."]
pub type TRANS_R = crate::BitReader;
#[doc = "Field `TRANS` writer - Enable DMA_DAC."]
pub type TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_FIFO` reader - Reset DIG DAC FIFO."]
pub type RESET_FIFO_R = crate::BitReader;
#[doc = "Field `RESET_FIFO` writer - Reset DIG DAC FIFO."]
pub type RESET_FIFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Reset DIG DAC by software."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - Reset DIG DAC by software."]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Set DAC timer target."]
    #[inline(always)]
    pub fn timer_target(&self) -> TIMER_TARGET_R {
        TIMER_TARGET_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Enable read dac data."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable DAC alter mode."]
    #[inline(always)]
    pub fn alter_mode(&self) -> ALTER_MODE_R {
        ALTER_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable DMA_DAC."]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset DIG DAC FIFO."]
    #[inline(always)]
    pub fn reset_fifo(&self) -> RESET_FIFO_R {
        RESET_FIFO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset DIG DAC by software."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_DAC_CTRL")
            .field(
                "timer_target",
                &format_args!("{}", self.timer_target().bits()),
            )
            .field("timer_en", &format_args!("{}", self.timer_en().bit()))
            .field("alter_mode", &format_args!("{}", self.alter_mode().bit()))
            .field("trans", &format_args!("{}", self.trans().bit()))
            .field("reset_fifo", &format_args!("{}", self.reset_fifo().bit()))
            .field("rst", &format_args!("{}", self.rst().bit()))
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
    pub fn timer_target(&mut self) -> TIMER_TARGET_W<APB_DAC_CTRL_SPEC> {
        TIMER_TARGET_W::new(self, 0)
    }
    #[doc = "Bit 12 - Enable read dac data."]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TIMER_EN_W<APB_DAC_CTRL_SPEC> {
        TIMER_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable DAC alter mode."]
    #[inline(always)]
    #[must_use]
    pub fn alter_mode(&mut self) -> ALTER_MODE_W<APB_DAC_CTRL_SPEC> {
        ALTER_MODE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable DMA_DAC."]
    #[inline(always)]
    #[must_use]
    pub fn trans(&mut self) -> TRANS_W<APB_DAC_CTRL_SPEC> {
        TRANS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Reset DIG DAC FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn reset_fifo(&mut self) -> RESET_FIFO_W<APB_DAC_CTRL_SPEC> {
        RESET_FIFO_W::new(self, 15)
    }
    #[doc = "Bit 16 - Reset DIG DAC by software."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<APB_DAC_CTRL_SPEC> {
        RST_W::new(self, 16)
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
