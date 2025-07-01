#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `IN_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 0."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 0."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1` reader - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1` writer - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1` reader - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1` writer - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L2` reader - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L2_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L2` writer - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L2` reader - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L2_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L2` writer - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_OVF` reader - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type IN_DSCR_TASK_OVF_R = crate::BitReader;
#[doc = "Field `IN_DSCR_TASK_OVF` writer - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type IN_DSCR_TASK_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 0."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> INFIFO_OVF_L1_R {
        INFIFO_OVF_L1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> INFIFO_UDF_L1_R {
        INFIFO_UDF_L1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l2(&self) -> INFIFO_OVF_L2_R {
        INFIFO_OVF_L2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l2(&self) -> INFIFO_UDF_L2_R {
        INFIFO_UDF_L2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    pub fn in_dscr_task_ovf(&self) -> IN_DSCR_TASK_OVF_R {
        IN_DSCR_TASK_OVF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("infifo_ovf_l1", &self.infifo_ovf_l1())
            .field("infifo_udf_l1", &self.infifo_udf_l1())
            .field("infifo_ovf_l2", &self.infifo_ovf_l2())
            .field("infifo_udf_l2", &self.infifo_udf_l2())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("in_dscr_task_ovf", &self.in_dscr_task_ovf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 0."]
    #[inline(always)]
    pub fn in_done(&mut self) -> IN_DONE_W<INT_RAW_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<INT_RAW_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and data error is detected"]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<INT_RAW_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<INT_RAW_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&mut self) -> INFIFO_OVF_L1_W<INT_RAW_SPEC> {
        INFIFO_OVF_L1_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1(&mut self) -> INFIFO_UDF_L1_W<INT_RAW_SPEC> {
        INFIFO_UDF_L1_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l2(&mut self) -> INFIFO_OVF_L2_W<INT_RAW_SPEC> {
        INFIFO_OVF_L2_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l2(&mut self) -> INFIFO_UDF_L2_W<INT_RAW_SPEC> {
        INFIFO_UDF_L2_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when the last descriptor is done but fifo also remain data."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<INT_RAW_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    pub fn in_dscr_task_ovf(&mut self) -> IN_DSCR_TASK_OVF_W<INT_RAW_SPEC> {
        IN_DSCR_TASK_OVF_W::new(self, 9)
    }
}
#[doc = "RX CHx interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
