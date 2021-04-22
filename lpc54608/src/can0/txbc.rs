#[doc = "Register `TXBC` reader"]
pub struct R(crate::R<TXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TXBC_SPEC>> for R {
    fn from(reader: crate::R<TXBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBC` writer"]
pub struct W(crate::W<TXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBC_SPEC>;
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
impl core::convert::From<crate::W<TXBC_SPEC>> for W {
    fn from(writer: crate::W<TXBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBSA` reader - Tx buffers start address."]
pub struct TBSA_R(crate::FieldReader<u16, u16>);
impl TBSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TBSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBSA` writer - Tx buffers start address."]
pub struct TBSA_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `NDTB` reader - Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
pub struct NDTB_R(crate::FieldReader<u8, u8>);
impl NDTB_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDTB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDTB` writer - Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
pub struct NDTB_W<'a> {
    w: &'a mut W,
}
impl<'a> NDTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `TFQS` reader - Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
pub struct TFQS_R(crate::FieldReader<u8, u8>);
impl TFQS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TFQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFQS` writer - Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
pub struct TFQS_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `TFQM` reader - Tx FIFO/queue mode."]
pub struct TFQM_R(crate::FieldReader<bool, bool>);
impl TFQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFQM` writer - Tx FIFO/queue mode."]
pub struct TFQM_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Tx buffers start address."]
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Tx FIFO/queue mode."]
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Tx buffers start address."]
    #[inline(always)]
    pub fn tbsa(&mut self) -> TBSA_W {
        TBSA_W { w: self }
    }
    #[doc = "Bits 16:21 - Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[inline(always)]
    pub fn ndtb(&mut self) -> NDTB_W {
        NDTB_W { w: self }
    }
    #[doc = "Bits 24:29 - Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[inline(always)]
    pub fn tfqs(&mut self) -> TFQS_W {
        TFQS_W { w: self }
    }
    #[doc = "Bit 30 - Tx FIFO/queue mode."]
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W {
        TFQM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbc](index.html) module"]
pub struct TXBC_SPEC;
impl crate::RegisterSpec for TXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbc::R](R) reader structure"]
impl crate::Readable for TXBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbc::W](W) writer structure"]
impl crate::Writable for TXBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TXBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
