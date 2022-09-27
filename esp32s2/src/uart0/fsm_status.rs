#[doc = "Register `FSM_STATUS` reader"]
pub struct R(crate::R<FSM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ST_URX_OUT` reader - This is the status register of the receiver."]
pub type ST_URX_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST_UTX_OUT` reader - This is the status register of the transmitter."]
pub type ST_UTX_OUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - This is the status register of the receiver."]
    #[inline(always)]
    pub fn st_urx_out(&self) -> ST_URX_OUT_R {
        ST_URX_OUT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This is the status register of the transmitter."]
    #[inline(always)]
    pub fn st_utx_out(&self) -> ST_UTX_OUT_R {
        ST_UTX_OUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "UART transmitter and receiver status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_status](index.html) module"]
pub struct FSM_STATUS_SPEC;
impl crate::RegisterSpec for FSM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_status::R](R) reader structure"]
impl crate::Readable for FSM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSM_STATUS to value 0"]
impl crate::Resettable for FSM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
