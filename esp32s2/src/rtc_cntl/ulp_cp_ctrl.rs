#[doc = "Register `ULP_CP_CTRL` reader"]
pub struct R(crate::R<ULP_CP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_CTRL` writer"]
pub struct W(crate::W<ULP_CP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_CTRL_SPEC>;
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
impl From<crate::W<ULP_CP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_MEM_ADDR_INIT` reader - "]
pub type ULP_CP_MEM_ADDR_INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ULP_CP_MEM_ADDR_INIT` writer - "]
pub type ULP_CP_MEM_ADDR_INIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, ULP_CP_CTRL_SPEC, 11, O, u16, u16>;
#[doc = "Field `ULP_CP_MEM_ADDR_SIZE` reader - "]
pub type ULP_CP_MEM_ADDR_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ULP_CP_MEM_ADDR_SIZE` writer - "]
pub type ULP_CP_MEM_ADDR_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, ULP_CP_CTRL_SPEC, 11, O, u16, u16>;
#[doc = "Field `ULP_CP_MEM_OFFSET_CLR` writer - "]
pub type ULP_CP_MEM_OFFSET_CLR_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_CTRL_SPEC, O>;
#[doc = "Field `ULP_CP_CLK_FO` reader - ULP-FSM clock force on"]
pub type ULP_CP_CLK_FO_R = crate::BitReader;
#[doc = "Field `ULP_CP_CLK_FO` writer - ULP-FSM clock force on"]
pub type ULP_CP_CLK_FO_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_CTRL_SPEC, O>;
#[doc = "Field `ULP_CP_RESET` reader - ULP-FSM clock software reset"]
pub type ULP_CP_RESET_R = crate::BitReader;
#[doc = "Field `ULP_CP_RESET` writer - ULP-FSM clock software reset"]
pub type ULP_CP_RESET_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_CTRL_SPEC, O>;
#[doc = "Field `ULP_CP_FORCE_START_TOP` reader - Write 1 to start ULP-FSM by software"]
pub type ULP_CP_FORCE_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_FORCE_START_TOP` writer - Write 1 to start ULP-FSM by software"]
pub type ULP_CP_FORCE_START_TOP_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_CTRL_SPEC, O>;
#[doc = "Field `ULP_CP_START_TOP` reader - Write 1 to start ULP-FSM"]
pub type ULP_CP_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_START_TOP` writer - Write 1 to start ULP-FSM"]
pub type ULP_CP_START_TOP_W<'a, const O: u8> = crate::BitWriter<'a, ULP_CP_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_init(&self) -> ULP_CP_MEM_ADDR_INIT_R {
        ULP_CP_MEM_ADDR_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn ulp_cp_mem_addr_size(&self) -> ULP_CP_MEM_ADDR_SIZE_R {
        ULP_CP_MEM_ADDR_SIZE_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - ULP-FSM clock force on"]
    #[inline(always)]
    pub fn ulp_cp_clk_fo(&self) -> ULP_CP_CLK_FO_R {
        ULP_CP_CLK_FO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULP-FSM clock software reset"]
    #[inline(always)]
    pub fn ulp_cp_reset(&self) -> ULP_CP_RESET_R {
        ULP_CP_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write 1 to start ULP-FSM by software"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&self) -> ULP_CP_FORCE_START_TOP_R {
        ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1 to start ULP-FSM"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&self) -> ULP_CP_START_TOP_R {
        ULP_CP_START_TOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_CTRL")
            .field(
                "ulp_cp_mem_addr_init",
                &format_args!("{}", self.ulp_cp_mem_addr_init().bits()),
            )
            .field(
                "ulp_cp_mem_addr_size",
                &format_args!("{}", self.ulp_cp_mem_addr_size().bits()),
            )
            .field(
                "ulp_cp_clk_fo",
                &format_args!("{}", self.ulp_cp_clk_fo().bit()),
            )
            .field(
                "ulp_cp_reset",
                &format_args!("{}", self.ulp_cp_reset().bit()),
            )
            .field(
                "ulp_cp_force_start_top",
                &format_args!("{}", self.ulp_cp_force_start_top().bit()),
            )
            .field(
                "ulp_cp_start_top",
                &format_args!("{}", self.ulp_cp_start_top().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ULP_CP_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_mem_addr_init(&mut self) -> ULP_CP_MEM_ADDR_INIT_W<0> {
        ULP_CP_MEM_ADDR_INIT_W::new(self)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_mem_addr_size(&mut self) -> ULP_CP_MEM_ADDR_SIZE_W<11> {
        ULP_CP_MEM_ADDR_SIZE_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_mem_offset_clr(&mut self) -> ULP_CP_MEM_OFFSET_CLR_W<22> {
        ULP_CP_MEM_OFFSET_CLR_W::new(self)
    }
    #[doc = "Bit 28 - ULP-FSM clock force on"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_clk_fo(&mut self) -> ULP_CP_CLK_FO_W<28> {
        ULP_CP_CLK_FO_W::new(self)
    }
    #[doc = "Bit 29 - ULP-FSM clock software reset"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_reset(&mut self) -> ULP_CP_RESET_W<29> {
        ULP_CP_RESET_W::new(self)
    }
    #[doc = "Bit 30 - Write 1 to start ULP-FSM by software"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_force_start_top(&mut self) -> ULP_CP_FORCE_START_TOP_W<30> {
        ULP_CP_FORCE_START_TOP_W::new(self)
    }
    #[doc = "Bit 31 - Write 1 to start ULP-FSM"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_start_top(&mut self) -> ULP_CP_START_TOP_W<31> {
        ULP_CP_START_TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ULP-FSM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_ctrl](index.html) module"]
pub struct ULP_CP_CTRL_SPEC;
impl crate::RegisterSpec for ULP_CP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_ctrl::R](R) reader structure"]
impl crate::Readable for ULP_CP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_ctrl::W](W) writer structure"]
impl crate::Writable for ULP_CP_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ULP_CP_CTRL to value 0x0010_0200"]
impl crate::Resettable for ULP_CP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0200;
}
