#[doc = "Register `ETM_TASK_P5_CFG` reader"]
pub type R = crate::R<ETM_TASK_P5_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P5_CFG` writer"]
pub type W = crate::W<ETM_TASK_P5_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO25_SEL` reader - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO25_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO25_SEL` writer - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO25_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO25_EN` reader - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO25_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO25_EN` writer - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO25_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO26_SEL` reader - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO26_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO26_SEL` writer - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO26_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO26_EN` reader - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO26_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO26_EN` writer - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO26_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO27_SEL` reader - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO27_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO27_SEL` writer - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO27_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO27_EN` reader - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO27_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO27_EN` writer - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO27_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO28_SEL` reader - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO28_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO28_SEL` writer - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO28_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO28_EN` reader - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO28_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO28_EN` writer - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO28_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio25_sel(&self) -> ETM_TASK_GPIO25_SEL_R {
        ETM_TASK_GPIO25_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio25_en(&self) -> ETM_TASK_GPIO25_EN_R {
        ETM_TASK_GPIO25_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio26_sel(&self) -> ETM_TASK_GPIO26_SEL_R {
        ETM_TASK_GPIO26_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio26_en(&self) -> ETM_TASK_GPIO26_EN_R {
        ETM_TASK_GPIO26_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio27_sel(&self) -> ETM_TASK_GPIO27_SEL_R {
        ETM_TASK_GPIO27_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio27_en(&self) -> ETM_TASK_GPIO27_EN_R {
        ETM_TASK_GPIO27_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio28_sel(&self) -> ETM_TASK_GPIO28_SEL_R {
        ETM_TASK_GPIO28_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio28_en(&self) -> ETM_TASK_GPIO28_EN_R {
        ETM_TASK_GPIO28_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P5_CFG")
            .field("etm_task_gpio25_sel", &self.etm_task_gpio25_sel())
            .field("etm_task_gpio25_en", &self.etm_task_gpio25_en())
            .field("etm_task_gpio26_sel", &self.etm_task_gpio26_sel())
            .field("etm_task_gpio26_en", &self.etm_task_gpio26_en())
            .field("etm_task_gpio27_sel", &self.etm_task_gpio27_sel())
            .field("etm_task_gpio27_en", &self.etm_task_gpio27_en())
            .field("etm_task_gpio28_sel", &self.etm_task_gpio28_sel())
            .field("etm_task_gpio28_en", &self.etm_task_gpio28_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio25_sel(&mut self) -> ETM_TASK_GPIO25_SEL_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO25_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio25_en(&mut self) -> ETM_TASK_GPIO25_EN_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO25_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio26_sel(&mut self) -> ETM_TASK_GPIO26_SEL_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO26_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio26_en(&mut self) -> ETM_TASK_GPIO26_EN_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO26_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio27_sel(&mut self) -> ETM_TASK_GPIO27_SEL_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO27_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio27_en(&mut self) -> ETM_TASK_GPIO27_EN_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO27_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio28_sel(&mut self) -> ETM_TASK_GPIO28_SEL_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO28_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio28_en(&mut self) -> ETM_TASK_GPIO28_EN_W<ETM_TASK_P5_CFG_SPEC> {
        ETM_TASK_GPIO28_EN_W::new(self, 23)
    }
}
#[doc = "GPIO selection register 5 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p5_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p5_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P5_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P5_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p5_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P5_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p5_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P5_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETM_TASK_P5_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P5_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
