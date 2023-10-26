#[doc = "Register `SAR_I2C_IO` reader"]
pub type R = crate::R<SAR_I2C_IO_SPEC>;
#[doc = "Register `SAR_I2C_IO` writer"]
pub type W = crate::W<SAR_I2C_IO_SPEC>;
#[doc = "Field `SAR_DEBUG_BIT_SEL` reader - "]
pub type SAR_DEBUG_BIT_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_DEBUG_BIT_SEL` writer - "]
pub type SAR_DEBUG_BIT_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SAR_I2C_SCL_SEL` reader - Ò0Ó using TOUCH_PAD\\[0\\] as i2c clk Ò1Ó using TOUCH_PAD\\[2\\] as i2c clk"]
pub type SAR_I2C_SCL_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_I2C_SCL_SEL` writer - Ò0Ó using TOUCH_PAD\\[0\\] as i2c clk Ò1Ó using TOUCH_PAD\\[2\\] as i2c clk"]
pub type SAR_I2C_SCL_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAR_I2C_SDA_SEL` reader - Ò0Ó using TOUCH_PAD\\[1\\] as i2c sda Ò1Ó using TOUCH_PAD\\[3\\] as i2c sda"]
pub type SAR_I2C_SDA_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_I2C_SDA_SEL` writer - Ò0Ó using TOUCH_PAD\\[1\\] as i2c sda Ò1Ó using TOUCH_PAD\\[3\\] as i2c sda"]
pub type SAR_I2C_SDA_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn sar_debug_bit_sel(&self) -> SAR_DEBUG_BIT_SEL_R {
        SAR_DEBUG_BIT_SEL_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Ò0Ó using TOUCH_PAD\\[0\\] as i2c clk Ò1Ó using TOUCH_PAD\\[2\\] as i2c clk"]
    #[inline(always)]
    pub fn sar_i2c_scl_sel(&self) -> SAR_I2C_SCL_SEL_R {
        SAR_I2C_SCL_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Ò0Ó using TOUCH_PAD\\[1\\] as i2c sda Ò1Ó using TOUCH_PAD\\[3\\] as i2c sda"]
    #[inline(always)]
    pub fn sar_i2c_sda_sel(&self) -> SAR_I2C_SDA_SEL_R {
        SAR_I2C_SDA_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_I2C_IO")
            .field(
                "sar_debug_bit_sel",
                &format_args!("{}", self.sar_debug_bit_sel().bits()),
            )
            .field(
                "sar_i2c_scl_sel",
                &format_args!("{}", self.sar_i2c_scl_sel().bits()),
            )
            .field(
                "sar_i2c_sda_sel",
                &format_args!("{}", self.sar_i2c_sda_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_I2C_IO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 23:27"]
    #[inline(always)]
    #[must_use]
    pub fn sar_debug_bit_sel(&mut self) -> SAR_DEBUG_BIT_SEL_W<SAR_I2C_IO_SPEC, 23> {
        SAR_DEBUG_BIT_SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - Ò0Ó using TOUCH_PAD\\[0\\] as i2c clk Ò1Ó using TOUCH_PAD\\[2\\] as i2c clk"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_scl_sel(&mut self) -> SAR_I2C_SCL_SEL_W<SAR_I2C_IO_SPEC, 28> {
        SAR_I2C_SCL_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - Ò0Ó using TOUCH_PAD\\[1\\] as i2c sda Ò1Ó using TOUCH_PAD\\[3\\] as i2c sda"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_sda_sel(&mut self) -> SAR_I2C_SDA_SEL_W<SAR_I2C_IO_SPEC, 30> {
        SAR_I2C_SDA_SEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_io::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_io::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_I2C_IO_SPEC;
impl crate::RegisterSpec for SAR_I2C_IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_i2c_io::R`](R) reader structure"]
impl crate::Readable for SAR_I2C_IO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_i2c_io::W`](W) writer structure"]
impl crate::Writable for SAR_I2C_IO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_I2C_IO to value 0"]
impl crate::Resettable for SAR_I2C_IO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
