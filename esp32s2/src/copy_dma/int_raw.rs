#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `IN_DONE` reader - Triggered when the last data of frame is received or the receive buffer is full indicated by receive descriptor."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - Triggered when the last data of one frame is received."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `OUT_DONE` reader - Triggered when all data indicated by one transmit descriptor has been pushed into TX FIFO."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - Triggered when the last data with EOF flag has been pushed into TX FIFO."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` reader - Triggered when detecting receive descriptor error, including owner error, the second and third word error of receive descriptor."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` reader - Triggered when detecting transmit descriptor error, including owner error, the second and third word error of transmit descriptor."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` reader - Triggered when receiving data is completed and no more receive descriptor."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - Triggered when data corresponding to all transmit descriptors and the last descriptor with valid EOF is transmitted out."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Triggered when the last data of frame is received or the receive buffer is full indicated by receive descriptor."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggered when the last data of one frame is received."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Triggered when all data indicated by one transmit descriptor has been pushed into TX FIFO."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Triggered when the last data with EOF flag has been pushed into TX FIFO."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Triggered when detecting receive descriptor error, including owner error, the second and third word error of receive descriptor."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Triggered when detecting transmit descriptor error, including owner error, the second and third word error of transmit descriptor."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Triggered when receiving data is completed and no more receive descriptor."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Triggered when data corresponding to all transmit descriptors and the last descriptor with valid EOF is transmitted out."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("out_total_eof", &self.out_total_eof())
            .finish()
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}