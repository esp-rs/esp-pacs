#[doc = "Register `CH2_CFG0` reader"]
pub type R = crate::R<CH2_CFG0_SPEC>;
#[doc = "Register `CH2_CFG0` writer"]
pub type W = crate::W<CH2_CFG0_SPEC>;
#[doc = "Field `CH2_SRC_MULTBLK_TYPE` reader - NA"]
pub type CH2_SRC_MULTBLK_TYPE_R = crate::FieldReader;
#[doc = "Field `CH2_SRC_MULTBLK_TYPE` writer - NA"]
pub type CH2_SRC_MULTBLK_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_DST_MULTBLK_TYPE` reader - NA"]
pub type CH2_DST_MULTBLK_TYPE_R = crate::FieldReader;
#[doc = "Field `CH2_DST_MULTBLK_TYPE` writer - NA"]
pub type CH2_DST_MULTBLK_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_RD_UID` reader - NA"]
pub type CH2_RD_UID_R = crate::FieldReader;
#[doc = "Field `CH2_WR_UID` reader - NA"]
pub type CH2_WR_UID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn ch2_src_multblk_type(&self) -> CH2_SRC_MULTBLK_TYPE_R {
        CH2_SRC_MULTBLK_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn ch2_dst_multblk_type(&self) -> CH2_DST_MULTBLK_TYPE_R {
        CH2_DST_MULTBLK_TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 18:21 - NA"]
    #[inline(always)]
    pub fn ch2_rd_uid(&self) -> CH2_RD_UID_R {
        CH2_RD_UID_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 25:28 - NA"]
    #[inline(always)]
    pub fn ch2_wr_uid(&self) -> CH2_WR_UID_R {
        CH2_WR_UID_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_CFG0")
            .field(
                "ch2_src_multblk_type",
                &format_args!("{}", self.ch2_src_multblk_type().bits()),
            )
            .field(
                "ch2_dst_multblk_type",
                &format_args!("{}", self.ch2_dst_multblk_type().bits()),
            )
            .field("ch2_rd_uid", &format_args!("{}", self.ch2_rd_uid().bits()))
            .field("ch2_wr_uid", &format_args!("{}", self.ch2_wr_uid().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_src_multblk_type(&mut self) -> CH2_SRC_MULTBLK_TYPE_W<CH2_CFG0_SPEC> {
        CH2_SRC_MULTBLK_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_dst_multblk_type(&mut self) -> CH2_DST_MULTBLK_TYPE_W<CH2_CFG0_SPEC> {
        CH2_DST_MULTBLK_TYPE_W::new(self, 2)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_CFG0_SPEC;
impl crate::RegisterSpec for CH2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_cfg0::R`](R) reader structure"]
impl crate::Readable for CH2_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_cfg0::W`](W) writer structure"]
impl crate::Writable for CH2_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_CFG0 to value 0"]
impl crate::Resettable for CH2_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
