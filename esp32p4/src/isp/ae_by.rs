#[doc = "Register `AE_BY` reader"]
pub type R = crate::R<AE_BY_SPEC>;
#[doc = "Register `AE_BY` writer"]
pub type W = crate::W<AE_BY_SPEC>;
#[doc = "Field `AE_Y_BSIZE` reader - this field configures every block y size"]
pub type AE_Y_BSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `AE_Y_BSIZE` writer - this field configures every block y size"]
pub type AE_Y_BSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AE_Y_START` reader - this field configures first block start y address"]
pub type AE_Y_START_R = crate::FieldReader<u16>;
#[doc = "Field `AE_Y_START` writer - this field configures first block start y address"]
pub type AE_Y_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - this field configures every block y size"]
    #[inline(always)]
    pub fn ae_y_bsize(&self) -> AE_Y_BSIZE_R {
        AE_Y_BSIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - this field configures first block start y address"]
    #[inline(always)]
    pub fn ae_y_start(&self) -> AE_Y_START_R {
        AE_Y_START_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BY")
            .field("ae_y_bsize", &format_args!("{}", self.ae_y_bsize().bits()))
            .field("ae_y_start", &format_args!("{}", self.ae_y_start().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AE_BY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10 - this field configures every block y size"]
    #[inline(always)]
    #[must_use]
    pub fn ae_y_bsize(&mut self) -> AE_Y_BSIZE_W<AE_BY_SPEC> {
        AE_Y_BSIZE_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - this field configures first block start y address"]
    #[inline(always)]
    #[must_use]
    pub fn ae_y_start(&mut self) -> AE_Y_START_W<AE_BY_SPEC> {
        AE_Y_START_W::new(self, 11)
    }
}
#[doc = "ae window register in y-direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ae_by::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_by::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_BY_SPEC;
impl crate::RegisterSpec for AE_BY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_by::R`](R) reader structure"]
impl crate::Readable for AE_BY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ae_by::W`](W) writer structure"]
impl crate::Writable for AE_BY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AE_BY to value 0xd8"]
impl crate::Resettable for AE_BY_SPEC {
    const RESET_VALUE: u32 = 0xd8;
}
