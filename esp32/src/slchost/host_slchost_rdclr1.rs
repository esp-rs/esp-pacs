#[doc = "Register `HOST_SLCHOST_RDCLR1` reader"]
pub struct R(crate::R<HOST_SLCHOST_RDCLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_RDCLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_RDCLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_RDCLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_RDCLR1` writer"]
pub struct W(crate::W<HOST_SLCHOST_RDCLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_RDCLR1_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_RDCLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_RDCLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_SLC1_BIT7_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC1_BIT7_CLRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HOST_SLCHOST_SLC1_BIT7_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC1_BIT7_CLRADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_RDCLR1_SPEC, 9, O, u16, u16>;
#[doc = "Field `HOST_SLCHOST_SLC1_BIT6_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC1_BIT6_CLRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HOST_SLCHOST_SLC1_BIT6_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC1_BIT6_CLRADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_RDCLR1_SPEC, 9, O, u16, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc1_bit7_clraddr(&self) -> HOST_SLCHOST_SLC1_BIT7_CLRADDR_R {
        HOST_SLCHOST_SLC1_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc1_bit6_clraddr(&self) -> HOST_SLCHOST_SLC1_BIT6_CLRADDR_R {
        HOST_SLCHOST_SLC1_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_RDCLR1")
            .field(
                "host_slchost_slc1_bit7_clraddr",
                &format_args!("{}", self.host_slchost_slc1_bit7_clraddr().bits()),
            )
            .field(
                "host_slchost_slc1_bit6_clraddr",
                &format_args!("{}", self.host_slchost_slc1_bit6_clraddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_RDCLR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_slc1_bit7_clraddr(&mut self) -> HOST_SLCHOST_SLC1_BIT7_CLRADDR_W<0> {
        HOST_SLCHOST_SLC1_BIT7_CLRADDR_W::new(self)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_slc1_bit6_clraddr(&mut self) -> HOST_SLCHOST_SLC1_BIT6_CLRADDR_W<9> {
        HOST_SLCHOST_SLC1_BIT6_CLRADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_rdclr1](index.html) module"]
pub struct HOST_SLCHOST_RDCLR1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_RDCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_rdclr1::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_rdclr1::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_RDCLR1 to value 0x0003_c1e0"]
impl crate::Resettable for HOST_SLCHOST_RDCLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_c1e0;
}
