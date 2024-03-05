#[doc = "Register `CH4_SWHSSRC0` reader"]
pub type R = crate::R<CH4_SWHSSRC0_SPEC>;
#[doc = "Register `CH4_SWHSSRC0` writer"]
pub type W = crate::W<CH4_SWHSSRC0_SPEC>;
#[doc = "Field `CH4_SWHS_REQ_SRC` reader - NA"]
pub type CH4_SWHS_REQ_SRC_R = crate::BitReader;
#[doc = "Field `CH4_SWHS_REQ_SRC` writer - NA"]
pub type CH4_SWHS_REQ_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SWHS_REQ_SRC_WE` writer - NA"]
pub type CH4_SWHS_REQ_SRC_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SWHS_SGLREQ_SRC` reader - NA"]
pub type CH4_SWHS_SGLREQ_SRC_R = crate::BitReader;
#[doc = "Field `CH4_SWHS_SGLREQ_SRC` writer - NA"]
pub type CH4_SWHS_SGLREQ_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SWHS_SGLREQ_SRC_WE` writer - NA"]
pub type CH4_SWHS_SGLREQ_SRC_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SWHS_LST_SRC` reader - NA"]
pub type CH4_SWHS_LST_SRC_R = crate::BitReader;
#[doc = "Field `CH4_SWHS_LST_SRC` writer - NA"]
pub type CH4_SWHS_LST_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_SWHS_LST_SRC_WE` writer - NA"]
pub type CH4_SWHS_LST_SRC_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch4_swhs_req_src(&self) -> CH4_SWHS_REQ_SRC_R {
        CH4_SWHS_REQ_SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch4_swhs_sglreq_src(&self) -> CH4_SWHS_SGLREQ_SRC_R {
        CH4_SWHS_SGLREQ_SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch4_swhs_lst_src(&self) -> CH4_SWHS_LST_SRC_R {
        CH4_SWHS_LST_SRC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_SWHSSRC0")
            .field(
                "ch4_swhs_req_src",
                &format_args!("{}", self.ch4_swhs_req_src().bit()),
            )
            .field(
                "ch4_swhs_sglreq_src",
                &format_args!("{}", self.ch4_swhs_sglreq_src().bit()),
            )
            .field(
                "ch4_swhs_lst_src",
                &format_args!("{}", self.ch4_swhs_lst_src().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_SWHSSRC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_swhs_req_src(&mut self) -> CH4_SWHS_REQ_SRC_W<CH4_SWHSSRC0_SPEC> {
        CH4_SWHS_REQ_SRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_swhs_req_src_we(&mut self) -> CH4_SWHS_REQ_SRC_WE_W<CH4_SWHSSRC0_SPEC> {
        CH4_SWHS_REQ_SRC_WE_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_swhs_sglreq_src(&mut self) -> CH4_SWHS_SGLREQ_SRC_W<CH4_SWHSSRC0_SPEC> {
        CH4_SWHS_SGLREQ_SRC_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_swhs_sglreq_src_we(&mut self) -> CH4_SWHS_SGLREQ_SRC_WE_W<CH4_SWHSSRC0_SPEC> {
        CH4_SWHS_SGLREQ_SRC_WE_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_swhs_lst_src(&mut self) -> CH4_SWHS_LST_SRC_W<CH4_SWHSSRC0_SPEC> {
        CH4_SWHS_LST_SRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_swhs_lst_src_we(&mut self) -> CH4_SWHS_LST_SRC_WE_W<CH4_SWHSSRC0_SPEC> {
        CH4_SWHS_LST_SRC_WE_W::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_swhssrc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_swhssrc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_SWHSSRC0_SPEC;
impl crate::RegisterSpec for CH4_SWHSSRC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_swhssrc0::R`](R) reader structure"]
impl crate::Readable for CH4_SWHSSRC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4_swhssrc0::W`](W) writer structure"]
impl crate::Writable for CH4_SWHSSRC0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_SWHSSRC0 to value 0"]
impl crate::Resettable for CH4_SWHSSRC0_SPEC {
    const RESET_VALUE: u32 = 0;
}
