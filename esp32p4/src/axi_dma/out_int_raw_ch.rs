#[doc = "Register `OUT_INT_RAW_CH%s` reader"]
pub type R = crate::R<OUT_INT_RAW_CH_SPEC>;
#[doc = "Register `OUT_INT_RAW_CH%s` writer"]
pub type W = crate::W<OUT_INT_RAW_CH_SPEC>;
#[doc = "Field `OUT_DONE_CH_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
pub type OUT_DONE_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DONE_CH_INT_RAW` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
pub type OUT_DONE_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
pub type OUT_EOF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_CH_INT_RAW` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
pub type OUT_EOF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_RAW` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
pub type OUT_DSCR_ERR_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_RAW` writer - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
pub type OUT_DSCR_ERR_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_RAW` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
pub type OUT_TOTAL_EOF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_RAW` writer - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
pub type OUT_TOTAL_EOF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_OVF_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L1_OVF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_OVF_CH_INT_RAW` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L1_OVF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_UDF_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L1_UDF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_UDF_CH_INT_RAW` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L1_UDF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_OVF_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L2_OVF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_OVF_CH_INT_RAW` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L2_OVF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_UDF_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L2_UDF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_UDF_CH_INT_RAW` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L2_UDF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_OVF_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L3_OVF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_OVF_CH_INT_RAW` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L3_OVF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_UDF_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L3_UDF_CH_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_UDF_CH_INT_RAW` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L3_UDF_CH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
    #[inline(always)]
    pub fn out_done_ch_int_raw(&self) -> OUT_DONE_CH_INT_RAW_R {
        OUT_DONE_CH_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
    #[inline(always)]
    pub fn out_eof_ch_int_raw(&self) -> OUT_EOF_CH_INT_RAW_R {
        OUT_EOF_CH_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_raw(&self) -> OUT_DSCR_ERR_CH_INT_RAW_R {
        OUT_DSCR_ERR_CH_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_raw(&self) -> OUT_TOTAL_EOF_CH_INT_RAW_R {
        OUT_TOTAL_EOF_CH_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    pub fn outfifo_l1_ovf_ch_int_raw(&self) -> OUTFIFO_L1_OVF_CH_INT_RAW_R {
        OUTFIFO_L1_OVF_CH_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    pub fn outfifo_l1_udf_ch_int_raw(&self) -> OUTFIFO_L1_UDF_CH_INT_RAW_R {
        OUTFIFO_L1_UDF_CH_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    pub fn outfifo_l2_ovf_ch_int_raw(&self) -> OUTFIFO_L2_OVF_CH_INT_RAW_R {
        OUTFIFO_L2_OVF_CH_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    pub fn outfifo_l2_udf_ch_int_raw(&self) -> OUTFIFO_L2_UDF_CH_INT_RAW_R {
        OUTFIFO_L2_UDF_CH_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    pub fn outfifo_l3_ovf_ch_int_raw(&self) -> OUTFIFO_L3_OVF_CH_INT_RAW_R {
        OUTFIFO_L3_OVF_CH_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    pub fn outfifo_l3_udf_ch_int_raw(&self) -> OUTFIFO_L3_UDF_CH_INT_RAW_R {
        OUTFIFO_L3_UDF_CH_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_INT_RAW_CH")
            .field(
                "out_done_ch_int_raw",
                &format_args!("{}", self.out_done_ch_int_raw().bit()),
            )
            .field(
                "out_eof_ch_int_raw",
                &format_args!("{}", self.out_eof_ch_int_raw().bit()),
            )
            .field(
                "out_dscr_err_ch_int_raw",
                &format_args!("{}", self.out_dscr_err_ch_int_raw().bit()),
            )
            .field(
                "out_total_eof_ch_int_raw",
                &format_args!("{}", self.out_total_eof_ch_int_raw().bit()),
            )
            .field(
                "outfifo_l1_ovf_ch_int_raw",
                &format_args!("{}", self.outfifo_l1_ovf_ch_int_raw().bit()),
            )
            .field(
                "outfifo_l1_udf_ch_int_raw",
                &format_args!("{}", self.outfifo_l1_udf_ch_int_raw().bit()),
            )
            .field(
                "outfifo_l2_ovf_ch_int_raw",
                &format_args!("{}", self.outfifo_l2_ovf_ch_int_raw().bit()),
            )
            .field(
                "outfifo_l2_udf_ch_int_raw",
                &format_args!("{}", self.outfifo_l2_udf_ch_int_raw().bit()),
            )
            .field(
                "outfifo_l3_ovf_ch_int_raw",
                &format_args!("{}", self.outfifo_l3_ovf_ch_int_raw().bit()),
            )
            .field(
                "outfifo_l3_udf_ch_int_raw",
                &format_args!("{}", self.outfifo_l3_udf_ch_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_RAW_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_ch_int_raw(&mut self) -> OUT_DONE_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUT_DONE_CH_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_ch_int_raw(&mut self) -> OUT_EOF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUT_EOF_CH_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_ch_int_raw(&mut self) -> OUT_DSCR_ERR_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUT_DSCR_ERR_CH_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_ch_int_raw(&mut self) -> OUT_TOTAL_EOF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUT_TOTAL_EOF_CH_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l1_ovf_ch_int_raw(
        &mut self,
    ) -> OUTFIFO_L1_OVF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUTFIFO_L1_OVF_CH_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l1_udf_ch_int_raw(
        &mut self,
    ) -> OUTFIFO_L1_UDF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUTFIFO_L1_UDF_CH_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l2_ovf_ch_int_raw(
        &mut self,
    ) -> OUTFIFO_L2_OVF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUTFIFO_L2_OVF_CH_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l2_udf_ch_int_raw(
        &mut self,
    ) -> OUTFIFO_L2_UDF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUTFIFO_L2_UDF_CH_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l3_ovf_ch_int_raw(
        &mut self,
    ) -> OUTFIFO_L3_OVF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUTFIFO_L3_OVF_CH_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l3_udf_ch_int_raw(
        &mut self,
    ) -> OUTFIFO_L3_UDF_CH_INT_RAW_W<OUT_INT_RAW_CH_SPEC> {
        OUTFIFO_L3_UDF_CH_INT_RAW_W::new(self, 9)
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
#[doc = "Raw status interrupt of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_RAW_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_RAW_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_int_raw_ch::R`](R) reader structure"]
impl crate::Readable for OUT_INT_RAW_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_int_raw_ch::W`](W) writer structure"]
impl crate::Writable for OUT_INT_RAW_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_INT_RAW_CH%s to value 0"]
impl crate::Resettable for OUT_INT_RAW_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
