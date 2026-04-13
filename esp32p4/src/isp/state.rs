#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `TAIL_BUSY` reader - Represents isp_tail state"]
pub type TAIL_BUSY_R = crate::BitReader;
#[doc = "Field `HEADER_BUSY` reader - Represents isp_header state"]
pub type HEADER_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents isp_tail state"]
    #[inline(always)]
    pub fn tail_busy(&self) -> TAIL_BUSY_R {
        TAIL_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents isp_header state"]
    #[inline(always)]
    pub fn header_busy(&self) -> HEADER_BUSY_R {
        HEADER_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("tail_busy", &self.tail_busy())
            .field("header_busy", &self.header_busy())
            .finish()
    }
}
#[doc = "awb window register in y-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {}
