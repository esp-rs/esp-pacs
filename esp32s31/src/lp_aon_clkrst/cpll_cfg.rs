#[doc = "Register `CPLL_CFG` reader"]
pub type R = crate::R<CPLL_CFG_SPEC>;
#[doc = "Register `CPLL_CFG` writer"]
pub type W = crate::W<CPLL_CFG_SPEC>;
#[doc = "Field `CPLL_DBIAS` reader - cpll dbias value"]
pub type CPLL_DBIAS_R = crate::FieldReader;
#[doc = "Field `CPLL_DBIAS` writer - cpll dbias value"]
pub type CPLL_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPLL_DCHGP` reader - cpll dchgp value"]
pub type CPLL_DCHGP_R = crate::FieldReader;
#[doc = "Field `CPLL_DCHGP` writer - cpll dchgp value"]
pub type CPLL_DCHGP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPLL_DR1` reader - cpll dr1 value"]
pub type CPLL_DR1_R = crate::FieldReader;
#[doc = "Field `CPLL_DR1` writer - cpll dr1 value"]
pub type CPLL_DR1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPLL_DR3` reader - cpll dr3 value"]
pub type CPLL_DR3_R = crate::FieldReader;
#[doc = "Field `CPLL_DR3` writer - cpll dr3 value"]
pub type CPLL_DR3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - cpll dbias value"]
    #[inline(always)]
    pub fn cpll_dbias(&self) -> CPLL_DBIAS_R {
        CPLL_DBIAS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - cpll dchgp value"]
    #[inline(always)]
    pub fn cpll_dchgp(&self) -> CPLL_DCHGP_R {
        CPLL_DCHGP_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - cpll dr1 value"]
    #[inline(always)]
    pub fn cpll_dr1(&self) -> CPLL_DR1_R {
        CPLL_DR1_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - cpll dr3 value"]
    #[inline(always)]
    pub fn cpll_dr3(&self) -> CPLL_DR3_R {
        CPLL_DR3_R::new(((self.bits >> 9) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPLL_CFG")
            .field("cpll_dbias", &self.cpll_dbias())
            .field("cpll_dchgp", &self.cpll_dchgp())
            .field("cpll_dr1", &self.cpll_dr1())
            .field("cpll_dr3", &self.cpll_dr3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - cpll dbias value"]
    #[inline(always)]
    pub fn cpll_dbias(&mut self) -> CPLL_DBIAS_W<'_, CPLL_CFG_SPEC> {
        CPLL_DBIAS_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - cpll dchgp value"]
    #[inline(always)]
    pub fn cpll_dchgp(&mut self) -> CPLL_DCHGP_W<'_, CPLL_CFG_SPEC> {
        CPLL_DCHGP_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - cpll dr1 value"]
    #[inline(always)]
    pub fn cpll_dr1(&mut self) -> CPLL_DR1_W<'_, CPLL_CFG_SPEC> {
        CPLL_DR1_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - cpll dr3 value"]
    #[inline(always)]
    pub fn cpll_dr3(&mut self) -> CPLL_DR3_W<'_, CPLL_CFG_SPEC> {
        CPLL_DR3_W::new(self, 9)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpll_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpll_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPLL_CFG_SPEC;
impl crate::RegisterSpec for CPLL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpll_cfg::R`](R) reader structure"]
impl crate::Readable for CPLL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpll_cfg::W`](W) writer structure"]
impl crate::Writable for CPLL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPLL_CFG to value 0"]
impl crate::Resettable for CPLL_CFG_SPEC {}
