#[doc = "Register `CAP_STATUS` reader"]
pub type R = crate::R<CAP_STATUS_SPEC>;
#[doc = "Describes the last captured edge on channel%s.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0_EDGE {
    #[doc = "0: Rising edge"]
    Rising = 0,
    #[doc = "1: Falling edge"]
    Falling = 1,
}
impl From<CAP0_EDGE> for bool {
    #[inline(always)]
    fn from(variant: CAP0_EDGE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP_EDGE(0-2)` reader - Describes the last captured edge on channel%s."]
pub type CAP_EDGE_R = crate::BitReader<CAP0_EDGE>;
impl CAP_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0_EDGE {
        match self.bits {
            false => CAP0_EDGE::Rising,
            true => CAP0_EDGE::Falling,
        }
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CAP0_EDGE::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CAP0_EDGE::Falling
    }
}
impl R {
    #[doc = "Describes the last captured edge on channel(0-2)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CAP0_EDGE` field.</div>"]
    #[inline(always)]
    pub fn cap_edge(&self, n: u8) -> CAP_EDGE_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CAP_EDGE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Describes the last captured edge on channel(0-2)."]
    #[inline(always)]
    pub fn cap_edge_iter(&self) -> impl Iterator<Item = CAP_EDGE_R> + '_ {
        (0..3).map(move |n| CAP_EDGE_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Describes the last captured edge on channel0."]
    #[inline(always)]
    pub fn cap0_edge(&self) -> CAP_EDGE_R {
        CAP_EDGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Describes the last captured edge on channel1."]
    #[inline(always)]
    pub fn cap1_edge(&self) -> CAP_EDGE_R {
        CAP_EDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Describes the last captured edge on channel2."]
    #[inline(always)]
    pub fn cap2_edge(&self) -> CAP_EDGE_R {
        CAP_EDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_STATUS")
            .field("cap0_edge", &self.cap0_edge())
            .field("cap1_edge", &self.cap1_edge())
            .field("cap2_edge", &self.cap2_edge())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_STATUS_SPEC;
impl crate::RegisterSpec for CAP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_status::R`](R) reader structure"]
impl crate::Readable for CAP_STATUS_SPEC {}
#[doc = "`reset()` method sets CAP_STATUS to value 0"]
impl crate::Resettable for CAP_STATUS_SPEC {}
