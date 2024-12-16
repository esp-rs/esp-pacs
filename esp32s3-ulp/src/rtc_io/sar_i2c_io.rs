#[doc = "Register `SAR_I2C_IO` reader"]
pub type R = crate::R<SAR_I2C_IO_SPEC>;
#[doc = "Register `SAR_I2C_IO` writer"]
pub type W = crate::W<SAR_I2C_IO_SPEC>;
#[doc = "Field `SAR_DEBUG_BIT_SEL` reader - ******* Description configure***"]
pub type SAR_DEBUG_BIT_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_DEBUG_BIT_SEL` writer - ******* Description configure***"]
pub type SAR_DEBUG_BIT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SAR_I2C_SCL_SEL` reader - ******* Description configure***"]
pub type SAR_I2C_SCL_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_I2C_SCL_SEL` writer - ******* Description configure***"]
pub type SAR_I2C_SCL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR_I2C_SDA_SEL` reader - ******* Description configure***"]
pub type SAR_I2C_SDA_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_I2C_SDA_SEL` writer - ******* Description configure***"]
pub type SAR_I2C_SDA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 23:27 - ******* Description configure***"]
    #[inline(always)]
    pub fn sar_debug_bit_sel(&self) -> SAR_DEBUG_BIT_SEL_R {
        SAR_DEBUG_BIT_SEL_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - ******* Description configure***"]
    #[inline(always)]
    pub fn sar_i2c_scl_sel(&self) -> SAR_I2C_SCL_SEL_R {
        SAR_I2C_SCL_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - ******* Description configure***"]
    #[inline(always)]
    pub fn sar_i2c_sda_sel(&self) -> SAR_I2C_SDA_SEL_R {
        SAR_I2C_SDA_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_I2C_IO")
            .field("sar_debug_bit_sel", &self.sar_debug_bit_sel())
            .field("sar_i2c_scl_sel", &self.sar_i2c_scl_sel())
            .field("sar_i2c_sda_sel", &self.sar_i2c_sda_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 23:27 - ******* Description configure***"]
    #[inline(always)]
    pub fn sar_debug_bit_sel(&mut self) -> SAR_DEBUG_BIT_SEL_W<SAR_I2C_IO_SPEC> {
        SAR_DEBUG_BIT_SEL_W::new(self, 23)
    }
    #[doc = "Bits 28:29 - ******* Description configure***"]
    #[inline(always)]
    pub fn sar_i2c_scl_sel(&mut self) -> SAR_I2C_SCL_SEL_W<SAR_I2C_IO_SPEC> {
        SAR_I2C_SCL_SEL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - ******* Description configure***"]
    #[inline(always)]
    pub fn sar_i2c_sda_sel(&mut self) -> SAR_I2C_SDA_SEL_W<SAR_I2C_IO_SPEC> {
        SAR_I2C_SDA_SEL_W::new(self, 30)
    }
}
#[doc = "configure rtc i2c mux\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_i2c_io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_i2c_io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_I2C_IO_SPEC;
impl crate::RegisterSpec for SAR_I2C_IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_i2c_io::R`](R) reader structure"]
impl crate::Readable for SAR_I2C_IO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_i2c_io::W`](W) writer structure"]
impl crate::Writable for SAR_I2C_IO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_I2C_IO to value 0"]
impl crate::Resettable for SAR_I2C_IO_SPEC {
    const RESET_VALUE: u32 = 0;
}
