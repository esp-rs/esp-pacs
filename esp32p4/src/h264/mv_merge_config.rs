#[doc = "Register `MV_MERGE_CONFIG` reader"]
pub type R = crate::R<MV_MERGE_CONFIG_SPEC>;
#[doc = "Register `MV_MERGE_CONFIG` writer"]
pub type W = crate::W<MV_MERGE_CONFIG_SPEC>;
#[doc = "Field `MV_MERGE_TYPE` reader - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
pub type MV_MERGE_TYPE_R = crate::FieldReader;
#[doc = "Field `MV_MERGE_TYPE` writer - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
pub type MV_MERGE_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_MV_OUT_EN` reader - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
pub type INT_MV_OUT_EN_R = crate::BitReader;
#[doc = "Field `INT_MV_OUT_EN` writer - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
pub type INT_MV_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_MV_MERGE_EN` reader - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
pub type A_MV_MERGE_EN_R = crate::BitReader;
#[doc = "Field `A_MV_MERGE_EN` writer - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
pub type A_MV_MERGE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_MV_MERGE_EN` reader - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
pub type B_MV_MERGE_EN_R = crate::BitReader;
#[doc = "Field `B_MV_MERGE_EN` writer - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
pub type B_MV_MERGE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB_VALID_NUM` reader - Represents the valid mb number of mv merge output."]
pub type MB_VALID_NUM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
    #[inline(always)]
    pub fn mv_merge_type(&self) -> MV_MERGE_TYPE_R {
        MV_MERGE_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
    #[inline(always)]
    pub fn int_mv_out_en(&self) -> INT_MV_OUT_EN_R {
        INT_MV_OUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    pub fn a_mv_merge_en(&self) -> A_MV_MERGE_EN_R {
        A_MV_MERGE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    pub fn b_mv_merge_en(&self) -> B_MV_MERGE_EN_R {
        B_MV_MERGE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:17 - Represents the valid mb number of mv merge output."]
    #[inline(always)]
    pub fn mb_valid_num(&self) -> MB_VALID_NUM_R {
        MB_VALID_NUM_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MV_MERGE_CONFIG")
            .field(
                "mv_merge_type",
                &format_args!("{}", self.mv_merge_type().bits()),
            )
            .field(
                "int_mv_out_en",
                &format_args!("{}", self.int_mv_out_en().bit()),
            )
            .field(
                "a_mv_merge_en",
                &format_args!("{}", self.a_mv_merge_en().bit()),
            )
            .field(
                "b_mv_merge_en",
                &format_args!("{}", self.b_mv_merge_en().bit()),
            )
            .field(
                "mb_valid_num",
                &format_args!("{}", self.mb_valid_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MV_MERGE_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure mv merge type.\\\\0: merge p16x16 mv\\\\1: merge min mv\\\\2: merge max mv\\\\3: not valid."]
    #[inline(always)]
    #[must_use]
    pub fn mv_merge_type(&mut self) -> MV_MERGE_TYPE_W<MV_MERGE_CONFIG_SPEC> {
        MV_MERGE_TYPE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configure mv merge output integer part not zero mv or all part not zero mv.\\\\0: output all part not zero mv\\\\1: output integer part not zero mv."]
    #[inline(always)]
    #[must_use]
    pub fn int_mv_out_en(&mut self) -> INT_MV_OUT_EN_W<MV_MERGE_CONFIG_SPEC> {
        INT_MV_OUT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configure whether or not to enable video A mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn a_mv_merge_en(&mut self) -> A_MV_MERGE_EN_W<MV_MERGE_CONFIG_SPEC> {
        A_MV_MERGE_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configure whether or not to enable video B mv merge.\\\\0: disable\\\\1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn b_mv_merge_en(&mut self) -> B_MV_MERGE_EN_W<MV_MERGE_CONFIG_SPEC> {
        B_MV_MERGE_EN_W::new(self, 4)
    }
}
#[doc = "Mv merge configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mv_merge_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mv_merge_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MV_MERGE_CONFIG_SPEC;
impl crate::RegisterSpec for MV_MERGE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mv_merge_config::R`](R) reader structure"]
impl crate::Readable for MV_MERGE_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mv_merge_config::W`](W) writer structure"]
impl crate::Writable for MV_MERGE_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MV_MERGE_CONFIG to value 0"]
impl crate::Resettable for MV_MERGE_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
