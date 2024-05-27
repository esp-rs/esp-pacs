///Register `HOST_SLCHOST_STATE_W1` reader
pub type R = crate::R<HOST_SLCHOST_STATE_W1_SPEC>;
///Field `HOST_SLCHOST_STATE4` reader -
pub type HOST_SLCHOST_STATE4_R = crate::FieldReader;
///Field `HOST_SLCHOST_STATE5` reader -
pub type HOST_SLCHOST_STATE5_R = crate::FieldReader;
///Field `HOST_SLCHOST_STATE6` reader -
pub type HOST_SLCHOST_STATE6_R = crate::FieldReader;
///Field `HOST_SLCHOST_STATE7` reader -
pub type HOST_SLCHOST_STATE7_R = crate::FieldReader;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn host_slchost_state4(&self) -> HOST_SLCHOST_STATE4_R {
        HOST_SLCHOST_STATE4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn host_slchost_state5(&self) -> HOST_SLCHOST_STATE5_R {
        HOST_SLCHOST_STATE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn host_slchost_state6(&self) -> HOST_SLCHOST_STATE6_R {
        HOST_SLCHOST_STATE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn host_slchost_state7(&self) -> HOST_SLCHOST_STATE7_R {
        HOST_SLCHOST_STATE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_STATE_W1")
            .field("host_slchost_state4", &self.host_slchost_state4())
            .field("host_slchost_state5", &self.host_slchost_state5())
            .field("host_slchost_state6", &self.host_slchost_state6())
            .field("host_slchost_state7", &self.host_slchost_state7())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_state_w1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLCHOST_STATE_W1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_STATE_W1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slchost_state_w1::R`](R) reader structure
impl crate::Readable for HOST_SLCHOST_STATE_W1_SPEC {}
///`reset()` method sets HOST_SLCHOST_STATE_W1 to value 0
impl crate::Resettable for HOST_SLCHOST_STATE_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
