///Register `STATUS` reader
pub type R = crate::R<STATUS_SPEC>;
///Field `VALUE` reader -
pub type VALUE_R = crate::FieldReader<u16>;
///Field `DIRECTION` reader -
pub type DIRECTION_R = crate::BitReader;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("value", &self.value())
            .field("direction", &self.direction())
            .finish()
    }
}
/**PWM TIMERx status register.

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUS_SPEC {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
