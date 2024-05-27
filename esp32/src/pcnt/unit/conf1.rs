#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `CNT_THRES0` reader - This register is used to configure thres0 value for unit0."]
pub type CNT_THRES0_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_THRES0` writer - This register is used to configure thres0 value for unit0."]
pub type CNT_THRES0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_THRES1` reader - This register is used to configure thres1 value for unit0."]
pub type CNT_THRES1_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_THRES1` writer - This register is used to configure thres1 value for unit0."]
pub type CNT_THRES1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres0(&self) -> CNT_THRES0_R {
        CNT_THRES0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit0."]
    #[inline(always)]
    pub fn cnt_thres1(&self) -> CNT_THRES1_R {
        CNT_THRES1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("cnt_thres0", &self.cnt_thres0())
            .field("cnt_thres1", &self.cnt_thres1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thres0(&mut self) -> CNT_THRES0_W<CONF1_SPEC> {
        CNT_THRES0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thres1(&mut self) -> CNT_THRES1_W<CONF1_SPEC> {
        CNT_THRES1_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
