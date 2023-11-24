#[doc = "Register `SAR_SLAVE_ADDR1` reader"]
pub type R = crate::R<SAR_SLAVE_ADDR1_SPEC>;
#[doc = "Register `SAR_SLAVE_ADDR1` writer"]
pub type W = crate::W<SAR_SLAVE_ADDR1_SPEC>;
#[doc = "Field `I2C_SLAVE_ADDR1` reader - "]
pub type I2C_SLAVE_ADDR1_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR1` writer - "]
pub type I2C_SLAVE_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `I2C_SLAVE_ADDR0` reader - "]
pub type I2C_SLAVE_ADDR0_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR0` writer - "]
pub type I2C_SLAVE_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MEAS_STATUS` reader - "]
pub type MEAS_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr1(&self) -> I2C_SLAVE_ADDR1_R {
        I2C_SLAVE_ADDR1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr0(&self) -> I2C_SLAVE_ADDR0_R {
        I2C_SLAVE_ADDR0_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn meas_status(&self) -> MEAS_STATUS_R {
        MEAS_STATUS_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR1")
            .field(
                "i2c_slave_addr1",
                &format_args!("{}", self.i2c_slave_addr1().bits()),
            )
            .field(
                "i2c_slave_addr0",
                &format_args!("{}", self.i2c_slave_addr0().bits()),
            )
            .field(
                "meas_status",
                &format_args!("{}", self.meas_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_SLAVE_ADDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr1(&mut self) -> I2C_SLAVE_ADDR1_W<SAR_SLAVE_ADDR1_SPEC> {
        I2C_SLAVE_ADDR1_W::new(self, 0)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr0(&mut self) -> I2C_SLAVE_ADDR0_W<SAR_SLAVE_ADDR1_SPEC> {
        I2C_SLAVE_ADDR0_W::new(self, 11)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SLAVE_ADDR1_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_slave_addr1::R`](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_slave_addr1::W`](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR1 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
