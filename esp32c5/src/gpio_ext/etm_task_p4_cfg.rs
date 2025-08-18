#[doc = "Register `ETM_TASK_P4_CFG` reader"]
pub type R = crate::R<ETM_TASK_P4_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P4_CFG` writer"]
pub type W = crate::W<ETM_TASK_P4_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO20_SEL` reader - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO20_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO20_SEL` writer - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO20_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO20_EN` reader - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO20_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO20_EN` writer - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO20_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO21_SEL` reader - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO21_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO21_SEL` writer - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO21_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO21_EN` reader - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO21_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO21_EN` writer - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO21_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO22_SEL` reader - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO22_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO22_SEL` writer - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO22_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO22_EN` reader - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO22_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO22_EN` writer - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO22_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO23_SEL` reader - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO23_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO23_SEL` writer - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO23_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO23_EN` reader - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO23_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO23_EN` writer - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO23_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO24_SEL` reader - Configures to select an ETM task channel for GPIO24.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO24_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO24_SEL` writer - Configures to select an ETM task channel for GPIO24.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO24_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO24_EN` reader - Configures whether or not to enable GPIO29 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO24_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO24_EN` writer - Configures whether or not to enable GPIO29 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO24_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio20_sel(&self) -> ETM_TASK_GPIO20_SEL_R {
        ETM_TASK_GPIO20_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio20_en(&self) -> ETM_TASK_GPIO20_EN_R {
        ETM_TASK_GPIO20_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio21_sel(&self) -> ETM_TASK_GPIO21_SEL_R {
        ETM_TASK_GPIO21_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio21_en(&self) -> ETM_TASK_GPIO21_EN_R {
        ETM_TASK_GPIO21_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio22_sel(&self) -> ETM_TASK_GPIO22_SEL_R {
        ETM_TASK_GPIO22_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio22_en(&self) -> ETM_TASK_GPIO22_EN_R {
        ETM_TASK_GPIO22_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio23_sel(&self) -> ETM_TASK_GPIO23_SEL_R {
        ETM_TASK_GPIO23_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio23_en(&self) -> ETM_TASK_GPIO23_EN_R {
        ETM_TASK_GPIO23_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO24.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio24_sel(&self) -> ETM_TASK_GPIO24_SEL_R {
        ETM_TASK_GPIO24_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO29 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio24_en(&self) -> ETM_TASK_GPIO24_EN_R {
        ETM_TASK_GPIO24_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P4_CFG")
            .field("etm_task_gpio20_sel", &self.etm_task_gpio20_sel())
            .field("etm_task_gpio20_en", &self.etm_task_gpio20_en())
            .field("etm_task_gpio21_sel", &self.etm_task_gpio21_sel())
            .field("etm_task_gpio21_en", &self.etm_task_gpio21_en())
            .field("etm_task_gpio22_sel", &self.etm_task_gpio22_sel())
            .field("etm_task_gpio22_en", &self.etm_task_gpio22_en())
            .field("etm_task_gpio23_sel", &self.etm_task_gpio23_sel())
            .field("etm_task_gpio23_en", &self.etm_task_gpio23_en())
            .field("etm_task_gpio24_sel", &self.etm_task_gpio24_sel())
            .field("etm_task_gpio24_en", &self.etm_task_gpio24_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio20_sel(&mut self) -> ETM_TASK_GPIO20_SEL_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO20_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO5 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio20_en(&mut self) -> ETM_TASK_GPIO20_EN_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO20_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO6.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio21_sel(&mut self) -> ETM_TASK_GPIO21_SEL_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO21_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO11 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio21_en(&mut self) -> ETM_TASK_GPIO21_EN_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO21_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO12.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio22_sel(&mut self) -> ETM_TASK_GPIO22_SEL_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO22_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO17 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio22_en(&mut self) -> ETM_TASK_GPIO22_EN_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO22_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO18.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio23_sel(&mut self) -> ETM_TASK_GPIO23_SEL_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO23_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO23 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio23_en(&mut self) -> ETM_TASK_GPIO23_EN_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO23_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO24.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\%\\label{fielddesc:GPIOSDETMTASKGPIO1EN}- \\[GPIOSD_ETM_TASK_GPIO1_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO1SEL}- \\[GPIOSD_ETM_TASK_GPIO1_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO2EN}- \\[GPIOSD_ETM_TASK_GPIO2_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO2SEL}- \\[GPIOSD_ETM_TASK_GPIO2_SEL\\] GPIO choose a etm task channel. %\\label{fielddesc:GPIOSDETMTASKGPIO3EN}\\item \\[GPIOSD_ETM_TASK_GPIO3_EN\\] Enable bit of GPIO response etm task. %\\label{fielddesc:GPIOSDETMTASKGPIO3SEL}\\item \\[GPIOSD_ETM_TASK_GPIO3_SEL\\] GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio24_sel(&mut self) -> ETM_TASK_GPIO24_SEL_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO24_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO29 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio24_en(&mut self) -> ETM_TASK_GPIO24_EN_W<'_, ETM_TASK_P4_CFG_SPEC> {
        ETM_TASK_GPIO24_EN_W::new(self, 29)
    }
}
#[doc = "GPIO selection register 4 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p4_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p4_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P4_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P4_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p4_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P4_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p4_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P4_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P4_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P4_CFG_SPEC {}
