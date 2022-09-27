#[doc = "Register `PRO_CPU_RECORD_CTRL` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_CTRL` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_CTRL_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CPU_RECORD_ENABLE` reader - "]
pub type PRO_CPU_RECORD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PRO_CPU_RECORD_ENABLE` writer - "]
pub type PRO_CPU_RECORD_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_CPU_RECORD_CTRL_SPEC, bool, O>;
#[doc = "Field `PRO_CPU_RECORD_DISABLE` reader - "]
pub type PRO_CPU_RECORD_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `PRO_CPU_RECORD_DISABLE` writer - "]
pub type PRO_CPU_RECORD_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_CPU_RECORD_CTRL_SPEC, bool, O>;
#[doc = "Field `PRO_CPU_PDEBUG_ENABLE` reader - "]
pub type PRO_CPU_PDEBUG_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PRO_CPU_PDEBUG_ENABLE` writer - "]
pub type PRO_CPU_PDEBUG_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_CPU_RECORD_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cpu_record_enable(&self) -> PRO_CPU_RECORD_ENABLE_R {
        PRO_CPU_RECORD_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pro_cpu_record_disable(&self) -> PRO_CPU_RECORD_DISABLE_R {
        PRO_CPU_RECORD_DISABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pro_cpu_pdebug_enable(&self) -> PRO_CPU_PDEBUG_ENABLE_R {
        PRO_CPU_PDEBUG_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cpu_record_enable(&mut self) -> PRO_CPU_RECORD_ENABLE_W<0> {
        PRO_CPU_RECORD_ENABLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pro_cpu_record_disable(&mut self) -> PRO_CPU_RECORD_DISABLE_W<4> {
        PRO_CPU_RECORD_DISABLE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pro_cpu_pdebug_enable(&mut self) -> PRO_CPU_PDEBUG_ENABLE_W<8> {
        PRO_CPU_PDEBUG_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_ctrl](index.html) module"]
pub struct PRO_CPU_RECORD_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_ctrl::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_ctrl::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_CTRL to value 0x0100"]
impl crate::Resettable for PRO_CPU_RECORD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
