#[doc = "Register `RMT_SCLK_CONF` reader"]
pub type R = crate::R<RMT_SCLK_CONF_SPEC>;
#[doc = "Register `RMT_SCLK_CONF` writer"]
pub type W = crate::W<RMT_SCLK_CONF_SPEC>;
#[doc = "Field `SCLK_DIV_A` reader - The denominator of the frequency divider factor of the rmt function clock."]
pub type SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_A` writer - The denominator of the frequency divider factor of the rmt function clock."]
pub type SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCLK_DIV_B` reader - The numerator of the frequency divider factor of the rmt function clock."]
pub type SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_B` writer - The numerator of the frequency divider factor of the rmt function clock."]
pub type SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCLK_DIV_NUM` reader - The integral part of the frequency divider factor of the rmt function clock."]
pub type SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV_NUM` writer - The integral part of the frequency divider factor of the rmt function clock."]
pub type SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_SEL` reader - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
pub type SCLK_SEL_R = crate::BitReader;
#[doc = "Field `SCLK_SEL` writer - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
pub type SCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_EN` reader - Set 1 to enable rmt function clock"]
pub type SCLK_EN_R = crate::BitReader;
#[doc = "Field `SCLK_EN` writer - Set 1 to enable rmt function clock"]
pub type SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set 1 to enable rmt function clock"]
    #[inline(always)]
    pub fn sclk_en(&self) -> SCLK_EN_R {
        SCLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT_SCLK_CONF")
            .field("sclk_div_a", &self.sclk_div_a())
            .field("sclk_div_b", &self.sclk_div_b())
            .field("sclk_div_num", &self.sclk_div_num())
            .field("sclk_sel", &self.sclk_sel())
            .field("sclk_en", &self.sclk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W<RMT_SCLK_CONF_SPEC> {
        SCLK_DIV_A_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W<RMT_SCLK_CONF_SPEC> {
        SCLK_DIV_B_W::new(self, 6)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the rmt function clock."]
    #[inline(always)]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<RMT_SCLK_CONF_SPEC> {
        SCLK_DIV_NUM_W::new(self, 12)
    }
    #[doc = "Bit 20 - set this field to select clock-source. 0: do not select anyone clock, 1(default): 80MHz, 2: FOSC, 3: XTAL."]
    #[inline(always)]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W<RMT_SCLK_CONF_SPEC> {
        SCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set 1 to enable rmt function clock"]
    #[inline(always)]
    pub fn sclk_en(&mut self) -> SCLK_EN_W<RMT_SCLK_CONF_SPEC> {
        SCLK_EN_W::new(self, 21)
    }
}
#[doc = "RMT_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_sclk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_sclk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMT_SCLK_CONF_SPEC;
impl crate::RegisterSpec for RMT_SCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmt_sclk_conf::R`](R) reader structure"]
impl crate::Readable for RMT_SCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmt_sclk_conf::W`](W) writer structure"]
impl crate::Writable for RMT_SCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMT_SCLK_CONF to value 0x0030_1000"]
impl crate::Resettable for RMT_SCLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0030_1000;
}
