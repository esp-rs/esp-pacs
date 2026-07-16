#[doc = "Register `I2S1_RX_DIV_CTRL0` reader"]
pub type R = crate::R<I2S1_RX_DIV_CTRL0_SPEC>;
#[doc = "Register `I2S1_RX_DIV_CTRL0` writer"]
pub type W = crate::W<I2S1_RX_DIV_CTRL0_SPEC>;
#[doc = "Field `I2S1_RX_DIV_X` reader - need_des"]
pub type I2S1_RX_DIV_X_R = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_X` writer - need_des"]
pub type I2S1_RX_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_Y` reader - need_des"]
pub type I2S1_RX_DIV_Y_R = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_Y` writer - need_des"]
pub type I2S1_RX_DIV_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_Z` reader - need_des"]
pub type I2S1_RX_DIV_Z_R = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_Z` writer - need_des"]
pub type I2S1_RX_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_YN1` reader - need_des"]
pub type I2S1_RX_DIV_YN1_R = crate::BitReader;
#[doc = "Field `I2S1_RX_DIV_YN1` writer - need_des"]
pub type I2S1_RX_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_x(&self) -> I2S1_RX_DIV_X_R {
        I2S1_RX_DIV_X_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_y(&self) -> I2S1_RX_DIV_Y_R {
        I2S1_RX_DIV_Y_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_z(&self) -> I2S1_RX_DIV_Z_R {
        I2S1_RX_DIV_Z_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_yn1(&self) -> I2S1_RX_DIV_YN1_R {
        I2S1_RX_DIV_YN1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S1_RX_DIV_CTRL0")
            .field("i2s1_rx_div_x", &self.i2s1_rx_div_x())
            .field("i2s1_rx_div_y", &self.i2s1_rx_div_y())
            .field("i2s1_rx_div_z", &self.i2s1_rx_div_z())
            .field("i2s1_rx_div_yn1", &self.i2s1_rx_div_yn1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_x(&mut self) -> I2S1_RX_DIV_X_W<'_, I2S1_RX_DIV_CTRL0_SPEC> {
        I2S1_RX_DIV_X_W::new(self, 0)
    }
    #[doc = "Bits 9:17 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_y(&mut self) -> I2S1_RX_DIV_Y_W<'_, I2S1_RX_DIV_CTRL0_SPEC> {
        I2S1_RX_DIV_Y_W::new(self, 9)
    }
    #[doc = "Bits 18:26 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_z(&mut self) -> I2S1_RX_DIV_Z_W<'_, I2S1_RX_DIV_CTRL0_SPEC> {
        I2S1_RX_DIV_Z_W::new(self, 18)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn i2s1_rx_div_yn1(&mut self) -> I2S1_RX_DIV_YN1_W<'_, I2S1_RX_DIV_CTRL0_SPEC> {
        I2S1_RX_DIV_YN1_W::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_rx_div_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_rx_div_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S1_RX_DIV_CTRL0_SPEC;
impl crate::RegisterSpec for I2S1_RX_DIV_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s1_rx_div_ctrl0::R`](R) reader structure"]
impl crate::Readable for I2S1_RX_DIV_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s1_rx_div_ctrl0::W`](W) writer structure"]
impl crate::Writable for I2S1_RX_DIV_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S1_RX_DIV_CTRL0 to value 0"]
impl crate::Resettable for I2S1_RX_DIV_CTRL0_SPEC {}
