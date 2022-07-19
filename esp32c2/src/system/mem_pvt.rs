#[doc = "Register `MEM_PVT` reader"]
pub struct R(crate::R<MEM_PVT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_PVT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_PVT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_PVT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_PVT` writer"]
pub struct W(crate::W<MEM_PVT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_PVT_SPEC>;
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
impl From<crate::W<MEM_PVT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_PVT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_PATH_LEN` reader - reg_mem_path_len"]
pub type MEM_PATH_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_PATH_LEN` writer - reg_mem_path_len"]
pub type MEM_PATH_LEN_W<'a> = crate::FieldWriter<'a, u32, MEM_PVT_SPEC, u8, u8, 4, 0>;
#[doc = "Field `MEM_ERR_CNT_CLR` writer - reg_mem_err_cnt_clr"]
pub type MEM_ERR_CNT_CLR_W<'a> = crate::BitWriter<'a, u32, MEM_PVT_SPEC, bool, 4>;
#[doc = "Field `MONITOR_EN` reader - reg_mem_pvt_monitor_en"]
pub type MONITOR_EN_R = crate::BitReader<bool>;
#[doc = "Field `MONITOR_EN` writer - reg_mem_pvt_monitor_en"]
pub type MONITOR_EN_W<'a> = crate::BitWriter<'a, u32, MEM_PVT_SPEC, bool, 5>;
#[doc = "Field `MEM_TIMING_ERR_CNT` reader - reg_mem_timing_err_cnt"]
pub type MEM_TIMING_ERR_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEM_VT_SEL` reader - reg_mem_vt_sel"]
pub type MEM_VT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_VT_SEL` writer - reg_mem_vt_sel"]
pub type MEM_VT_SEL_W<'a> = crate::FieldWriter<'a, u32, MEM_PVT_SPEC, u8, u8, 2, 22>;
impl R {
    #[doc = "Bits 0:3 - reg_mem_path_len"]
    #[inline(always)]
    pub fn mem_path_len(&self) -> MEM_PATH_LEN_R {
        MEM_PATH_LEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - reg_mem_pvt_monitor_en"]
    #[inline(always)]
    pub fn monitor_en(&self) -> MONITOR_EN_R {
        MONITOR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:21 - reg_mem_timing_err_cnt"]
    #[inline(always)]
    pub fn mem_timing_err_cnt(&self) -> MEM_TIMING_ERR_CNT_R {
        MEM_TIMING_ERR_CNT_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bits 22:23 - reg_mem_vt_sel"]
    #[inline(always)]
    pub fn mem_vt_sel(&self) -> MEM_VT_SEL_R {
        MEM_VT_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_mem_path_len"]
    #[inline(always)]
    pub fn mem_path_len(&mut self) -> MEM_PATH_LEN_W {
        MEM_PATH_LEN_W::new(self)
    }
    #[doc = "Bit 4 - reg_mem_err_cnt_clr"]
    #[inline(always)]
    pub fn mem_err_cnt_clr(&mut self) -> MEM_ERR_CNT_CLR_W {
        MEM_ERR_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - reg_mem_pvt_monitor_en"]
    #[inline(always)]
    pub fn monitor_en(&mut self) -> MONITOR_EN_W {
        MONITOR_EN_W::new(self)
    }
    #[doc = "Bits 22:23 - reg_mem_vt_sel"]
    #[inline(always)]
    pub fn mem_vt_sel(&mut self) -> MEM_VT_SEL_W {
        MEM_VT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mem pvt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pvt](index.html) module"]
pub struct MEM_PVT_SPEC;
impl crate::RegisterSpec for MEM_PVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_pvt::R](R) reader structure"]
impl crate::Readable for MEM_PVT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_pvt::W](W) writer structure"]
impl crate::Writable for MEM_PVT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_PVT to value 0x03"]
impl crate::Resettable for MEM_PVT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
