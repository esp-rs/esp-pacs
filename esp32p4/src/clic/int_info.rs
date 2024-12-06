#[doc = "Register `INT_INFO` reader"]
pub type R = crate::R<INT_INFO_SPEC>;
#[doc = "Register `INT_INFO` writer"]
pub type W = crate::W<INT_INFO_SPEC>;
#[doc = "Field `INT_INFO_NUM_INT` reader - Number of interrupts"]
pub type INT_INFO_NUM_INT_R = crate::FieldReader<u16>;
#[doc = "Field `INT_INFO_NUM_INT` writer - Number of interrupts"]
pub type INT_INFO_NUM_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `INT_INFO_VERSION` reader - Version of interrupt module"]
pub type INT_INFO_VERSION_R = crate::FieldReader;
#[doc = "Field `INT_INFO_VERSION` writer - Version of interrupt module"]
pub type INT_INFO_VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_INFO_CTLBITS` reader - Control bits for interrupt"]
pub type INT_INFO_CTLBITS_R = crate::FieldReader;
#[doc = "Field `INT_INFO_CTLBITS` writer - Control bits for interrupt"]
pub type INT_INFO_CTLBITS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:12 - Number of interrupts"]
    #[inline(always)]
    pub fn int_info_num_int(&self) -> INT_INFO_NUM_INT_R {
        INT_INFO_NUM_INT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - Version of interrupt module"]
    #[inline(always)]
    pub fn int_info_version(&self) -> INT_INFO_VERSION_R {
        INT_INFO_VERSION_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:24 - Control bits for interrupt"]
    #[inline(always)]
    pub fn int_info_ctlbits(&self) -> INT_INFO_CTLBITS_R {
        INT_INFO_CTLBITS_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_INFO")
            .field("int_info_ctlbits", &self.int_info_ctlbits())
            .field("int_info_version", &self.int_info_version())
            .field("int_info_num_int", &self.int_info_num_int())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - Number of interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn int_info_num_int(&mut self) -> INT_INFO_NUM_INT_W<INT_INFO_SPEC> {
        INT_INFO_NUM_INT_W::new(self, 0)
    }
    #[doc = "Bits 13:20 - Version of interrupt module"]
    #[inline(always)]
    #[must_use]
    pub fn int_info_version(&mut self) -> INT_INFO_VERSION_W<INT_INFO_SPEC> {
        INT_INFO_VERSION_W::new(self, 13)
    }
    #[doc = "Bits 21:24 - Control bits for interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn int_info_ctlbits(&mut self) -> INT_INFO_CTLBITS_W<INT_INFO_SPEC> {
        INT_INFO_CTLBITS_W::new(self, 21)
    }
}
#[doc = "Interrupt information register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_INFO_SPEC;
impl crate::RegisterSpec for INT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_info::R`](R) reader structure"]
impl crate::Readable for INT_INFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_info::W`](W) writer structure"]
impl crate::Writable for INT_INFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_INFO to value 0"]
impl crate::Resettable for INT_INFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
