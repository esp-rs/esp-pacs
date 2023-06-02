#[doc = "Register `PRO_IRAM0_2` reader"]
pub struct R(crate::R<PRO_IRAM0_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_IRAM0_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_IRAM0_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_IRAM0_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_IRAM0_2` writer"]
pub struct W(crate::W<PRO_IRAM0_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_IRAM0_2_SPEC>;
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
impl From<crate::W<PRO_IRAM0_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_IRAM0_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_IRAM0_SRAM_4_SPLTADDR` reader - Configure the split address of SRAM Block 4-21 for IBUS access."]
pub type PRO_IRAM0_SRAM_4_SPLTADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_IRAM0_SRAM_4_SPLTADDR` writer - Configure the split address of SRAM Block 4-21 for IBUS access."]
pub type PRO_IRAM0_SRAM_4_SPLTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_IRAM0_2_SPEC, 17, O, u32, u32>;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_F_W<'a, const O: u8> = crate::BitWriter<'a, PRO_IRAM0_2_SPEC, O>;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_R_W<'a, const O: u8> = crate::BitWriter<'a, PRO_IRAM0_2_SPEC, O>;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_W_W<'a, const O: u8> = crate::BitWriter<'a, PRO_IRAM0_2_SPEC, O>;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_F_W<'a, const O: u8> = crate::BitWriter<'a, PRO_IRAM0_2_SPEC, O>;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_R_W<'a, const O: u8> = crate::BitWriter<'a, PRO_IRAM0_2_SPEC, O>;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_W_W<'a, const O: u8> = crate::BitWriter<'a, PRO_IRAM0_2_SPEC, O>;
impl R {
    #[doc = "Bits 0:16 - Configure the split address of SRAM Block 4-21 for IBUS access."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_spltaddr(&self) -> PRO_IRAM0_SRAM_4_SPLTADDR_R {
        PRO_IRAM0_SRAM_4_SPLTADDR_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_f(&self) -> PRO_IRAM0_SRAM_4_L_F_R {
        PRO_IRAM0_SRAM_4_L_F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_r(&self) -> PRO_IRAM0_SRAM_4_L_R_R {
        PRO_IRAM0_SRAM_4_L_R_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_w(&self) -> PRO_IRAM0_SRAM_4_L_W_R {
        PRO_IRAM0_SRAM_4_L_W_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_f(&self) -> PRO_IRAM0_SRAM_4_H_F_R {
        PRO_IRAM0_SRAM_4_H_F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_r(&self) -> PRO_IRAM0_SRAM_4_H_R_R {
        PRO_IRAM0_SRAM_4_H_R_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_w(&self) -> PRO_IRAM0_SRAM_4_H_W_R {
        PRO_IRAM0_SRAM_4_H_W_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_IRAM0_2")
            .field(
                "pro_iram0_sram_4_spltaddr",
                &format_args!("{}", self.pro_iram0_sram_4_spltaddr().bits()),
            )
            .field(
                "pro_iram0_sram_4_l_f",
                &format_args!("{}", self.pro_iram0_sram_4_l_f().bit()),
            )
            .field(
                "pro_iram0_sram_4_l_r",
                &format_args!("{}", self.pro_iram0_sram_4_l_r().bit()),
            )
            .field(
                "pro_iram0_sram_4_l_w",
                &format_args!("{}", self.pro_iram0_sram_4_l_w().bit()),
            )
            .field(
                "pro_iram0_sram_4_h_f",
                &format_args!("{}", self.pro_iram0_sram_4_h_f().bit()),
            )
            .field(
                "pro_iram0_sram_4_h_r",
                &format_args!("{}", self.pro_iram0_sram_4_h_r().bit()),
            )
            .field(
                "pro_iram0_sram_4_h_w",
                &format_args!("{}", self.pro_iram0_sram_4_h_w().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_IRAM0_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:16 - Configure the split address of SRAM Block 4-21 for IBUS access."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_sram_4_spltaddr(&mut self) -> PRO_IRAM0_SRAM_4_SPLTADDR_W<0> {
        PRO_IRAM0_SRAM_4_SPLTADDR_W::new(self)
    }
    #[doc = "Bit 17 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_sram_4_l_f(&mut self) -> PRO_IRAM0_SRAM_4_L_F_W<17> {
        PRO_IRAM0_SRAM_4_L_F_W::new(self)
    }
    #[doc = "Bit 18 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_sram_4_l_r(&mut self) -> PRO_IRAM0_SRAM_4_L_R_W<18> {
        PRO_IRAM0_SRAM_4_L_R_W::new(self)
    }
    #[doc = "Bit 19 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_sram_4_l_w(&mut self) -> PRO_IRAM0_SRAM_4_L_W_W<19> {
        PRO_IRAM0_SRAM_4_L_W_W::new(self)
    }
    #[doc = "Bit 20 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_sram_4_h_f(&mut self) -> PRO_IRAM0_SRAM_4_H_F_W<20> {
        PRO_IRAM0_SRAM_4_H_F_W::new(self)
    }
    #[doc = "Bit 21 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_sram_4_h_r(&mut self) -> PRO_IRAM0_SRAM_4_H_R_W<21> {
        PRO_IRAM0_SRAM_4_H_R_W::new(self)
    }
    #[doc = "Bit 22 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_iram0_sram_4_h_w(&mut self) -> PRO_IRAM0_SRAM_4_H_W_W<22> {
        PRO_IRAM0_SRAM_4_H_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IBUS permission control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_iram0_2](index.html) module"]
pub struct PRO_IRAM0_2_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_iram0_2::R](R) reader structure"]
impl crate::Readable for PRO_IRAM0_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_iram0_2::W](W) writer structure"]
impl crate::Writable for PRO_IRAM0_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_IRAM0_2 to value 0x007e_0000"]
impl crate::Resettable for PRO_IRAM0_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x007e_0000;
}
