#[doc = "Register `CALI0` reader"]
pub type R = crate::R<CALI0_SPEC>;
#[doc = "Register `CALI0` writer"]
pub type W = crate::W<CALI0_SPEC>;
#[doc = "Field `LP_CALI_DIV_CYCLE` reader - need_des"]
pub type LP_CALI_DIV_CYCLE_R = crate::FieldReader;
#[doc = "Field `LP_CALI_DIV_CYCLE` writer - need_des"]
pub type LP_CALI_DIV_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_CALI_FULL_CNT_DONE` reader - need_des"]
pub type LP_CALI_FULL_CNT_DONE_R = crate::BitReader;
#[doc = "Field `LP_CALI_DIV_CALI_CNT` reader - need_des"]
pub type LP_CALI_DIV_CALI_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `LP_CALI_DIV_NUMERATOR_TYPE` reader - need_des"]
pub type LP_CALI_DIV_NUMERATOR_TYPE_R = crate::BitReader;
#[doc = "Field `LP_CALI_DIV_NUM` reader - need_des"]
pub type LP_CALI_DIV_NUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_cali_div_cycle(&self) -> LP_CALI_DIV_CYCLE_R {
        LP_CALI_DIV_CYCLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn lp_cali_full_cnt_done(&self) -> LP_CALI_FULL_CNT_DONE_R {
        LP_CALI_FULL_CNT_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:24 - need_des"]
    #[inline(always)]
    pub fn lp_cali_div_cali_cnt(&self) -> LP_CALI_DIV_CALI_CNT_R {
        LP_CALI_DIV_CALI_CNT_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_cali_div_numerator_type(&self) -> LP_CALI_DIV_NUMERATOR_TYPE_R {
        LP_CALI_DIV_NUMERATOR_TYPE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn lp_cali_div_num(&self) -> LP_CALI_DIV_NUM_R {
        LP_CALI_DIV_NUM_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI0")
            .field("lp_cali_div_cycle", &self.lp_cali_div_cycle())
            .field("lp_cali_full_cnt_done", &self.lp_cali_full_cnt_done())
            .field("lp_cali_div_cali_cnt", &self.lp_cali_div_cali_cnt())
            .field(
                "lp_cali_div_numerator_type",
                &self.lp_cali_div_numerator_type(),
            )
            .field("lp_cali_div_num", &self.lp_cali_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_cali_div_cycle(&mut self) -> LP_CALI_DIV_CYCLE_W<'_, CALI0_SPEC> {
        LP_CALI_DIV_CYCLE_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cali0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI0_SPEC;
impl crate::RegisterSpec for CALI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali0::R`](R) reader structure"]
impl crate::Readable for CALI0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cali0::W`](W) writer structure"]
impl crate::Writable for CALI0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALI0 to value 0x01"]
impl crate::Resettable for CALI0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
