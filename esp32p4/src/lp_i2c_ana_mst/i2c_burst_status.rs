#[doc = "Register `I2C_BURST_STATUS` reader"]
pub type R = crate::R<I2C_BURST_STATUS_SPEC>;
#[doc = "Register `I2C_BURST_STATUS` writer"]
pub type W = crate::W<I2C_BURST_STATUS_SPEC>;
#[doc = "Field `I2C_MST_BURST_DONE` reader - need des"]
pub type I2C_MST_BURST_DONE_R = crate::BitReader;
#[doc = "Field `I2C_MST0_BURST_ERR_FLAG` reader - need des"]
pub type I2C_MST0_BURST_ERR_FLAG_R = crate::BitReader;
#[doc = "Field `I2C_MST1_BURST_ERR_FLAG` reader - need des"]
pub type I2C_MST1_BURST_ERR_FLAG_R = crate::BitReader;
#[doc = "Field `I2C_MST_BURST_TIMEOUT_CNT` reader - need des"]
pub type I2C_MST_BURST_TIMEOUT_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_MST_BURST_TIMEOUT_CNT` writer - need des"]
pub type I2C_MST_BURST_TIMEOUT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    pub fn i2c_mst_burst_done(&self) -> I2C_MST_BURST_DONE_R {
        I2C_MST_BURST_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need des"]
    #[inline(always)]
    pub fn i2c_mst0_burst_err_flag(&self) -> I2C_MST0_BURST_ERR_FLAG_R {
        I2C_MST0_BURST_ERR_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn i2c_mst1_burst_err_flag(&self) -> I2C_MST1_BURST_ERR_FLAG_R {
        I2C_MST1_BURST_ERR_FLAG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 20:31 - need des"]
    #[inline(always)]
    pub fn i2c_mst_burst_timeout_cnt(&self) -> I2C_MST_BURST_TIMEOUT_CNT_R {
        I2C_MST_BURST_TIMEOUT_CNT_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_BURST_STATUS")
            .field(
                "i2c_mst_burst_done",
                &format_args!("{}", self.i2c_mst_burst_done().bit()),
            )
            .field(
                "i2c_mst0_burst_err_flag",
                &format_args!("{}", self.i2c_mst0_burst_err_flag().bit()),
            )
            .field(
                "i2c_mst1_burst_err_flag",
                &format_args!("{}", self.i2c_mst1_burst_err_flag().bit()),
            )
            .field(
                "i2c_mst_burst_timeout_cnt",
                &format_args!("{}", self.i2c_mst_burst_timeout_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_BURST_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 20:31 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mst_burst_timeout_cnt(
        &mut self,
    ) -> I2C_MST_BURST_TIMEOUT_CNT_W<I2C_BURST_STATUS_SPEC> {
        I2C_MST_BURST_TIMEOUT_CNT_W::new(self, 20)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_burst_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_burst_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_BURST_STATUS_SPEC;
impl crate::RegisterSpec for I2C_BURST_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_burst_status::R`](R) reader structure"]
impl crate::Readable for I2C_BURST_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_burst_status::W`](W) writer structure"]
impl crate::Writable for I2C_BURST_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_BURST_STATUS to value 0x4000_0000"]
impl crate::Resettable for I2C_BURST_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
