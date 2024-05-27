#[doc = "Register `CFG0` reader"]
pub type R = crate::R<CFG0_SPEC>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<CFG0_SPEC>;
#[doc = "Field `CH1_SRC_MULTBLK_TYPE` reader - NA"]
pub type CH1_SRC_MULTBLK_TYPE_R = crate::FieldReader;
#[doc = "Field `CH1_SRC_MULTBLK_TYPE` writer - NA"]
pub type CH1_SRC_MULTBLK_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_DST_MULTBLK_TYPE` reader - NA"]
pub type CH1_DST_MULTBLK_TYPE_R = crate::FieldReader;
#[doc = "Field `CH1_DST_MULTBLK_TYPE` writer - NA"]
pub type CH1_DST_MULTBLK_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_RD_UID` reader - NA"]
pub type CH1_RD_UID_R = crate::FieldReader;
#[doc = "Field `CH1_WR_UID` reader - NA"]
pub type CH1_WR_UID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn ch1_src_multblk_type(&self) -> CH1_SRC_MULTBLK_TYPE_R {
        CH1_SRC_MULTBLK_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn ch1_dst_multblk_type(&self) -> CH1_DST_MULTBLK_TYPE_R {
        CH1_DST_MULTBLK_TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 18:21 - NA"]
    #[inline(always)]
    pub fn ch1_rd_uid(&self) -> CH1_RD_UID_R {
        CH1_RD_UID_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 25:28 - NA"]
    #[inline(always)]
    pub fn ch1_wr_uid(&self) -> CH1_WR_UID_R {
        CH1_WR_UID_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG0")
            .field("ch1_src_multblk_type", &self.ch1_src_multblk_type())
            .field("ch1_dst_multblk_type", &self.ch1_dst_multblk_type())
            .field("ch1_rd_uid", &self.ch1_rd_uid())
            .field("ch1_wr_uid", &self.ch1_wr_uid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_src_multblk_type(&mut self) -> CH1_SRC_MULTBLK_TYPE_W<CFG0_SPEC> {
        CH1_SRC_MULTBLK_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dst_multblk_type(&mut self) -> CH1_DST_MULTBLK_TYPE_W<CFG0_SPEC> {
        CH1_DST_MULTBLK_TYPE_W::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
