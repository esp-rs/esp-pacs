#[doc = "Register `RDCLR0` reader"]
pub struct R(crate::R<RDCLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDCLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDCLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDCLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDCLR0` writer"]
pub struct W(crate::W<RDCLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDCLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RDCLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDCLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_SLC0_BIT7_CLRADDR` reader - *******Description***********"]
pub type SLCHOST_SLC0_BIT7_CLRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLCHOST_SLC0_BIT7_CLRADDR` writer - *******Description***********"]
pub type SLCHOST_SLC0_BIT7_CLRADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, RDCLR0_SPEC, 9, O, u16, u16>;
#[doc = "Field `SLCHOST_SLC0_BIT6_CLRADDR` reader - *******Description***********"]
pub type SLCHOST_SLC0_BIT6_CLRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLCHOST_SLC0_BIT6_CLRADDR` writer - *******Description***********"]
pub type SLCHOST_SLC0_BIT6_CLRADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, RDCLR0_SPEC, 9, O, u16, u16>;
impl R {
    #[doc = "Bits 0:8 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_slc0_bit7_clraddr(&self) -> SLCHOST_SLC0_BIT7_CLRADDR_R {
        SLCHOST_SLC0_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_slc0_bit6_clraddr(&self) -> SLCHOST_SLC0_BIT6_CLRADDR_R {
        SLCHOST_SLC0_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDCLR0")
            .field(
                "slchost_slc0_bit7_clraddr",
                &format_args!("{}", self.slchost_slc0_bit7_clraddr().bits()),
            )
            .field(
                "slchost_slc0_bit6_clraddr",
                &format_args!("{}", self.slchost_slc0_bit6_clraddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RDCLR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_slc0_bit7_clraddr(&mut self) -> SLCHOST_SLC0_BIT7_CLRADDR_W<0> {
        SLCHOST_SLC0_BIT7_CLRADDR_W::new(self)
    }
    #[doc = "Bits 9:17 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_slc0_bit6_clraddr(&mut self) -> SLCHOST_SLC0_BIT6_CLRADDR_W<9> {
        SLCHOST_SLC0_BIT6_CLRADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdclr0](index.html) module"]
pub struct RDCLR0_SPEC;
impl crate::RegisterSpec for RDCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdclr0::R](R) reader structure"]
impl crate::Readable for RDCLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdclr0::W](W) writer structure"]
impl crate::Writable for RDCLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDCLR0 to value 0x0003_c044"]
impl crate::Resettable for RDCLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_c044;
}
