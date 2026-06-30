#[doc = "Register `CFG0` reader"]
pub type R = crate::R<CFG0_SPEC>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<CFG0_SPEC>;
#[doc = "Field `DIV_CYCLE` reader - need_des"]
pub type DIV_CYCLE_R = crate::FieldReader;
#[doc = "Field `DIV_CYCLE` writer - need_des"]
pub type DIV_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FULL_CNT_DONE` reader - need_des"]
pub type FULL_CNT_DONE_R = crate::BitReader;
#[doc = "Field `DIV_CALI_CNT` reader - need_des"]
pub type DIV_CALI_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_NUMERATOR_TYPE` reader - need_des"]
pub type DIV_NUMERATOR_TYPE_R = crate::BitReader;
#[doc = "Field `DIV_NUM` reader - need_des"]
pub type DIV_NUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn div_cycle(&self) -> DIV_CYCLE_R {
        DIV_CYCLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn full_cnt_done(&self) -> FULL_CNT_DONE_R {
        FULL_CNT_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:24 - need_des"]
    #[inline(always)]
    pub fn div_cali_cnt(&self) -> DIV_CALI_CNT_R {
        DIV_CALI_CNT_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn div_numerator_type(&self) -> DIV_NUMERATOR_TYPE_R {
        DIV_NUMERATOR_TYPE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn div_num(&self) -> DIV_NUM_R {
        DIV_NUM_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG0")
            .field("div_cycle", &self.div_cycle())
            .field("full_cnt_done", &self.full_cnt_done())
            .field("div_cali_cnt", &self.div_cali_cnt())
            .field("div_numerator_type", &self.div_numerator_type())
            .field("div_num", &self.div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn div_cycle(&mut self) -> DIV_CYCLE_W<'_, CFG0_SPEC> {
        DIV_CYCLE_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0x01"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
