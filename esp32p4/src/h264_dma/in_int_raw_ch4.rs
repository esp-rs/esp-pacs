#[doc = "Register `IN_INT_RAW_CH4` reader"]
pub type R = crate::R<IN_INT_RAW_CH4_SPEC>;
#[doc = "Register `IN_INT_RAW_CH4` writer"]
pub type W = crate::W<IN_INT_RAW_CH4_SPEC>;
#[doc = "Field `IN_DONE_CH4_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
pub type IN_DONE_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DONE_CH4_INT_RAW` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
pub type IN_DONE_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH4_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
pub type IN_SUC_EOF_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH4_INT_RAW` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
pub type IN_SUC_EOF_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH4_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
pub type IN_ERR_EOF_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH4_INT_RAW` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
pub type IN_ERR_EOF_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH4_INT_RAW` reader - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 1."]
pub type IN_DSCR_ERR_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH4_INT_RAW` writer - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 1."]
pub type IN_DSCR_ERR_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1_CH4_INT_RAW` reader - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L1_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1_CH4_INT_RAW` writer - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L1_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1_CH4_INT_RAW` reader - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L1_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1_CH4_INT_RAW` writer - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L1_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L2_CH4_INT_RAW` reader - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L2_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L2_CH4_INT_RAW` writer - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L2_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L2_CH4_INT_RAW` reader - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L2_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L2_CH4_INT_RAW` writer - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L2_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH4_INT_RAW` reader - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
pub type IN_DSCR_EMPTY_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH4_INT_RAW` writer - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
pub type IN_DSCR_EMPTY_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_OVF_CH4_INT_RAW` reader - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type IN_DSCR_TASK_OVF_CH4_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_TASK_OVF_CH4_INT_RAW` writer - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type IN_DSCR_TASK_OVF_CH4_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
    #[inline(always)]
    pub fn in_done_ch4_int_raw(&self) -> IN_DONE_CH4_INT_RAW_R {
        IN_DONE_CH4_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
    #[inline(always)]
    pub fn in_suc_eof_ch4_int_raw(&self) -> IN_SUC_EOF_CH4_INT_RAW_R {
        IN_SUC_EOF_CH4_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
    #[inline(always)]
    pub fn in_err_eof_ch4_int_raw(&self) -> IN_ERR_EOF_CH4_INT_RAW_R {
        IN_ERR_EOF_CH4_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 1."]
    #[inline(always)]
    pub fn in_dscr_err_ch4_int_raw(&self) -> IN_DSCR_ERR_CH4_INT_RAW_R {
        IN_DSCR_ERR_CH4_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1_ch4_int_raw(&self) -> INFIFO_OVF_L1_CH4_INT_RAW_R {
        INFIFO_OVF_L1_CH4_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1_ch4_int_raw(&self) -> INFIFO_UDF_L1_CH4_INT_RAW_R {
        INFIFO_UDF_L1_CH4_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l2_ch4_int_raw(&self) -> INFIFO_OVF_L2_CH4_INT_RAW_R {
        INFIFO_OVF_L2_CH4_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l2_ch4_int_raw(&self) -> INFIFO_UDF_L2_CH4_INT_RAW_R {
        INFIFO_UDF_L2_CH4_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
    #[inline(always)]
    pub fn in_dscr_empty_ch4_int_raw(&self) -> IN_DSCR_EMPTY_CH4_INT_RAW_R {
        IN_DSCR_EMPTY_CH4_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    pub fn in_dscr_task_ovf_ch4_int_raw(&self) -> IN_DSCR_TASK_OVF_CH4_INT_RAW_R {
        IN_DSCR_TASK_OVF_CH4_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_INT_RAW_CH4")
            .field(
                "in_done_ch4_int_raw",
                &format_args!("{}", self.in_done_ch4_int_raw().bit()),
            )
            .field(
                "in_suc_eof_ch4_int_raw",
                &format_args!("{}", self.in_suc_eof_ch4_int_raw().bit()),
            )
            .field(
                "in_err_eof_ch4_int_raw",
                &format_args!("{}", self.in_err_eof_ch4_int_raw().bit()),
            )
            .field(
                "in_dscr_err_ch4_int_raw",
                &format_args!("{}", self.in_dscr_err_ch4_int_raw().bit()),
            )
            .field(
                "infifo_ovf_l1_ch4_int_raw",
                &format_args!("{}", self.infifo_ovf_l1_ch4_int_raw().bit()),
            )
            .field(
                "infifo_udf_l1_ch4_int_raw",
                &format_args!("{}", self.infifo_udf_l1_ch4_int_raw().bit()),
            )
            .field(
                "infifo_ovf_l2_ch4_int_raw",
                &format_args!("{}", self.infifo_ovf_l2_ch4_int_raw().bit()),
            )
            .field(
                "infifo_udf_l2_ch4_int_raw",
                &format_args!("{}", self.infifo_udf_l2_ch4_int_raw().bit()),
            )
            .field(
                "in_dscr_empty_ch4_int_raw",
                &format_args!("{}", self.in_dscr_empty_ch4_int_raw().bit()),
            )
            .field(
                "in_dscr_task_ovf_ch4_int_raw",
                &format_args!("{}", self.in_dscr_task_ovf_ch4_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_RAW_CH4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_ch4_int_raw(&mut self) -> IN_DONE_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        IN_DONE_CH4_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_ch4_int_raw(&mut self) -> IN_SUC_EOF_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        IN_SUC_EOF_CH4_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_ch4_int_raw(&mut self) -> IN_ERR_EOF_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        IN_ERR_EOF_CH4_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_ch4_int_raw(&mut self) -> IN_DSCR_ERR_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        IN_DSCR_ERR_CH4_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1_ch4_int_raw(
        &mut self,
    ) -> INFIFO_OVF_L1_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        INFIFO_OVF_L1_CH4_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1_ch4_int_raw(
        &mut self,
    ) -> INFIFO_UDF_L1_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        INFIFO_UDF_L1_CH4_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l2_ch4_int_raw(
        &mut self,
    ) -> INFIFO_OVF_L2_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        INFIFO_OVF_L2_CH4_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l2_ch4_int_raw(
        &mut self,
    ) -> INFIFO_UDF_L2_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        INFIFO_UDF_L2_CH4_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_ch4_int_raw(
        &mut self,
    ) -> IN_DSCR_EMPTY_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        IN_DSCR_EMPTY_CH4_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_task_ovf_ch4_int_raw(
        &mut self,
    ) -> IN_DSCR_TASK_OVF_CH4_INT_RAW_W<IN_INT_RAW_CH4_SPEC> {
        IN_DSCR_TASK_OVF_CH4_INT_RAW_W::new(self, 9)
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
#[doc = "RX CH4 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_RAW_CH4_SPEC;
impl crate::RegisterSpec for IN_INT_RAW_CH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_int_raw_ch4::R`](R) reader structure"]
impl crate::Readable for IN_INT_RAW_CH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_int_raw_ch4::W`](W) writer structure"]
impl crate::Writable for IN_INT_RAW_CH4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_INT_RAW_CH4 to value 0"]
impl crate::Resettable for IN_INT_RAW_CH4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
