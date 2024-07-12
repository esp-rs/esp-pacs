#[doc = "Register `SWHSDST0` reader"]
pub type R = crate::R<SWHSDST0_SPEC>;
#[doc = "Register `SWHSDST0` writer"]
pub type W = crate::W<SWHSDST0_SPEC>;
#[doc = "Field `CH1_SWHS_REQ_DST` reader - NA"]
pub type CH1_SWHS_REQ_DST_R = crate::BitReader;
#[doc = "Field `CH1_SWHS_REQ_DST` writer - NA"]
pub type CH1_SWHS_REQ_DST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_REQ_DST_WE` writer - NA"]
pub type CH1_SWHS_REQ_DST_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_SGLREQ_DST` reader - NA"]
pub type CH1_SWHS_SGLREQ_DST_R = crate::BitReader;
#[doc = "Field `CH1_SWHS_SGLREQ_DST` writer - NA"]
pub type CH1_SWHS_SGLREQ_DST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_SGLREQ_DST_WE` writer - NA"]
pub type CH1_SWHS_SGLREQ_DST_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_LST_DST` reader - NA"]
pub type CH1_SWHS_LST_DST_R = crate::BitReader;
#[doc = "Field `CH1_SWHS_LST_DST` writer - NA"]
pub type CH1_SWHS_LST_DST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SWHS_LST_DST_WE` writer - NA"]
pub type CH1_SWHS_LST_DST_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_req_dst(&self) -> CH1_SWHS_REQ_DST_R {
        CH1_SWHS_REQ_DST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_sglreq_dst(&self) -> CH1_SWHS_SGLREQ_DST_R {
        CH1_SWHS_SGLREQ_DST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_swhs_lst_dst(&self) -> CH1_SWHS_LST_DST_R {
        CH1_SWHS_LST_DST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWHSDST0")
            .field("ch1_swhs_req_dst", &self.ch1_swhs_req_dst())
            .field("ch1_swhs_sglreq_dst", &self.ch1_swhs_sglreq_dst())
            .field("ch1_swhs_lst_dst", &self.ch1_swhs_lst_dst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_swhs_req_dst(&mut self) -> CH1_SWHS_REQ_DST_W<SWHSDST0_SPEC> {
        CH1_SWHS_REQ_DST_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_swhs_req_dst_we(&mut self) -> CH1_SWHS_REQ_DST_WE_W<SWHSDST0_SPEC> {
        CH1_SWHS_REQ_DST_WE_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_swhs_sglreq_dst(&mut self) -> CH1_SWHS_SGLREQ_DST_W<SWHSDST0_SPEC> {
        CH1_SWHS_SGLREQ_DST_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_swhs_sglreq_dst_we(&mut self) -> CH1_SWHS_SGLREQ_DST_WE_W<SWHSDST0_SPEC> {
        CH1_SWHS_SGLREQ_DST_WE_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_swhs_lst_dst(&mut self) -> CH1_SWHS_LST_DST_W<SWHSDST0_SPEC> {
        CH1_SWHS_LST_DST_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_swhs_lst_dst_we(&mut self) -> CH1_SWHS_LST_DST_WE_W<SWHSDST0_SPEC> {
        CH1_SWHS_LST_DST_WE_W::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`swhsdst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swhsdst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWHSDST0_SPEC;
impl crate::RegisterSpec for SWHSDST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swhsdst0::R`](R) reader structure"]
impl crate::Readable for SWHSDST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swhsdst0::W`](W) writer structure"]
impl crate::Writable for SWHSDST0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWHSDST0 to value 0"]
impl crate::Resettable for SWHSDST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
