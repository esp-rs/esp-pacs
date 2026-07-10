#[doc = "Register `IN_DONE_DES_ADDR_CH0` reader"]
pub type R = crate::R<IN_DONE_DES_ADDR_CH0_SPEC>;
#[doc = "Field `IN_DONE_DES_ADDR_CH0` reader - Represents the address of the inlink descriptor when this descriptor is completed ."]
pub type IN_DONE_DES_ADDR_CH0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the inlink descriptor when this descriptor is completed ."]
    #[inline(always)]
    pub fn in_done_des_addr_ch0(&self) -> IN_DONE_DES_ADDR_CH0_R {
        IN_DONE_DES_ADDR_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DONE_DES_ADDR_CH0")
            .field("in_done_des_addr_ch0", &self.in_done_des_addr_ch0())
            .finish()
    }
}
#[doc = "RX_done Inlink descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_DONE_DES_ADDR_CH0_SPEC;
impl crate::RegisterSpec for IN_DONE_DES_ADDR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_done_des_addr_ch0::R`](R) reader structure"]
impl crate::Readable for IN_DONE_DES_ADDR_CH0_SPEC {}
#[doc = "`reset()` method sets IN_DONE_DES_ADDR_CH0 to value 0"]
impl crate::Resettable for IN_DONE_DES_ADDR_CH0_SPEC {}
