#[doc = "Register `SLEEP_CONF1` reader"]
pub type R = crate::R<SLEEP_CONF1_SPEC>;
#[doc = "Register `SLEEP_CONF1` writer"]
pub type W = crate::W<SLEEP_CONF1_SPEC>;
#[doc = "Field `LP_REG_SLV_WK_CHAR1` reader - NA"]
pub type LP_REG_SLV_WK_CHAR1_R = crate::FieldReader;
#[doc = "Field `LP_REG_SLV_WK_CHAR1` writer - NA"]
pub type LP_REG_SLV_WK_CHAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_REG_SLV_WK_CHAR2` reader - NA"]
pub type LP_REG_SLV_WK_CHAR2_R = crate::FieldReader;
#[doc = "Field `LP_REG_SLV_WK_CHAR2` writer - NA"]
pub type LP_REG_SLV_WK_CHAR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_REG_SLV_WK_CHAR3` reader - NA"]
pub type LP_REG_SLV_WK_CHAR3_R = crate::FieldReader;
#[doc = "Field `LP_REG_SLV_WK_CHAR3` writer - NA"]
pub type LP_REG_SLV_WK_CHAR3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_REG_SLV_WK_CHAR4` reader - NA"]
pub type LP_REG_SLV_WK_CHAR4_R = crate::FieldReader;
#[doc = "Field `LP_REG_SLV_WK_CHAR4` writer - NA"]
pub type LP_REG_SLV_WK_CHAR4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char1(&self) -> LP_REG_SLV_WK_CHAR1_R {
        LP_REG_SLV_WK_CHAR1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char2(&self) -> LP_REG_SLV_WK_CHAR2_R {
        LP_REG_SLV_WK_CHAR2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char3(&self) -> LP_REG_SLV_WK_CHAR3_R {
        LP_REG_SLV_WK_CHAR3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char4(&self) -> LP_REG_SLV_WK_CHAR4_R {
        LP_REG_SLV_WK_CHAR4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF1")
            .field("lp_reg_slv_wk_char1", &self.lp_reg_slv_wk_char1())
            .field("lp_reg_slv_wk_char2", &self.lp_reg_slv_wk_char2())
            .field("lp_reg_slv_wk_char3", &self.lp_reg_slv_wk_char3())
            .field("lp_reg_slv_wk_char4", &self.lp_reg_slv_wk_char4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char1(&mut self) -> LP_REG_SLV_WK_CHAR1_W<'_, SLEEP_CONF1_SPEC> {
        LP_REG_SLV_WK_CHAR1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char2(&mut self) -> LP_REG_SLV_WK_CHAR2_W<'_, SLEEP_CONF1_SPEC> {
        LP_REG_SLV_WK_CHAR2_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char3(&mut self) -> LP_REG_SLV_WK_CHAR3_W<'_, SLEEP_CONF1_SPEC> {
        LP_REG_SLV_WK_CHAR3_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char4(&mut self) -> LP_REG_SLV_WK_CHAR4_W<'_, SLEEP_CONF1_SPEC> {
        LP_REG_SLV_WK_CHAR4_W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF1_SPEC;
impl crate::RegisterSpec for SLEEP_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf1::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf1::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF1 to value 0"]
impl crate::Resettable for SLEEP_CONF1_SPEC {}
