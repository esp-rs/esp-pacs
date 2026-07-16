#[doc = "Register `DEC_FAILURE_INFO` reader"]
pub type R = crate::R<DEC_FAILURE_INFO_SPEC>;
#[doc = "Register `DEC_FAILURE_INFO` writer"]
pub type W = crate::W<DEC_FAILURE_INFO_SPEC>;
#[doc = "Field `REG_DEC_FAILURE_READ_FIRST` reader - "]
pub type REG_DEC_FAILURE_READ_FIRST_R = crate::BitReader;
#[doc = "Field `REG_DEC_FAILURE_READ_FIRST` writer - "]
pub type REG_DEC_FAILURE_READ_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DEC_FAILURE_VALID` reader - "]
pub type REG_DEC_FAILURE_VALID_R = crate::BitReader;
#[doc = "Field `REG_DEC_FAILURE_TYPE` reader - "]
pub type REG_DEC_FAILURE_TYPE_R = crate::BitReader;
#[doc = "Field `REG_DEC_FAILURE_CAUSE` reader - "]
pub type REG_DEC_FAILURE_CAUSE_R = crate::FieldReader;
#[doc = "Field `REG_DEC_FAILURE_MST_ID` reader - "]
pub type REG_DEC_FAILURE_MST_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_read_first(&self) -> REG_DEC_FAILURE_READ_FIRST_R {
        REG_DEC_FAILURE_READ_FIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_dec_failure_valid(&self) -> REG_DEC_FAILURE_VALID_R {
        REG_DEC_FAILURE_VALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_dec_failure_type(&self) -> REG_DEC_FAILURE_TYPE_R {
        REG_DEC_FAILURE_TYPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn reg_dec_failure_cause(&self) -> REG_DEC_FAILURE_CAUSE_R {
        REG_DEC_FAILURE_CAUSE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:12"]
    #[inline(always)]
    pub fn reg_dec_failure_mst_id(&self) -> REG_DEC_FAILURE_MST_ID_R {
        REG_DEC_FAILURE_MST_ID_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEC_FAILURE_INFO")
            .field(
                "reg_dec_failure_read_first",
                &self.reg_dec_failure_read_first(),
            )
            .field("reg_dec_failure_valid", &self.reg_dec_failure_valid())
            .field("reg_dec_failure_type", &self.reg_dec_failure_type())
            .field("reg_dec_failure_cause", &self.reg_dec_failure_cause())
            .field("reg_dec_failure_mst_id", &self.reg_dec_failure_mst_id())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_read_first(
        &mut self,
    ) -> REG_DEC_FAILURE_READ_FIRST_W<'_, DEC_FAILURE_INFO_SPEC> {
        REG_DEC_FAILURE_READ_FIRST_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dec_failure_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dec_failure_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEC_FAILURE_INFO_SPEC;
impl crate::RegisterSpec for DEC_FAILURE_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dec_failure_info::R`](R) reader structure"]
impl crate::Readable for DEC_FAILURE_INFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dec_failure_info::W`](W) writer structure"]
impl crate::Writable for DEC_FAILURE_INFO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEC_FAILURE_INFO to value 0"]
impl crate::Resettable for DEC_FAILURE_INFO_SPEC {}
