#[doc = "Register `SAR_TSENS_CTRL` reader"]
pub type R = crate::R<SAR_TSENS_CTRL_SPEC>;
#[doc = "Register `SAR_TSENS_CTRL` writer"]
pub type W = crate::W<SAR_TSENS_CTRL_SPEC>;
#[doc = "Field `TSENS_OUT` reader - Temperature sensor data out."]
pub type TSENS_OUT_R = crate::FieldReader;
#[doc = "Field `TSENS_READY` reader - Indicate temperature sensor out ready."]
pub type TSENS_READY_R = crate::BitReader;
#[doc = "Field `TSENS_INT_EN` reader - Enable temperature sensor to send out interrupt."]
pub type TSENS_INT_EN_R = crate::BitReader;
#[doc = "Field `TSENS_INT_EN` writer - Enable temperature sensor to send out interrupt."]
pub type TSENS_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_IN_INV` reader - Invert temperature sensor data."]
pub type TSENS_IN_INV_R = crate::BitReader;
#[doc = "Field `TSENS_IN_INV` writer - Invert temperature sensor data."]
pub type TSENS_IN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_CLK_DIV` reader - Temperature sensor clock divider."]
pub type TSENS_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `TSENS_CLK_DIV` writer - Temperature sensor clock divider."]
pub type TSENS_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TSENS_POWER_UP` reader - Temperature sensor power up."]
pub type TSENS_POWER_UP_R = crate::BitReader;
#[doc = "Field `TSENS_POWER_UP` writer - Temperature sensor power up."]
pub type TSENS_POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_POWER_UP_FORCE` reader - 1: dump out and power up controlled by software. 0: by FSM."]
pub type TSENS_POWER_UP_FORCE_R = crate::BitReader;
#[doc = "Field `TSENS_POWER_UP_FORCE` writer - 1: dump out and power up controlled by software. 0: by FSM."]
pub type TSENS_POWER_UP_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_DUMP_OUT` reader - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
pub type TSENS_DUMP_OUT_R = crate::BitReader;
#[doc = "Field `TSENS_DUMP_OUT` writer - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
pub type TSENS_DUMP_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Temperature sensor data out."]
    #[inline(always)]
    pub fn tsens_out(&self) -> TSENS_OUT_R {
        TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Indicate temperature sensor out ready."]
    #[inline(always)]
    pub fn tsens_ready(&self) -> TSENS_READY_R {
        TSENS_READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    pub fn tsens_int_en(&self) -> TSENS_INT_EN_R {
        TSENS_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    pub fn tsens_power_up(&self) -> TSENS_POWER_UP_R {
        TSENS_POWER_UP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: dump out and power up controlled by software. 0: by FSM."]
    #[inline(always)]
    pub fn tsens_power_up_force(&self) -> TSENS_POWER_UP_FORCE_R {
        TSENS_POWER_UP_FORCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
    #[inline(always)]
    pub fn tsens_dump_out(&self) -> TSENS_DUMP_OUT_R {
        TSENS_DUMP_OUT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TSENS_CTRL")
            .field("tsens_out", &self.tsens_out())
            .field("tsens_ready", &self.tsens_ready())
            .field("tsens_int_en", &self.tsens_int_en())
            .field("tsens_in_inv", &self.tsens_in_inv())
            .field("tsens_clk_div", &self.tsens_clk_div())
            .field("tsens_power_up", &self.tsens_power_up())
            .field("tsens_power_up_force", &self.tsens_power_up_force())
            .field("tsens_dump_out", &self.tsens_dump_out())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    pub fn tsens_int_en(&mut self) -> TSENS_INT_EN_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_INT_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_IN_INV_W::new(self, 13)
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_CLK_DIV_W::new(self, 14)
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    pub fn tsens_power_up(&mut self) -> TSENS_POWER_UP_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_POWER_UP_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: dump out and power up controlled by software. 0: by FSM."]
    #[inline(always)]
    pub fn tsens_power_up_force(&mut self) -> TSENS_POWER_UP_FORCE_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_POWER_UP_FORCE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Temperature sensor dump out only active when SENS_TSENS_POWER_UP_FORCE = 1."]
    #[inline(always)]
    pub fn tsens_dump_out(&mut self) -> TSENS_DUMP_OUT_W<SAR_TSENS_CTRL_SPEC> {
        TSENS_DUMP_OUT_W::new(self, 24)
    }
}
#[doc = "Temperature sensor data control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_tsens_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_tsens_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_tsens_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_tsens_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL to value 0x0001_9000"]
impl crate::Resettable for SAR_TSENS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_9000;
}
