#[doc = "Register `SPI_DIN_NUM` reader"]
pub type R = crate::R<SPI_DIN_NUM_SPEC>;
#[doc = "Register `SPI_DIN_NUM` writer"]
pub type W = crate::W<SPI_DIN_NUM_SPEC>;
#[doc = "Field `SPI_DIN0_NUM` reader - Configures the delays to input signal FSPID based on the setting of SPI_DIN0_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN0_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN0_NUM` writer - Configures the delays to input signal FSPID based on the setting of SPI_DIN0_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN1_NUM` reader - Configures the delays to input signal FSPIQ based on the setting of SPI_DIN1_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN1_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN1_NUM` writer - Configures the delays to input signal FSPIQ based on the setting of SPI_DIN1_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN2_NUM` reader - Configures the delays to input signal FSPIWP based on the setting of SPI_DIN2_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN2_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN2_NUM` writer - Configures the delays to input signal FSPIWP based on the setting of SPI_DIN2_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN3_NUM` reader - Configures the delays to input signal FSPIHD based on the setting of SPI_DIN3_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN3_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN3_NUM` writer - Configures the delays to input signal FSPIHD based on the setting of SPI_DIN3_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
pub type SPI_DIN3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN4_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN4_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN5_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN5_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN6_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN6_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN7_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN7_NUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Configures the delays to input signal FSPID based on the setting of SPI_DIN0_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din0_num(&self) -> SPI_DIN0_NUM_R {
        SPI_DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configures the delays to input signal FSPIQ based on the setting of SPI_DIN1_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din1_num(&self) -> SPI_DIN1_NUM_R {
        SPI_DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configures the delays to input signal FSPIWP based on the setting of SPI_DIN2_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din2_num(&self) -> SPI_DIN2_NUM_R {
        SPI_DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configures the delays to input signal FSPIHD based on the setting of SPI_DIN3_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din3_num(&self) -> SPI_DIN3_NUM_R {
        SPI_DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din4_num(&self) -> SPI_DIN4_NUM_R {
        SPI_DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din5_num(&self) -> SPI_DIN5_NUM_R {
        SPI_DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din6_num(&self) -> SPI_DIN6_NUM_R {
        SPI_DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din7_num(&self) -> SPI_DIN7_NUM_R {
        SPI_DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DIN_NUM")
            .field("spi_din0_num", &self.spi_din0_num())
            .field("spi_din1_num", &self.spi_din1_num())
            .field("spi_din2_num", &self.spi_din2_num())
            .field("spi_din3_num", &self.spi_din3_num())
            .field("spi_din4_num", &self.spi_din4_num())
            .field("spi_din5_num", &self.spi_din5_num())
            .field("spi_din6_num", &self.spi_din6_num())
            .field("spi_din7_num", &self.spi_din7_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures the delays to input signal FSPID based on the setting of SPI_DIN0_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din0_num(&mut self) -> SPI_DIN0_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configures the delays to input signal FSPIQ based on the setting of SPI_DIN1_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din1_num(&mut self) -> SPI_DIN1_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configures the delays to input signal FSPIWP based on the setting of SPI_DIN2_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din2_num(&mut self) -> SPI_DIN2_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configures the delays to input signal FSPIHD based on the setting of SPI_DIN3_MODE.\\\\ 0: Delayed by 1 clock cycle\\\\ 1: Delayed by 2 clock cycles\\\\ 2: Delayed by 3 clock cycles\\\\ 3: Delayed by 4 clock cycles\\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din3_num(&mut self) -> SPI_DIN3_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN3_NUM_W::new(self, 6)
    }
}
#[doc = "SPI input delay number configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_din_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_din_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DIN_NUM_SPEC;
impl crate::RegisterSpec for SPI_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_din_num::R`](R) reader structure"]
impl crate::Readable for SPI_DIN_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_din_num::W`](W) writer structure"]
impl crate::Writable for SPI_DIN_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DIN_NUM to value 0"]
impl crate::Resettable for SPI_DIN_NUM_SPEC {}
