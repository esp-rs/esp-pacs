#[doc = "Register `SAR_MEM_WR_CTRL` reader"]
pub struct R(crate::R<SAR_MEM_WR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEM_WR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEM_WR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEM_WR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEM_WR_CTRL` writer"]
pub struct W(crate::W<SAR_MEM_WR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEM_WR_CTRL_SPEC>;
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
impl From<crate::W<SAR_MEM_WR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEM_WR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_WR_ADDR_INIT` reader - "]
pub type MEM_WR_ADDR_INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEM_WR_ADDR_INIT` writer - "]
pub type MEM_WR_ADDR_INIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_MEM_WR_CTRL_SPEC, 11, O, u16, u16>;
#[doc = "Field `MEM_WR_ADDR_SIZE` reader - "]
pub type MEM_WR_ADDR_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEM_WR_ADDR_SIZE` writer - "]
pub type MEM_WR_ADDR_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_MEM_WR_CTRL_SPEC, 11, O, u16, u16>;
#[doc = "Field `RTC_MEM_WR_OFFST_CLR` writer - "]
pub type RTC_MEM_WR_OFFST_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SAR_MEM_WR_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mem_wr_addr_init(&self) -> MEM_WR_ADDR_INIT_R {
        MEM_WR_ADDR_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn mem_wr_addr_size(&self) -> MEM_WR_ADDR_SIZE_R {
        MEM_WR_ADDR_SIZE_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEM_WR_CTRL")
            .field(
                "mem_wr_addr_init",
                &format_args!("{}", self.mem_wr_addr_init().bits()),
            )
            .field(
                "mem_wr_addr_size",
                &format_args!("{}", self.mem_wr_addr_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEM_WR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn mem_wr_addr_init(&mut self) -> MEM_WR_ADDR_INIT_W<0> {
        MEM_WR_ADDR_INIT_W::new(self)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    #[must_use]
    pub fn mem_wr_addr_size(&mut self) -> MEM_WR_ADDR_SIZE_W<11> {
        MEM_WR_ADDR_SIZE_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_wr_offst_clr(&mut self) -> RTC_MEM_WR_OFFST_CLR_W<22> {
        RTC_MEM_WR_OFFST_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_mem_wr_ctrl](index.html) module"]
pub struct SAR_MEM_WR_CTRL_SPEC;
impl crate::RegisterSpec for SAR_MEM_WR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_mem_wr_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_MEM_WR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_mem_wr_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_MEM_WR_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEM_WR_CTRL to value 0x0010_0200"]
impl crate::Resettable for SAR_MEM_WR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0200;
}
