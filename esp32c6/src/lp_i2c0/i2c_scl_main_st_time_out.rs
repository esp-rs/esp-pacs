#[doc = "Register `I2C_SCL_MAIN_ST_TIME_OUT` reader"]
pub type R = crate::R<I2C_SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "Register `I2C_SCL_MAIN_ST_TIME_OUT` writer"]
pub type W = crate::W<I2C_SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "Field `I2C_SCL_MAIN_ST_TO_I2C` reader - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
pub type I2C_SCL_MAIN_ST_TO_I2C_R = crate::FieldReader;
#[doc = "Field `I2C_SCL_MAIN_ST_TO_I2C` writer - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
pub type I2C_SCL_MAIN_ST_TO_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
    #[inline(always)]
    pub fn i2c_scl_main_st_to_i2c(&self) -> I2C_SCL_MAIN_ST_TO_I2C_R {
        I2C_SCL_MAIN_ST_TO_I2C_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SCL_MAIN_ST_TIME_OUT")
            .field(
                "i2c_scl_main_st_to_i2c",
                &format_args!("{}", self.i2c_scl_main_st_to_i2c().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_SCL_MAIN_ST_TIME_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - The threshold value of SCL_MAIN_FSM state unchanged period.nIt should be o more than 23"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_main_st_to_i2c(
        &mut self,
    ) -> I2C_SCL_MAIN_ST_TO_I2C_W<I2C_SCL_MAIN_ST_TIME_OUT_SPEC> {
        I2C_SCL_MAIN_ST_TO_I2C_W::new(self, 0)
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
#[doc = "SCL main status time out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_main_st_time_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_main_st_time_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_SCL_MAIN_ST_TIME_OUT_SPEC;
impl crate::RegisterSpec for I2C_SCL_MAIN_ST_TIME_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_scl_main_st_time_out::R`](R) reader structure"]
impl crate::Readable for I2C_SCL_MAIN_ST_TIME_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_scl_main_st_time_out::W`](W) writer structure"]
impl crate::Writable for I2C_SCL_MAIN_ST_TIME_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_SCL_MAIN_ST_TIME_OUT to value 0x10"]
impl crate::Resettable for I2C_SCL_MAIN_ST_TIME_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
