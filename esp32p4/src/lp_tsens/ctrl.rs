#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `OUT` reader - Temperature sensor data out."]
pub type OUT_R = crate::FieldReader;
#[doc = "Field `READY` reader - Indicate temperature sensor out ready."]
pub type READY_R = crate::BitReader;
#[doc = "Field `SAMPLE_EN` reader - Enable sample signal for wakeup module."]
pub type SAMPLE_EN_R = crate::BitReader;
#[doc = "Field `SAMPLE_EN` writer - Enable sample signal for wakeup module."]
pub type SAMPLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_MASK` reader - Wake up signal mask."]
pub type WAKEUP_MASK_R = crate::BitReader;
#[doc = "Field `WAKEUP_MASK` writer - Wake up signal mask."]
pub type WAKEUP_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - Enable temperature sensor to send out interrupt."]
pub type INT_EN_R = crate::BitReader;
#[doc = "Field `INT_EN` writer - Enable temperature sensor to send out interrupt."]
pub type INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_INV` reader - Invert temperature sensor data."]
pub type IN_INV_R = crate::BitReader;
#[doc = "Field `IN_INV` writer - Invert temperature sensor data."]
pub type IN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DIV` reader - Temperature sensor clock divider."]
pub type CLK_DIV_R = crate::FieldReader;
#[doc = "Field `CLK_DIV` writer - Temperature sensor clock divider."]
pub type CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POWER_UP` reader - Temperature sensor power up."]
pub type POWER_UP_R = crate::BitReader;
#[doc = "Field `POWER_UP` writer - Temperature sensor power up."]
pub type POWER_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWER_UP_FORCE` reader - 1: dump out &amp; power up controlled by SW, 0: by FSM."]
pub type POWER_UP_FORCE_R = crate::BitReader;
#[doc = "Field `POWER_UP_FORCE` writer - 1: dump out &amp; power up controlled by SW, 0: by FSM."]
pub type POWER_UP_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Temperature sensor data out."]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Indicate temperature sensor out ready."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable sample signal for wakeup module."]
    #[inline(always)]
    pub fn sample_en(&self) -> SAMPLE_EN_R {
        SAMPLE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake up signal mask."]
    #[inline(always)]
    pub fn wakeup_mask(&self) -> WAKEUP_MASK_R {
        WAKEUP_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    pub fn in_inv(&self) -> IN_INV_R {
        IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    pub fn power_up(&self) -> POWER_UP_R {
        POWER_UP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: dump out &amp; power up controlled by SW, 0: by FSM."]
    #[inline(always)]
    pub fn power_up_force(&self) -> POWER_UP_FORCE_R {
        POWER_UP_FORCE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("out", &self.out())
            .field("ready", &self.ready())
            .field("sample_en", &self.sample_en())
            .field("wakeup_mask", &self.wakeup_mask())
            .field("int_en", &self.int_en())
            .field("in_inv", &self.in_inv())
            .field("clk_div", &self.clk_div())
            .field("power_up", &self.power_up())
            .field("power_up_force", &self.power_up_force())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - Enable sample signal for wakeup module."]
    #[inline(always)]
    #[must_use]
    pub fn sample_en(&mut self) -> SAMPLE_EN_W<CTRL_SPEC> {
        SAMPLE_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wake up signal mask."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_mask(&mut self) -> WAKEUP_MASK_W<CTRL_SPEC> {
        WAKEUP_MASK_W::new(self, 10)
    }
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<CTRL_SPEC> {
        INT_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    #[must_use]
    pub fn in_inv(&mut self) -> IN_INV_W<CTRL_SPEC> {
        IN_INV_W::new(self, 13)
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> CLK_DIV_W<CTRL_SPEC> {
        CLK_DIV_W::new(self, 14)
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    #[must_use]
    pub fn power_up(&mut self) -> POWER_UP_W<CTRL_SPEC> {
        POWER_UP_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: dump out &amp; power up controlled by SW, 0: by FSM."]
    #[inline(always)]
    #[must_use]
    pub fn power_up_force(&mut self) -> POWER_UP_FORCE_W<CTRL_SPEC> {
        POWER_UP_FORCE_W::new(self, 23)
    }
}
#[doc = "Tsens configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0001_9400"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_9400;
}
